//! Grammar drift test — ensures Zed tree-sitter keyword arrays stay aligned
//! with the canonical language registry in `crates/frame_core/src/language.rs`.

use std::collections::HashSet;

fn grammar_js_path() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("editors/zed/tree-sitter-frame/grammar.js")
}

fn extract_array(source: &str, name: &str) -> Vec<String> {
    let pattern = format!(r#"(?s)const\s+{}\s+=\s+\[(.*?)\];"#, regex::escape(name));
    let re = regex::Regex::new(&pattern).unwrap();
    let caps = re.captures(source).unwrap_or_else(|| {
        panic!("could not find array {name} in grammar.js");
    });
    let body = caps.get(1).unwrap().as_str();
    body.lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Keywords that have dedicated grammar rules in tree-sitter and therefore
/// do not appear in the generic keyword arrays.
const DEDICATED_RULE_KEYWORDS: &[&str] = &["component", "supports", "style-group", "style-order"];

#[test]
fn declaration_keywords_match_registry() {
    let source = std::fs::read_to_string(grammar_js_path()).unwrap();
    let grammar_names = extract_array(&source, "DECLARATION_KEYWORDS");
    let grammar: HashSet<&str> = grammar_names.iter().map(|s| s.as_str()).collect();

    let mut registry_names: Vec<&str> = frame_core::language::declaration_keywords().to_vec();
    registry_names.extend(frame_core::language::ui_primitive_keywords());
    registry_names.sort();
    registry_names.dedup();
    let registry: HashSet<&str> = registry_names.iter().copied().collect();

    // Every grammar keyword must exist in the registry
    let missing_in_registry: Vec<_> = grammar.difference(&registry).collect();
    assert!(
        missing_in_registry.is_empty(),
        "grammar.js DECLARATION_KEYWORDS have items not in registry: {:?}",
        missing_in_registry
    );

    // Registry declarations (minus dedicated-rule ones) should all be in grammar.js
    let registry_minus_dedicated: HashSet<&str> = registry
        .iter()
        .copied()
        .filter(|k| !DEDICATED_RULE_KEYWORDS.contains(k))
        .collect();
    let missing_in_grammar: Vec<_> = registry_minus_dedicated.difference(&grammar).collect();
    assert!(
        missing_in_grammar.is_empty(),
        "registry declaration/primitive keywords missing in grammar.js DECLARATION_KEYWORDS: {:?}",
        missing_in_grammar
    );
}

#[test]
fn ui_element_keywords_match_registry() {
    let source = std::fs::read_to_string(grammar_js_path()).unwrap();
    let grammar_names = extract_array(&source, "UI_ELEMENT_KEYWORDS");
    let grammar: HashSet<&str> = grammar_names.iter().map(|s| s.as_str()).collect();

    let mut registry_names: Vec<&str> = frame_core::language::declaration_keywords().to_vec();
    registry_names.extend(frame_core::language::ui_primitive_keywords());
    registry_names.sort();
    registry_names.dedup();
    let registry: HashSet<&str> = registry_names.iter().copied().collect();

    let missing_in_registry: Vec<_> = grammar.difference(&registry).collect();
    assert!(
        missing_in_registry.is_empty(),
        "grammar.js UI_ELEMENT_KEYWORDS have items not in registry: {:?}",
        missing_in_registry
    );

    // UI_ELEMENT_KEYWORDS excludes dedicated-rule keywords and also root-only declarations
    // like "tokens" and "keyframes" which do not appear inside view blocks.
    let ui_exceptions: HashSet<&str> = ["tokens", "keyframes"].iter().copied().collect();
    let registry_minus_dedicated: HashSet<&str> = registry
        .iter()
        .copied()
        .filter(|k| !DEDICATED_RULE_KEYWORDS.contains(k) && !ui_exceptions.contains(k))
        .collect();
    let missing_in_grammar: Vec<_> = registry_minus_dedicated.difference(&grammar).collect();
    assert!(
        missing_in_grammar.is_empty(),
        "registry declaration/primitive keywords missing in grammar.js UI_ELEMENT_KEYWORDS: {:?}",
        missing_in_grammar
    );
}

#[test]
fn state_keywords_match_registry() {
    let source = std::fs::read_to_string(grammar_js_path()).unwrap();
    let grammar_names = extract_array(&source, "STATE_KEYWORDS");
    let grammar: HashSet<&str> = grammar_names.iter().map(|s| s.as_str()).collect();
    let registry: HashSet<&str> = frame_core::language::state_keywords()
        .iter()
        .copied()
        .collect();
    let missing_in_grammar: Vec<_> = registry.difference(&grammar).collect();
    let extra_in_grammar: Vec<_> = grammar.difference(&registry).collect();
    assert!(
        missing_in_grammar.is_empty(),
        "registry state keywords missing in grammar.js STATE_KEYWORDS: {:?}",
        missing_in_grammar
    );
    assert!(
        extra_in_grammar.is_empty(),
        "grammar.js STATE_KEYWORDS has extras not in registry: {:?}",
        extra_in_grammar
    );
}

#[test]
fn event_names_match_registry() {
    let source = std::fs::read_to_string(grammar_js_path()).unwrap();
    let grammar_names = extract_array(&source, "UI_EVENT_NAMES");
    let grammar: HashSet<&str> = grammar_names.iter().map(|s| s.as_str()).collect();
    let registry: HashSet<&str> = frame_core::language::event_keywords()
        .iter()
        .copied()
        .collect();
    let missing_in_grammar: Vec<_> = registry.difference(&grammar).collect();
    let extra_in_grammar: Vec<_> = grammar.difference(&registry).collect();
    assert!(
        missing_in_grammar.is_empty(),
        "registry event keywords missing in grammar.js UI_EVENT_NAMES: {:?}",
        missing_in_grammar
    );
    assert!(
        extra_in_grammar.is_empty(),
        "grammar.js UI_EVENT_NAMES has extras not in registry: {:?}",
        extra_in_grammar
    );
}

#[test]
fn event_modifiers_match_registry() {
    let source = std::fs::read_to_string(grammar_js_path()).unwrap();
    let grammar_names = extract_array(&source, "UI_EVENT_MODIFIERS");
    let grammar: HashSet<&str> = grammar_names.iter().map(|s| s.as_str()).collect();
    let registry: HashSet<&str> = frame_core::language::event_modifiers()
        .iter()
        .copied()
        .collect();
    let missing_in_grammar: Vec<_> = registry.difference(&grammar).collect();
    let extra_in_grammar: Vec<_> = grammar.difference(&registry).collect();
    assert!(
        missing_in_grammar.is_empty(),
        "registry event modifiers missing in grammar.js UI_EVENT_MODIFIERS: {:?}",
        missing_in_grammar
    );
    assert!(
        extra_in_grammar.is_empty(),
        "grammar.js UI_EVENT_MODIFIERS has extras not in registry: {:?}",
        extra_in_grammar
    );
}
