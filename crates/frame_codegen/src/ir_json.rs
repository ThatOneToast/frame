use frame_core::{ir::FrameIrDocument, Document};

pub fn generate_ir_json(document: &Document) -> Result<String, serde_json::Error> {
    let ir = frame_core::ir::lower_document_to_ir(document);
    generate_ir_json_from_ir(&ir)
}

pub fn generate_ir_typescript(document: &Document) -> Result<String, serde_json::Error> {
    let ir = frame_core::ir::lower_document_to_ir(document);
    generate_ir_typescript_from_ir(&ir)
}

pub fn generate_ir_json_from_ir(ir: &FrameIrDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(ir).map(|json| format!("{json}\n"))
}

pub fn generate_ir_typescript_from_ir(ir: &FrameIrDocument) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string_pretty(ir)?;
    Ok(format!(
        "import {{ defineFrameIrDocument }} from '@frame/runtime-dom';\n\nconst ir = defineFrameIrDocument({json} as const);\n\nexport default ir;\n"
    ))
}

#[cfg(test)]
mod tests {
    use frame_core::{
        ir::{FrameIrDocument, FRAME_IR_VERSION},
        ComponentDecl, Document, Identifier, Span, StateDecl, StateDefault, StateType, StateValue,
    };

    use super::*;

    #[test]
    fn serializes_initial_ui_ir_as_stable_json() {
        let document = Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: vec![ComponentDecl {
                name: Identifier::new("Counter", Span { start: 0, end: 20 }),
                props: None,
                state: Some(StateDecl {
                    values: vec![StateValue {
                        name: Identifier::new("count", Span { start: 10, end: 15 }),
                        value_type: StateType::Number,
                        default: StateDefault::Number("0".to_string()),
                        span: Span { start: 10, end: 26 },
                    }],
                    span: Span { start: 8, end: 28 },
                }),
                view: None,
                slots: Vec::new(),
                span: Span { start: 0, end: 30 },
            }],
        };

        let json = generate_ir_json(&document).expect("json");

        assert!(json.contains(&format!("\"version\": {FRAME_IR_VERSION}")));
        assert!(json.contains("\"components\""));
        assert!(json.contains("\"name\": \"Counter\""));
        assert!(json.contains("\"value_type\": \"Number\""));
        assert!(json.contains("\"Number\": \"0\""));
        assert!(json.ends_with('\n'));
    }

    #[test]
    fn serializes_empty_ir_document() {
        let ir = FrameIrDocument {
            version: FRAME_IR_VERSION,
            components: Vec::new(),
        };

        let json = generate_ir_json_from_ir(&ir).expect("json");

        assert_eq!(json, "{\n  \"version\": 1,\n  \"components\": []\n}\n");
    }

    #[test]
    fn emits_typescript_ir_module_with_literal_type_check() {
        let ir = FrameIrDocument {
            version: FRAME_IR_VERSION,
            components: Vec::new(),
        };

        let ts = generate_ir_typescript_from_ir(&ir).expect("ts");

        assert!(ts.starts_with("import { defineFrameIrDocument } from '@frame/runtime-dom';"));
        assert!(ts.contains("const ir = defineFrameIrDocument({"));
        assert!(ts.contains("\"components\": []"));
        assert!(ts.contains("} as const);"));
        assert!(ts.ends_with("export default ir;\n"));
    }
}
