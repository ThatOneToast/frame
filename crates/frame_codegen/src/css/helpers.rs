use frame_core::{Declaration, DeclarationKind, Identifier};

pub(crate) fn declaration_from_block(block: &frame_core::Block) -> Option<Declaration> {
    let mut parts = block.name.split_whitespace();
    let kind_text = parts.next()?;
    let kind = match kind_text {
        "grid" => DeclarationKind::Grid,
        "area" => DeclarationKind::Area,
        "card" => DeclarationKind::Card,
        "stack" => DeclarationKind::Stack,
        "row" => DeclarationKind::Row,
        "button" => DeclarationKind::Button,
        "text" => DeclarationKind::Text,
        "center" => DeclarationKind::Center,
        "split" => DeclarationKind::Split,
        "overlay" => DeclarationKind::Overlay,
        "dock" => DeclarationKind::Dock,
        "keyframes" => DeclarationKind::Keyframes,
        "html" => DeclarationKind::Html,
        "page-body" => DeclarationKind::Body,
        _ => return None,
    };

    let unnamed = matches!(kind, DeclarationKind::Html | DeclarationKind::Body);
    let name_text = if unnamed {
        kind_text.to_string()
    } else {
        parts.next()?.to_string()
    };

    Some(Declaration {
        kind,
        name: Identifier::new(&name_text, block.span),
        extends: None,
        body: block.body.clone(),
        span: block.span,
    })
}

pub(crate) fn style_order_names(order: &str) -> Vec<String> {
    order
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub(crate) fn is_keyframe_selector(name: &str) -> bool {
    matches!(name, "from" | "to")
        || name
            .strip_suffix('%')
            .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}
