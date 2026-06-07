use std::{fs, path::PathBuf, process::Command};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn fixture_path(name: &str) -> PathBuf {
    fixtures_dir().join(name)
}

fn run_emit_ir(fixture: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("emit-ir")
        .arg(fixture_path(fixture))
        .output()
        .expect("frame emit-ir should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_emit_contracts(fixture: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("emit-contracts")
        .arg(fixture_path(fixture))
        .output()
        .expect("frame emit-contracts should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_check(fixture: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_frame"))
        .arg("check")
        .arg(fixture_path(fixture))
        .output()
        .expect("frame check should run")
}

fn parse_ir_json(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("IR JSON should be valid")
}

fn assert_version_and_components(ir: &serde_json::Value) {
    assert_eq!(ir.get("version").and_then(|v| v.as_u64()), Some(1));
    assert!(ir.get("components").is_some(), "IR should have components");
}

// Helpers to unwrap enum-wrapped IR nodes
fn as_element(node: &serde_json::Value) -> &serde_json::Value {
    node.get("Element").expect("expected Element node")
}

fn as_text(node: &serde_json::Value) -> &serde_json::Value {
    node.get("Text").expect("expected Text node")
}

fn as_component(node: &serde_json::Value) -> &serde_json::Value {
    node.get("Component").expect("expected Component node")
}

fn as_list(node: &serde_json::Value) -> &serde_json::Value {
    node.get("List").expect("expected List node")
}

#[test]
fn golden_simple_component_ir() {
    let json = run_emit_ir("simple-component.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    assert_eq!(components.len(), 1);
    assert_eq!(components[0]["name"], "SimpleApp");

    let nodes = components[0]["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    let screen = as_element(&nodes[0]);
    assert_eq!(screen["kind"], "screen");
    assert_eq!(screen["name"], "Main");

    let children = screen["children"].as_array().unwrap();
    assert_eq!(children.len(), 1);
    let text = as_text(&children[0]);
    assert_eq!(text["value"]["Literal"], "Hello World");
}

#[test]
fn golden_simple_component_contracts() {
    let ts = run_emit_contracts("simple-component.frame");
    assert!(ts.contains("export type SimpleAppState = {\n};\n"));
    assert!(ts.contains("export type SimpleAppHandlers = {\n};\n"));
}

#[test]
fn golden_props_state_ir() {
    let json = run_emit_ir("props-state.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    assert_eq!(components.len(), 1);
    let component = &components[0];
    assert_eq!(component["name"], "PropsStateApp");

    let props = component["props"].as_array().unwrap();
    assert_eq!(props.len(), 2);
    assert_eq!(props[0]["name"], "title");
    assert_eq!(props[0]["value_type"], "Text");
    assert_eq!(props[1]["name"], "count");
    assert_eq!(props[1]["value_type"], "Number");

    let state = component["state"].as_array().unwrap();
    assert_eq!(state.len(), 3);
    assert_eq!(state[0]["name"], "message");
    assert_eq!(state[0]["value_type"], "Text");
    assert_eq!(state[0]["default"]["Text"], "Hello");
    assert_eq!(state[1]["name"], "active");
    assert_eq!(state[1]["value_type"], "Bool");
    assert_eq!(state[1]["default"]["Bool"], false);
    assert_eq!(state[2]["name"], "items");
    assert_eq!(state[2]["value_type"], "List");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("ComponentComposition")));
}

#[test]
fn golden_props_state_contracts() {
    let ts = run_emit_contracts("props-state.frame");
    assert!(
        ts.contains("export type PropsStateAppProps = {\n  title: string;\n  count: number;\n};\n")
    );
    assert!(ts.contains("export type PropsStateAppState = {\n  message: string;\n  active: boolean;\n  items: unknown[];\n};\n"));
}

#[test]
fn golden_semantic_primitives_ir() {
    let json = run_emit_ir("semantic-primitives.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "SemanticApp");

    let nodes = component["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    let screen = as_element(&nodes[0]);
    let children = screen["children"].as_array().unwrap();
    assert_eq!(children.len(), 6);

    // action SearchButton
    let action = as_element(&children[0]);
    assert_eq!(action["kind"], "action");
    assert_eq!(action["name"], "SearchButton");
    assert_eq!(action["render_kind"], "button");
    assert_eq!(action["events"][0]["event"], "click");
    assert_eq!(action["events"][0]["handler"], "performSearch");

    // field QueryField
    let field = as_element(&children[1]);
    assert_eq!(field["kind"], "field");
    assert_eq!(field["name"], "QueryField");
    assert_eq!(field["render_kind"], "div");

    // input SearchInput inside field
    let input = as_element(&field["children"][0]);
    assert_eq!(input["kind"], "input");
    assert_eq!(input["name"], "SearchInput");
    assert_eq!(input["render_kind"], "input");
    assert_eq!(input["attributes"][0]["name"], "value");
    assert_eq!(input["attributes"][0]["value"]["DataRef"], "query");

    // editor NotesEditor
    let editor = as_element(&children[2]);
    assert_eq!(editor["kind"], "editor");
    assert_eq!(editor["name"], "NotesEditor");
    assert_eq!(editor["render_kind"], "textarea");
    assert_eq!(editor["bindings"][0]["property"], "value");
    assert_eq!(editor["bindings"][0]["state"], "query");

    // toggle FeatureToggle
    let toggle = as_element(&children[3]);
    assert_eq!(toggle["kind"], "toggle");
    assert_eq!(toggle["name"], "FeatureToggle");
    assert_eq!(toggle["render_kind"], "input");
    let toggle_conditions = toggle["conditions"].as_array().unwrap();
    assert_eq!(toggle_conditions[0]["Property"]["property"], "checked");
    assert_eq!(toggle_conditions[0]["Property"]["state"], "enabled");

    // title "Settings" shorthand
    let title = as_element(&children[4]);
    assert_eq!(title["kind"], "title");
    assert_eq!(title["name"], "Title");
    assert_eq!(title["render_kind"], "h2");
    assert_eq!(title["attributes"][0]["name"], "value");
    assert_eq!(title["attributes"][0]["value"]["Literal"], "Settings");

    // label "Active" shorthand
    let label = as_element(&children[5]);
    assert_eq!(label["kind"], "label");
    assert_eq!(label["name"], "Label");
    assert_eq!(label["render_kind"], "span");
    assert_eq!(label["attributes"][0]["value"]["Literal"], "Active");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("EventBinding")));
    assert!(capabilities.contains(&serde_json::json!("TwoWayBinding")));
    assert!(capabilities.contains(&serde_json::json!("ConditionalRendering")));
}

#[test]
fn golden_semantic_primitives_contracts() {
    let ts = run_emit_contracts("semantic-primitives.frame");
    assert!(ts
        .contains("export type SemanticAppState = {\n  query: string;\n  enabled: boolean;\n};\n"));
    assert!(ts.contains("performSearch(ctx"));
    assert!(ts.contains("export type SemanticAppHandlers = {\n"));
}

#[test]
fn golden_style_bindings_ir() {
    let json = run_emit_ir("style-bindings.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "StyleApp");

    let nodes = component["nodes"].as_array().unwrap();
    let screen = as_element(&nodes[0]);
    let children = screen["children"].as_array().unwrap();
    assert_eq!(children.len(), 3);

    // automatic style lookup
    let auto = as_element(&children[0]);
    assert_eq!(auto["name"], "PrimaryButton");
    assert!(auto["style"].get("Automatic").is_some());
    assert_eq!(auto["style"]["Automatic"]["style"], "PrimaryButton");

    // explicit style mapping
    let explicit = as_element(&children[1]);
    assert_eq!(explicit["name"], "Send");
    assert!(explicit["style"].get("Explicit").is_some());
    assert_eq!(explicit["style"]["Explicit"]["style"], "PrimaryButton");

    // conditional style alias
    let conditional = as_element(&children[2]);
    assert_eq!(conditional["name"], "Submit");
    assert!(conditional["style"].get("Explicit").is_some());
    assert_eq!(conditional["style"]["Explicit"]["style"], "PrimaryButton");
    let conditions = conditional["conditions"].as_array().unwrap();
    assert_eq!(conditions[0]["Style"]["state"], "loading");
    assert_eq!(conditions[0]["Style"]["style"], "LoadingButton");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("ConditionalStyles")));
}

#[test]
fn golden_style_bindings_contracts() {
    let ts = run_emit_contracts("style-bindings.frame");
    assert!(ts.contains("export type StyleAppState = {\n  loading: boolean;\n};\n"));
}

#[test]
fn golden_events_ir() {
    let json = run_emit_ir("events.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "EventApp");

    let screen = as_element(&component["nodes"][0]);
    let children = screen["children"].as_array().unwrap();

    let action = as_element(&children[0]);
    assert_eq!(action["name"], "Send");
    let events = action["events"].as_array().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["event"], "click");
    assert_eq!(events[0]["handler"], "sendMessage");
    assert_eq!(events[0]["modifiers"].as_array().unwrap().len(), 0);
    assert_eq!(events[1]["event"], "keydown");
    assert_eq!(events[1]["handler"], "sendMessage");
    assert_eq!(events[1]["modifiers"].as_array().unwrap(), &["enter"]);

    let input = as_element(&children[1]);
    assert_eq!(input["name"], "SearchBox");
    let input_events = input["events"].as_array().unwrap();
    assert_eq!(input_events.len(), 2);
    assert_eq!(input_events[0]["event"], "input");
    assert_eq!(input_events[0]["handler"], "updateQuery");
    assert_eq!(input_events[1]["event"], "focus");
    assert_eq!(input_events[1]["handler"], "trackFocus");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("EventBinding")));
    assert!(capabilities.contains(&serde_json::json!("TwoWayBinding")));
}

#[test]
fn golden_events_contracts() {
    let ts = run_emit_contracts("events.frame");
    assert_eq!(ts.matches("sendMessage(ctx").count(), 1);
    assert!(ts.contains("updateQuery(ctx"));
    assert!(ts.contains("trackFocus(ctx"));
}

#[test]
fn golden_conditional_ir() {
    let json = run_emit_ir("conditional.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "ConditionalApp");

    let screen = as_element(&component["nodes"][0]);
    let children = screen["children"].as_array().unwrap();

    let info_panel = as_element(&children[0]);
    assert_eq!(info_panel["name"], "InfoPanel");
    let conditions = info_panel["conditions"].as_array().unwrap();
    assert_eq!(conditions[0]["Show"]["state"], "visible");

    let admin_panel = as_element(&children[1]);
    assert_eq!(admin_panel["name"], "AdminPanel");
    let admin_conditions = admin_panel["conditions"].as_array().unwrap();
    assert_eq!(admin_conditions[0]["Hidden"]["state"], "loggedIn");

    let login_button = as_element(&children[2]);
    assert_eq!(login_button["name"], "LoginButton");
    let btn_conditions = login_button["conditions"].as_array().unwrap();
    assert_eq!(btn_conditions[0]["Property"]["property"], "disabled");
    assert_eq!(btn_conditions[0]["Property"]["state"], "loggedIn");

    let admin_text = as_text(&children[3]);
    assert_eq!(admin_text["value"]["Literal"], "Admin view");

    let screen_conditions = screen["conditions"].as_array().unwrap();
    assert_eq!(screen_conditions[0]["Show"]["state"], "admin");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("ConditionalRendering")));
}

#[test]
fn golden_conditional_contracts() {
    let ts = run_emit_contracts("conditional.frame");
    assert!(ts.contains("export type ConditionalAppState = {\n  visible: boolean;\n  loggedIn: boolean;\n  admin: boolean;\n};\n"));
    assert!(ts.contains("export type ConditionalAppHandlers = {\n};\n"));
}

#[test]
fn golden_keyed_list_ir() {
    let json = run_emit_ir("keyed-list.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "KeyedListApp");

    let screen = as_element(&component["nodes"][0]);
    let list = as_element(&screen["children"][0]);
    assert_eq!(list["kind"], "list");

    let list_node = as_list(&list["children"][0]);
    assert_eq!(list_node["item"], "item");
    assert_eq!(list_node["collection"], "items");
    assert_eq!(list_node["key"], "selectedId");
    let list_children = list_node["children"].as_array().unwrap();
    assert_eq!(list_children.len(), 1);
    let text = as_text(&list_children[0]);
    assert_eq!(text["value"]["DataRef"], "item");

    let capabilities = component["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("ListRendering")));
}

#[test]
fn golden_keyed_list_contracts() {
    let ts = run_emit_contracts("keyed-list.frame");
    assert!(ts.contains(
        "export type KeyedListAppState = {\n  items: unknown[];\n  selectedId: string;\n};\n"
    ));
}

#[test]
fn golden_nested_components_ir() {
    let json = run_emit_ir("nested-components.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    assert_eq!(components.len(), 2);

    let child = &components[0];
    assert_eq!(child["name"], "ChildItem");
    let child_props = child["props"].as_array().unwrap();
    assert_eq!(child_props.len(), 1);
    assert_eq!(child_props[0]["name"], "label");
    assert_eq!(child_props[0]["value_type"], "Text");

    let parent = &components[1];
    assert_eq!(parent["name"], "NestedApp");
    let slots = parent["slots"].as_array().unwrap();
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0]["name"], "Header");
    let fallback = slots[0]["fallback"].as_array().unwrap();
    assert_eq!(fallback.len(), 1);
    let fallback_text = as_text(&fallback[0]);
    assert_eq!(fallback_text["value"]["Literal"], "Default header");

    let screen = as_element(&parent["nodes"][0]);
    let stack = as_element(&screen["children"][0]);
    assert_eq!(stack["kind"], "stack");
    let invocations = stack["children"].as_array().unwrap();
    assert_eq!(invocations.len(), 2);
    let first = as_component(&invocations[0]);
    assert_eq!(first["name"], "ChildItem");
    assert_eq!(first["arguments"][0]["name"], "label");
    assert_eq!(first["arguments"][0]["value"]["Literal"], "First");
    let second = as_component(&invocations[1]);
    assert_eq!(second["arguments"][0]["value"]["Literal"], "Second");

    let capabilities = parent["capabilities"].as_array().unwrap();
    assert!(capabilities.contains(&serde_json::json!("ComponentComposition")));
    assert!(capabilities.contains(&serde_json::json!("SlotContent")));
}

#[test]
fn golden_nested_components_contracts() {
    let ts = run_emit_contracts("nested-components.frame");
    assert!(ts.contains("export type ChildItemProps = {\n  label: string;\n};\n"));
    assert!(ts.contains("export type ChildItemState = {\n};\n"));
    assert!(ts.contains("export type NestedAppState = {\n  query: string;\n};\n"));
}

#[test]
fn golden_media_url_check_warning() {
    let output = run_check("media-url.frame");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("navigation or media destination"));
}

#[test]
fn golden_media_url_ir() {
    let json = run_emit_ir("media-url.frame");
    let ir = parse_ir_json(&json);
    assert_version_and_components(&ir);

    let components = ir["components"].as_array().unwrap();
    let component = &components[0];
    assert_eq!(component["name"], "MediaApp");

    let screen = as_element(&component["nodes"][0]);
    let media = as_element(&screen["children"][0]);
    assert_eq!(media["kind"], "media");
    assert_eq!(media["name"], "Preview");
    assert_eq!(media["render_kind"], "media");
    assert_eq!(media["attributes"][0]["name"], "source");
    assert_eq!(media["attributes"][0]["value"]["Literal"], "/video.mp4");
}

#[test]
fn golden_media_url_contracts() {
    let ts = run_emit_contracts("media-url.frame");
    assert!(ts.contains("export type MediaAppState = {\n};\n"));
    assert!(ts.contains("export type MediaAppHandlers = {\n};\n"));
}

// Structural snapshot test: verify all fixtures produce valid IR and contracts
#[test]
fn all_fixtures_emit_valid_ir() {
    let dir = fixtures_dir();
    let mut entries: Vec<_> = fs::read_dir(&dir)
        .expect("fixtures dir should be readable")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "frame")
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .collect();
    entries.sort();

    assert!(
        entries.len() >= 8,
        "expected at least 8 fixture files, found {}",
        entries.len()
    );

    for path in &entries {
        let fixture = path.file_name().unwrap().to_str().unwrap();
        let json = run_emit_ir(fixture);
        let ir = parse_ir_json(&json);
        assert_version_and_components(&ir);

        let ts = run_emit_contracts(fixture);
        assert!(ts.contains("export type FrameEventContext"));
        assert!(ts.contains("FrameEventContext<TState, TProps>"));
    }
}
