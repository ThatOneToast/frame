use frame_core::style::{document_contract, document_recipes, document_themes, StyleContext};
use frame_core::{DeclarationKind, Document};

pub fn generate_typescript(document: &Document) -> String {
    let mut ts = String::from("export const ui = {\n");

    for declaration in &document.declarations {
        if matches!(
            declaration.kind,
            DeclarationKind::Tokens
                | DeclarationKind::Theme
                | DeclarationKind::Motion
                | DeclarationKind::Keyframes
        ) {
            continue;
        }
        ts.push_str(&format!(
            "  {}: 'fr-{}',\n",
            property_name(&declaration.name.text),
            declaration.name.text
        ));
    }

    ts.push_str("} as const;\n\n");
    ts.push_str("export type UiClass = keyof typeof ui;\n");

    let themes = document_themes(document);
    if !themes.is_empty() {
        ts.push('\n');
        ts.push_str("export const themes = [");
        ts.push_str(
            &themes
                .iter()
                .map(|theme| format!("'{}'", theme.name))
                .collect::<Vec<_>>()
                .join(", "),
        );
        ts.push_str("] as const;\n\n");
        ts.push_str("export type FrameTheme = (typeof themes)[number];\n\n");
        ts.push_str(&format!(
            "export const defaultTheme: FrameTheme = '{}';\n\n",
            themes[0].name
        ));
        ts.push_str(
            "export function applyTheme(theme: FrameTheme, root: HTMLElement = document.documentElement): void {\n  root.setAttribute('data-frame-theme', theme);\n}\n",
        );
    }

    let contract = document_contract(document);
    let ctx = StyleContext::new(&contract);
    let recipes = document_recipes(document, &ctx);
    if !recipes.is_empty() {
        ts.push('\n');
        ts.push_str("export const recipes = {\n");
        for recipe in &recipes {
            ts.push_str(&format!(
                "  {}: {{\n    base: '{}',\n    variants: {{\n",
                property_name(&recipe.name),
                recipe.base_class()
            ));
            for group in &recipe.variants {
                ts.push_str(&format!("      {}: {{\n", property_name(&group.name)));
                for (option, _) in &group.options {
                    ts.push_str(&format!(
                        "        {}: '{}',\n",
                        property_name(option),
                        recipe.variant_class(&group.name, option)
                    ));
                }
                ts.push_str("      },\n");
            }
            ts.push_str("    },\n  },\n");
        }
        ts.push_str("} as const;\n\n");
        ts.push_str("export type RecipeName = keyof typeof recipes;\n");
    }
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
            includes: Vec::new(),
            declarations: vec![Declaration {
                kind: DeclarationKind::Card,
                name: Identifier::new("QuickLinkCard", Span::default()),
                extends: None,
                body: vec![],
                span: Span::default(),
            }],
            components: Vec::new(),
        };

        let ts = generate_typescript(&document);
        assert!(ts.contains("QuickLinkCard: 'fr-QuickLinkCard'"));
    }

    #[test]
    fn generates_theme_exports() {
        let document = Document {
            includes: Vec::new(),
            declarations: vec![
                Declaration {
                    kind: DeclarationKind::Theme,
                    name: Identifier::new("dark", Span::default()),
                    extends: Some(Identifier::new("default", Span::default())),
                    body: vec![],
                    span: Span::default(),
                },
                Declaration {
                    kind: DeclarationKind::Theme,
                    name: Identifier::new("light", Span::default()),
                    extends: Some(Identifier::new("default", Span::default())),
                    body: vec![],
                    span: Span::default(),
                },
            ],
            components: Vec::new(),
        };

        let ts = generate_typescript(&document);
        assert!(ts.contains("export const themes = ['dark', 'light'] as const;"));
        assert!(ts.contains("export const defaultTheme: FrameTheme = 'dark';"));
        assert!(ts.contains("export function applyTheme"));
        // Themes are not classes.
        assert!(!ts.contains("dark: 'fr-dark'"));
    }
}
