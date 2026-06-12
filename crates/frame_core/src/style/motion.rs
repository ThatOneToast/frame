//! Semantic motion.
//!
//! A `motion` declaration names interaction intent — enter animation, state
//! effects, and transition feel — and is applied with a `motion Name`
//! statement inside any styled declaration or recipe. Motions lower to the
//! built-in keyframes, transforms, and transition declarations; they never
//! add runtime work.
//!
//! ```frame
//! motion Pressable {
//!   enter fade up soft
//!   hover lift sm
//!   active press
//!   focus ring accent
//!   duration normal
//!   easing smooth
//! }
//! ```

use crate::{DeclarationKind, Document, Node};

use super::normalize::{normalize_effect, statements};
use super::schema::{merge_facts, NormalizedStyle, StateScope, StyleFact};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Motion {
    pub name: String,
    /// Facts applied to the base rule (enter animation + transition feel).
    pub facts: Vec<StyleFact>,
    /// Facts applied to interaction states (hover, active, focus, ...).
    pub states: Vec<StateScope>,
}

/// Map an `enter` intent to one of the built-in keyframes.
///
/// `enter <fade|pop|slide> [up|down] [soft|normal|brisk]`
fn enter_fact(words: &[String]) -> Option<StyleFact> {
    let mut keyframes = None;
    let mut duration = "240ms";
    for word in words {
        match word.as_str() {
            "fade" => keyframes = Some(keyframes.unwrap_or("frame-fade-in")),
            "pop" => keyframes = Some("frame-pop-in"),
            "slide" | "up" => keyframes = Some("frame-slide-up"),
            "down" => keyframes = Some("frame-slide-up"),
            "soft" => duration = "320ms",
            "normal" => duration = "240ms",
            "brisk" => duration = "160ms",
            _ => {}
        }
    }
    let keyframes = keyframes?;
    Some(StyleFact::single(
        "motion.animation",
        "animation",
        format!("{keyframes} {duration} ease both"),
    ))
}

fn transition_fact(duration: &str, easing: &str) -> StyleFact {
    let duration = match duration {
        "fast" => "120ms",
        "slow" => "360ms",
        _ => "200ms",
    };
    let easing = match easing {
        "linear" => "linear",
        "bounce" => "cubic-bezier(.2, 1.4, .4, 1)",
        "sharp" => "cubic-bezier(.4, 0, 1, 1)",
        _ => "ease",
    };
    StyleFact::single(
        "motion.transition",
        "transition",
        format!("all {duration} {easing}"),
    )
}

const MOTION_STATES: &[&str] = &["hover", "active", "focus", "focus-within", "disabled"];

/// Lower a motion declaration body.
pub fn lower_motion(name: &str, body: &[Node]) -> Motion {
    let mut motion = Motion {
        name: name.to_string(),
        ..Motion::default()
    };
    let mut duration = "normal".to_string();
    let mut easing = "smooth".to_string();
    let mut has_feel = false;
    let mut has_states = false;

    for statement in statements(body) {
        let Some(keyword) = statement.words.first().map(String::as_str) else {
            continue;
        };
        match keyword {
            "enter" => {
                if let Some(fact) = enter_fact(&statement.words[1..]) {
                    motion.facts.push(fact);
                }
            }
            "duration" => {
                if let Some(value) = statement.words.get(1) {
                    duration = value.clone();
                    has_feel = true;
                }
            }
            "easing" | "ease" => {
                if let Some(value) = statement.words.get(1) {
                    easing = value.clone();
                    has_feel = true;
                }
            }
            state if MOTION_STATES.contains(&state) => {
                has_states = true;
                let effect = crate::Statement {
                    words: statement.words[1..].to_vec(),
                    span: statement.span,
                };
                if let Some(fact) = normalize_effect(&effect) {
                    if let Some(scope) = motion.states.iter_mut().find(|scope| scope.state == state)
                    {
                        scope.facts.push(fact);
                    } else {
                        motion.states.push(StateScope {
                            state: state.to_string(),
                            facts: vec![fact],
                        });
                    }
                }
            }
            _ => {}
        }
    }

    // State effects need a transition to feel like motion; emit one whenever
    // the motion declares states or an explicit feel.
    if has_feel || has_states {
        motion.facts.push(transition_fact(&duration, &easing));
    }

    motion
}

/// Collect every motion declaration in a document.
pub fn document_motions(document: &Document) -> Vec<Motion> {
    document
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Motion)
        .map(|declaration| lower_motion(&declaration.name.text, &declaration.body))
        .collect()
}

/// Expand `motion Name` reference facts inside a resolved style.
///
/// The reference fact is replaced in place by the motion's base facts; the
/// motion's state facts merge under the style's own state scopes, with the
/// style's explicit state effects winning by property path.
pub fn expand_motion_references(style: &mut NormalizedStyle, motions: &[Motion]) {
    let mut expanded = Vec::with_capacity(style.facts.len());
    let mut applied: Vec<&Motion> = Vec::new();

    for fact in style.facts.drain(..) {
        if fact.path == "motion.reference" {
            let Some(name) = fact
                .decls
                .first()
                .map(|decl| decl.value.clone())
                .filter(|_| true)
            else {
                continue;
            };
            if let Some(motion) = motions.iter().find(|motion| motion.name == name) {
                expanded = merge_facts(expanded, motion.facts.clone());
                applied.push(motion);
            }
            continue;
        }
        expanded.push(fact);
    }
    style.facts = expanded;

    for motion in applied {
        for motion_scope in &motion.states {
            if let Some(own) = style
                .states
                .iter_mut()
                .find(|scope| scope.state == motion_scope.state)
            {
                own.facts = merge_facts(motion_scope.facts.clone(), std::mem::take(&mut own.facts));
            } else {
                style.states.push(motion_scope.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Span, Statement};

    fn statement(words: &[&str]) -> Node {
        Node::Statement(Statement {
            words: words.iter().map(|word| word.to_string()).collect(),
            span: Span::default(),
        })
    }

    #[test]
    fn lowers_enter_and_states() {
        let motion = lower_motion(
            "Pressable",
            &[
                statement(&["enter", "fade", "up", "soft"]),
                statement(&["hover", "lift", "sm"]),
                statement(&["active", "press"]),
                statement(&["focus", "ring", "accent"]),
                statement(&["duration", "normal"]),
                statement(&["easing", "smooth"]),
            ],
        );

        assert_eq!(motion.facts.len(), 2);
        assert_eq!(motion.facts[0].decls[0].property, "animation");
        assert!(motion.facts[0].decls[0].value.contains("frame-slide-up"));
        assert!(motion.facts[0].decls[0].value.contains("320ms"));
        assert_eq!(motion.facts[1].decls[0].property, "transition");
        assert_eq!(motion.states.len(), 3);
        assert_eq!(motion.states[0].state, "hover");
    }

    #[test]
    fn expands_references_with_own_states_winning() {
        let motion = lower_motion("Pressable", &[statement(&["hover", "lift", "sm"])]);
        let mut style = NormalizedStyle {
            facts: vec![StyleFact::single(
                "motion.reference",
                "@motion",
                "Pressable",
            )],
            states: vec![StateScope {
                state: "hover".to_string(),
                facts: vec![StyleFact::single(
                    "effect.lift",
                    "@transform-part",
                    "translateY(-12px)",
                )],
            }],
            ..NormalizedStyle::default()
        };

        expand_motion_references(&mut style, &[motion]);

        // Reference replaced by transition fact.
        assert!(style
            .facts
            .iter()
            .any(|fact| fact.path == "motion.transition"));
        assert!(!style
            .facts
            .iter()
            .any(|fact| fact.path == "motion.reference"));
        // Own hover lift overrides the motion's lift by path.
        let hover = &style.states[0];
        assert_eq!(hover.facts.len(), 1);
        assert_eq!(hover.facts[0].decls[0].value, "translateY(-12px)");
    }
}
