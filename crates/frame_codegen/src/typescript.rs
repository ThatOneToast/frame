use frame_core::Document;

pub fn generate_typescript(document: &Document) -> String {
    let mut ts = String::from("export const ui = {\n");

    for declaration in &document.declarations {
        ts.push_str(&format!(
            "  {}: 'fr-{}',\n",
            property_name(&declaration.name.text),
            declaration.name.text
        ));
    }

    ts.push_str("} as const;\n\n");
    ts.push_str("export type UiClass = keyof typeof ui;\n");
    ts
}

fn property_name(name: &str) -> String {
    if is_typescript_identifier(name) {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}

fn is_typescript_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    (first == '_' || first == '$' || first.is_ascii_alphabetic())
        && chars.all(|char| char == '_' || char == '$' || char.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_core::{Declaration, DeclarationKind, Document, Identifier, Span};

    #[test]
    fn generates_ui_exports() {
        let document = Document {
            declarations: vec![Declaration {
                kind: DeclarationKind::Card,
                name: Identifier::new("QuickLinkCard", Span::default()),
                body: vec![],
                span: Span::default(),
            }],
        };

        let ts = generate_typescript(&document);
        assert!(ts.contains("QuickLinkCard: 'fr-QuickLinkCard'"));
    }
}
