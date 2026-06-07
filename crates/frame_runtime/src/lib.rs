//! Renderer-neutral runtime model for Frame IR.
//!
//! This crate intentionally contains no DOM, browser, WebView, or native toolkit
//! references. Renderers consume these contracts and provide concrete handles.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeNodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StateKey(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeValue {
    Text(String),
    Bool(bool),
    Number(String),
    List(Vec<RuntimeValue>),
    Record(BTreeMap<String, RuntimeValue>),
    Null,
}

pub trait Renderer {
    type Node: RenderNode;
    type Element: RenderElement;
    type Text: RenderText;
    type Event: RenderEvent;

    fn capabilities(&self) -> RendererCapabilities;
}

pub trait RenderNode {
    fn id(&self) -> RuntimeNodeId;
}

pub trait RenderElement: RenderNode {
    fn element_kind(&self) -> &str;
}

pub trait RenderText: RenderNode {
    fn text_value(&self) -> &str;
}

pub trait RenderEvent {
    fn descriptor(&self) -> &EventDescriptor;
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RendererCapabilities {
    pub elements: BTreeSet<String>,
    pub events: BTreeSet<String>,
    pub text: bool,
    pub keyed_lists: bool,
    pub slots: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRuntime {
    pub id: ComponentId,
    pub name: String,
    pub props: PropStore,
    pub state: StateStore,
    pub handlers: HandlerRegistry,
    pub slots: SlotRegistry,
    pub lifecycle: LifecycleMetadata,
}

impl ComponentRuntime {
    pub fn new(id: ComponentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            props: PropStore::default(),
            state: StateStore::default(),
            handlers: HandlerRegistry::default(),
            slots: SlotRegistry::default(),
            lifecycle: LifecycleMetadata::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LifecycleMetadata {
    pub mount_requested: bool,
    pub update_requested: bool,
    pub disposed: bool,
    pub generation: u64,
}

impl LifecycleMetadata {
    pub fn request_mount(&mut self) {
        self.mount_requested = true;
    }

    pub fn request_update(&mut self) {
        self.update_requested = true;
        self.generation += 1;
    }

    pub fn dispose(&mut self) {
        self.disposed = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropDescriptor {
    pub name: String,
    pub value_type: ValueType,
    pub readonly: bool,
    pub binding: BindingMetadata,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingMetadata {
    Input,
    TwoWayAllowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    Text,
    Bool,
    Number,
    List,
    Record,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PropStore {
    descriptors: BTreeMap<String, PropDescriptor>,
    values: BTreeMap<String, RuntimeValue>,
}

impl PropStore {
    pub fn define(&mut self, descriptor: PropDescriptor) {
        self.descriptors.insert(descriptor.name.clone(), descriptor);
    }

    pub fn set(&mut self, name: &str, value: RuntimeValue) -> Result<(), RuntimeError> {
        let Some(descriptor) = self.descriptors.get(name) else {
            return Err(RuntimeError::UnknownProp(name.to_string()));
        };
        if !value_matches(&value, &descriptor.value_type) {
            return Err(RuntimeError::InvalidPropType(name.to_string()));
        }
        self.values.insert(name.to_string(), value);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        self.values.get(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDescriptor {
    pub key: StateKey,
    pub name: String,
    pub value_type: ValueType,
    pub initial: RuntimeValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StateStore {
    descriptors: BTreeMap<StateKey, StateDescriptor>,
    names: BTreeMap<String, StateKey>,
    values: BTreeMap<StateKey, RuntimeValue>,
    subscribers: BTreeMap<StateKey, BTreeSet<SubscriptionId>>,
    dependencies: BTreeMap<SubscriptionId, BTreeSet<StateKey>>,
    dirty: BTreeSet<StateKey>,
    current_tracker: Option<SubscriptionId>,
    batch_depth: u32,
}

impl StateStore {
    pub fn define(&mut self, descriptor: StateDescriptor) {
        self.names.insert(descriptor.name.clone(), descriptor.key);
        self.values
            .insert(descriptor.key, descriptor.initial.clone());
        self.descriptors.insert(descriptor.key, descriptor);
    }

    pub fn read(&mut self, name: &str) -> Option<&RuntimeValue> {
        let key = *self.names.get(name)?;
        if let Some(tracker) = self.current_tracker {
            self.dependencies.entry(tracker).or_default().insert(key);
            self.subscribers.entry(key).or_default().insert(tracker);
        }
        self.values.get(&key)
    }

    pub fn write(&mut self, name: &str, value: RuntimeValue) -> Result<(), RuntimeError> {
        let key = *self
            .names
            .get(name)
            .ok_or_else(|| RuntimeError::UnknownState(name.to_string()))?;
        let descriptor = self
            .descriptors
            .get(&key)
            .ok_or_else(|| RuntimeError::UnknownState(name.to_string()))?;
        if !value_matches(&value, &descriptor.value_type) {
            return Err(RuntimeError::InvalidStateType(name.to_string()));
        }
        if self.values.get(&key) != Some(&value) {
            self.values.insert(key, value);
            self.dirty.insert(key);
        }
        Ok(())
    }

    pub fn track_reads<T>(
        &mut self,
        subscription: SubscriptionId,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.dependencies.remove(&subscription);
        self.current_tracker = Some(subscription);
        let value = f(self);
        self.current_tracker = None;
        value
    }

    pub fn begin_batch(&mut self) {
        self.batch_depth += 1;
    }

    pub fn end_batch(&mut self) -> BTreeSet<SubscriptionId> {
        self.batch_depth = self.batch_depth.saturating_sub(1);
        if self.batch_depth == 0 {
            self.flush_dirty()
        } else {
            BTreeSet::new()
        }
    }

    pub fn flush_dirty(&mut self) -> BTreeSet<SubscriptionId> {
        let mut affected = BTreeSet::new();
        for key in std::mem::take(&mut self.dirty) {
            if let Some(subscribers) = self.subscribers.get(&key) {
                affected.extend(subscribers.iter().copied());
            }
        }
        affected
    }

    pub fn dirty_keys(&self) -> &BTreeSet<StateKey> {
        &self.dirty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandlerRegistry {
    handlers: BTreeMap<String, HandlerDescriptor>,
}

impl HandlerRegistry {
    pub fn register(&mut self, descriptor: HandlerDescriptor) {
        self.handlers.insert(descriptor.name.clone(), descriptor);
    }

    pub fn lookup(&self, name: &str) -> Option<&HandlerDescriptor> {
        self.handlers.get(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerDescriptor {
    pub name: String,
    pub accepts: Vec<EventDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDescriptor {
    pub event: String,
    pub modifiers: Vec<EventModifier>,
    pub handler: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventModifier {
    Key(String),
    Control(String),
    Pointer(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchMetadata {
    pub component: ComponentId,
    pub target: RuntimeNodeId,
    pub descriptor: EventDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SlotRegistry {
    slots: BTreeMap<String, SlotDescriptor>,
}

impl SlotRegistry {
    pub fn define(&mut self, descriptor: SlotDescriptor) {
        self.slots.insert(descriptor.name.clone(), descriptor);
    }

    pub fn lookup(&self, name: &str) -> Option<&SlotDescriptor> {
        self.slots.get(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotDescriptor {
    pub name: String,
    pub has_fallback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionDescriptor {
    pub kind: ConditionKind,
    pub dependency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionKind {
    Show,
    Hidden,
    Property(String),
    Style(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListDescriptor {
    pub item: String,
    pub collection: String,
    pub key: Option<String>,
    pub behavior: ListUpdateBehavior,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListUpdateBehavior {
    KeyedReuse,
    Positional,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeError {
    UnknownProp(String),
    InvalidPropType(String),
    UnknownState(String),
    InvalidStateType(String),
}

fn value_matches(value: &RuntimeValue, value_type: &ValueType) -> bool {
    matches!(
        (value, value_type),
        (RuntimeValue::Text(_), ValueType::Text)
            | (RuntimeValue::Bool(_), ValueType::Bool)
            | (RuntimeValue::Number(_), ValueType::Number)
            | (RuntimeValue::List(_), ValueType::List)
            | (RuntimeValue::Record(_), ValueType::Record)
            | (_, ValueType::Unknown(_))
            | (RuntimeValue::Null, _)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn props_validate_type_and_lookup_values() {
        let mut props = PropStore::default();
        props.define(PropDescriptor {
            name: "title".to_string(),
            value_type: ValueType::Text,
            readonly: true,
            binding: BindingMetadata::Input,
            required: true,
        });

        props
            .set("title", RuntimeValue::Text("Inbox".to_string()))
            .expect("valid prop");

        assert_eq!(
            props.get("title"),
            Some(&RuntimeValue::Text("Inbox".to_string()))
        );
        assert_eq!(
            props.set("title", RuntimeValue::Bool(true)),
            Err(RuntimeError::InvalidPropType("title".to_string()))
        );
    }

    #[test]
    fn state_tracks_reads_writes_dirty_keys_and_batches() {
        let mut state = StateStore::default();
        state.define(StateDescriptor {
            key: StateKey(1),
            name: "count".to_string(),
            value_type: ValueType::Number,
            initial: RuntimeValue::Number("0".to_string()),
        });

        state.track_reads(SubscriptionId(7), |state| {
            assert_eq!(
                state.read("count"),
                Some(&RuntimeValue::Number("0".to_string()))
            );
        });
        state.begin_batch();
        state
            .write("count", RuntimeValue::Number("1".to_string()))
            .expect("valid state write");
        assert!(state.dirty_keys().contains(&StateKey(1)));

        let affected = state.end_batch();
        assert!(affected.contains(&SubscriptionId(7)));
        assert!(state.dirty_keys().is_empty());
    }

    #[test]
    fn component_lifecycle_records_mount_update_and_disposal_requests() {
        let mut component = ComponentRuntime::new(ComponentId(1), "Counter");
        component.lifecycle.request_mount();
        component.lifecycle.request_update();
        component.lifecycle.dispose();

        assert!(component.lifecycle.mount_requested);
        assert!(component.lifecycle.update_requested);
        assert!(component.lifecycle.disposed);
        assert_eq!(component.lifecycle.generation, 1);
    }

    #[test]
    fn slots_events_conditions_and_lists_are_metadata_only() {
        let mut slots = SlotRegistry::default();
        slots.define(SlotDescriptor {
            name: "Header".to_string(),
            has_fallback: true,
        });

        let event = EventDescriptor {
            event: "keydown".to_string(),
            modifiers: vec![EventModifier::Key("enter".to_string())],
            handler: "send".to_string(),
        };
        let condition = ConditionDescriptor {
            kind: ConditionKind::Show,
            dependency: "loggedIn".to_string(),
        };
        let list = ListDescriptor {
            item: "message".to_string(),
            collection: "messages".to_string(),
            key: Some("messageId".to_string()),
            behavior: ListUpdateBehavior::KeyedReuse,
        };

        assert_eq!(
            slots.lookup("Header").map(|slot| slot.has_fallback),
            Some(true)
        );
        assert_eq!(
            event.modifiers,
            vec![EventModifier::Key("enter".to_string())]
        );
        assert_eq!(condition.kind, ConditionKind::Show);
        assert_eq!(list.behavior, ListUpdateBehavior::KeyedReuse);
    }
}
