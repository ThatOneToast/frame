use super::*;
use frame_core::{Declaration, DeclarationKind, Identifier, Node, Span, Statement};

fn declaration(kind: DeclarationKind, name: &str, body: Vec<Node>) -> Declaration {
    Declaration {
        kind,
        name: Identifier::new(name, Span::default()),
        extends: None,
        body,
        span: Span::default(),
    }
}

fn statement(words: &[&str]) -> Node {
    Node::Statement(Statement {
        words: words.iter().map(|word| word.to_string()).collect(),
        span: Span::default(),
    })
}

#[test]
fn generates_named_grid_columns_and_area_placement() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Grid,
                "AppShell",
                vec![statement(&["columns", "sidebar", "content", "inspector"])],
            ),
            declaration(
                DeclarationKind::Area,
                "Sidebar",
                vec![statement(&["place", "sidebar"])],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-areas: \"sidebar content inspector\";"));
    assert!(css.contains("grid-area: sidebar;"));
}

#[test]
fn grids_emit_common_and_advanced_properties() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "AppShell",
            vec![
                statement(&["columns", "sidebar", "content"]),
                statement(&["background", "panel"]),
                Node::Block(frame_core::Block {
                    name: "advanced".to_string(),
                    body: vec![statement(&[
                        "css",
                        "\"grid-template-areas\"",
                        "\"header",
                        "header\"",
                        "\"sidebar",
                        "content\"",
                    ])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("background: var(--frame-surface-panel);"));
    assert!(css.contains("grid-template-areas: \"header header\" \"sidebar content\";"));
}

#[test]
fn generates_app_driven_layout_vocabulary() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Grid,
                "AppShell",
                vec![
                    statement(&["columns", "header", "sidebar", "content", "users"]),
                    statement(&["tracks", "columns", "rail", "panel", "fill", "side"]),
                    statement(&["tracks", "rows", "header", "fill", "composer"]),
                    statement(&["areas", "header", "header", "header", "header"]),
                    statement(&["areas", "sidebar", "channels", "chat", "users"]),
                    statement(&["areas", "composer", "composer", "composer", "composer"]),
                    statement(&["overflow", "hidden"]),
                    statement(&["box", "border"]),
                ],
            ),
            declaration(
                DeclarationKind::Button,
                "ChannelButton",
                vec![
                    statement(&["layout", "icon-content-action"]),
                    statement(&["gap", "small"]),
                    statement(&["control", "reset"]),
                    statement(&["interactive"]),
                    statement(&["align-text", "left"]),
                    statement(&["width", "fill"]),
                ],
            ),
            declaration(
                DeclarationKind::Text,
                "ChannelName",
                vec![statement(&["truncate"])],
            ),
            declaration(
                DeclarationKind::Text,
                "MessageText",
                vec![
                    statement(&["margin", "none"]),
                    statement(&["wrap", "anywhere"]),
                    statement(&["line", "relaxed"]),
                    statement(&["letter", "normal"]),
                ],
            ),
            declaration(
                DeclarationKind::Center,
                "PresenceDot",
                vec![statement(&["square", "presence"])],
            ),
            declaration(
                DeclarationKind::Area,
                "Panel",
                vec![
                    statement(&["border", "right", "accent"]),
                    statement(&["scroll", "y"]),
                    statement(&["scrollbar", "dense"]),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: 4.5rem 18rem minmax(0, 1fr) 16rem;"));
    assert!(css.contains("grid-template-rows: 3.25rem minmax(0, 1fr) 4.75rem;"));
    assert!(css.contains(
        "grid-template-areas: \"header header header header\" \"sidebar channels chat users\" \"composer composer composer composer\";"
    ));
    assert!(css.contains("grid-template-columns: auto minmax(0, 1fr) auto;"));
    assert!(css.contains("appearance: none;"));
    assert!(css.contains("cursor: pointer;"));
    assert!(css.contains("text-align: left;"));
    assert!(css.contains("white-space: nowrap;"));
    assert!(css.contains("overflow-wrap: anywhere;"));
    assert!(css.contains("line-height: 1.45;"));
    assert!(css.contains("letter-spacing: 0;"));
    assert!(css.contains("width: 0.65rem;\n  height: 0.65rem;"));
    assert!(css.contains("border-right: 1px solid var(--frame-color-accent);"));
    assert!(css.contains("overflow-y: auto;"));
    assert!(css.contains("scrollbar-width: thin;"));
}

#[test]
fn generates_display_flex_visibility_and_logical_sizing() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "Panel",
            vec![
                statement(&["display", "inline-flex"]),
                statement(&["visibility", "hidden"]),
                statement(&["flex", "direction", "column"]),
                statement(&["flex", "wrap", "wrap"]),
                statement(&["flex", "grow", "1"]),
                statement(&["flex", "shrink", "0"]),
                statement(&["flex", "basis", "fill"]),
                statement(&["inline-size", "fill"]),
                statement(&["block-size", "screen"]),
                statement(&["min-inline-size", "zero"]),
                statement(&["max-block-size", "100%"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: inline-flex;"));
    assert!(css.contains("visibility: hidden;"));
    assert!(css.contains("flex-direction: column;"));
    assert!(css.contains("flex-wrap: wrap;"));
    assert!(css.contains("flex-grow: 1;"));
    assert!(css.contains("flex-shrink: 0;"));
    assert!(css.contains("flex-basis: 100%;"));
    assert!(css.contains("inline-size: 100%;"));
    assert!(css.contains("block-size: 100vh;"));
    assert!(css.contains("min-inline-size: 0;"));
    assert!(css.contains("max-block-size: 100%;"));
}

#[test]
fn generates_expanded_typography_controls() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Text,
            "MessageBody",
            vec![
                statement(&["align-text", "justify"]),
                statement(&["case", "capitalize"]),
                statement(&["decoration", "line-through"]),
                statement(&["whitespace", "pre-wrap"]),
                statement(&["word-break", "break-word"]),
                statement(&["hyphenate", "auto"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("text-align: justify;"));
    assert!(css.contains("text-transform: capitalize;"));
    assert!(css.contains("text-decoration-line: line-through;"));
    assert!(css.contains("white-space: pre-wrap;"));
    assert!(css.contains("word-break: break-word;"));
    assert!(css.contains("hyphens: auto;"));
}

#[test]
fn generates_border_styles_and_outline_offsets() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "Panel",
            vec![
                statement(&["border", "style", "dotted"]),
                statement(&["border", "width", "large"]),
                statement(&["outline", "accent"]),
                statement(&["outline", "offset", "small"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("border-style: dotted;"));
    assert!(css.contains("border-width: 3px;"));
    assert!(css.contains("outline: 2px solid var(--frame-color-accent);"));
    assert!(css.contains("outline-offset: var(--frame-space-small);"));
}

#[test]
fn generates_responsive_card_grid_and_hover_effects() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Grid,
                "QuickLinks",
                vec![statement(&["columns", "responsive", "cards"])],
            ),
            declaration(
                DeclarationKind::Card,
                "QuickLinkCard",
                vec![Node::Block(frame_core::Block {
                    name: "hover".to_string(),
                    body: vec![
                        statement(&["lift", "small"]),
                        statement(&["glow", "accent"]),
                        statement(&["brighten", "subtle"]),
                    ],
                    span: Span::default(),
                })],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("repeat(auto-fit, minmax(220px, 1fr))"));
    assert!(css.contains(".fr-QuickLinkCard:hover"));
    assert!(css.contains("transform: translateY(-4px);"));
    assert!(css.contains("filter: brightness(1.04);"));
}

#[test]
fn generates_intent_motion_helpers_in_source_order() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "FloatingCard",
            vec![
                statement(&["lift", "small"]),
                statement(&["tilt", "right", "subtle"]),
                statement(&["grow", "slight"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("transform: translateY(-4px) rotate(1deg) scale(1.02);"));
}

#[test]
fn generates_intent_motion_helpers_in_state_blocks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Button,
            "Send",
            vec![
                Node::Block(frame_core::Block {
                    name: "hover".to_string(),
                    body: vec![
                        statement(&["lift", "small"]),
                        statement(&["grow", "slight"]),
                    ],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "active".to_string(),
                    body: vec![statement(&["press"])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains(".fr-Send:hover"));
    assert!(css.contains("transform: translateY(-4px) scale(1.02);"));
    assert!(css.contains(".fr-Send:active"));
    assert!(css.contains("transform: translateY(1px);"));
}

#[test]
fn generates_tuned_motion_interpolation_and_extrapolation() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "TunedFloatingCard",
            vec![
                statement(&["lift", "small%50"]),
                statement(&["sink", "huge%50"]),
                statement(&["shift", "left", "tiny%100"]),
                statement(&["tilt", "right", "subtle%25"]),
                statement(&["grow", "slight%50"]),
                statement(&["shrink", "dramatic%50"]),
                statement(&["pop"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("translateY(-6px)"));
    assert!(css.contains("translateY(18px)"));
    assert!(css.contains("translateX(-4px)"));
    assert!(css.contains("rotate(1.25deg)"));
    assert!(css.contains("scale(1.03)"));
    assert!(css.contains("scale(0.81)"));
    assert!(css.contains("scale(1.04)"));
}

#[test]
fn generates_expanded_interaction_state_selectors() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Button,
            "FieldButton",
            vec![
                Node::Block(frame_core::Block {
                    name: "focus-visible".to_string(),
                    body: vec![statement(&["ring", "accent"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "focus-within".to_string(),
                    body: vec![statement(&["ring", "accent"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "checked".to_string(),
                    body: vec![statement(&["glow", "accent"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "invalid".to_string(),
                    body: vec![statement(&["ring", "danger"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "required".to_string(),
                    body: vec![statement(&["glow", "warning"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "target".to_string(),
                    body: vec![statement(&["glow", "accent"])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains(".fr-FieldButton:focus-visible"));
    assert!(css.contains(".fr-FieldButton:focus-within"));
    assert!(css.contains(".fr-FieldButton:checked"));
    assert!(css.contains(".fr-FieldButton:invalid"));
    assert!(css.contains(".fr-FieldButton:required"));
    assert!(css.contains(".fr-FieldButton:target"));
}

#[test]
fn generates_expanded_layout_and_type_concepts() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Center,
                "EmptyState",
                vec![
                    statement(&["height", "screen"]),
                    statement(&["surface", "glass"]),
                ],
            ),
            declaration(
                DeclarationKind::Row,
                "Toolbar",
                vec![
                    statement(&["align", "center"]),
                    statement(&["justify", "between"]),
                    statement(&["border", "accent"]),
                ],
            ),
            declaration(
                DeclarationKind::Text,
                "PageTitle",
                vec![
                    statement(&["size", "heading"]),
                    statement(&["weight", "bold"]),
                    statement(&["color", "bright"]),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains(".fr-EmptyState"));
    assert!(css.contains("place-items: center;"));
    assert!(css.contains("height: 100vh;"));
    assert!(css.contains("justify-content: space-between;"));
    assert!(css.contains("border: 1px solid var(--frame-color-accent);"));
    assert!(css.contains("font-size: 2rem;"));
    assert!(css.contains("font-weight: 700;"));
}

#[test]
fn generates_percentage_columns_and_sizes() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Grid,
                "Dashboard",
                vec![statement(&["columns", "25%", "50%", "25%"])],
            ),
            declaration(
                DeclarationKind::Area,
                "Sidebar",
                vec![statement(&["width", "25%"]), statement(&["height", "100%"])],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: 25% 50% 25%;"));
    assert!(!css.contains("grid-template-areas: \"25% 50% 25%\";"));
    assert!(css.contains("width: 25%;"));
    assert!(css.contains("height: 100%;"));
}

#[test]
fn generates_vertical_grid_flow_and_section_spacing() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "HoverCardInfo",
            vec![
                statement(&["flow", "vertical"]),
                statement(&["columns", "title", "description"]),
                statement(&["gap", "small"]),
                Node::Block(frame_core::Block {
                    name: "section title".to_string(),
                    body: vec![statement(&["padding", "bottom", "small"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "section description".to_string(),
                    body: vec![statement(&["padding", "top", "none"])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: minmax(0, 1fr);"));
    assert!(css.contains("grid-template-areas: \"title\" \"description\";"));
    assert!(css.contains(".fr-HoverCardInfo > :nth-child(1)"));
    assert!(css.contains("grid-area: title;"));
    assert!(css.contains("padding-bottom: var(--frame-space-small);"));
    assert!(css.contains("padding-top: var(--frame-space-none);"));
}

#[test]
fn generates_numeric_area_placement() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Area,
            "Sidebar",
            vec![statement(&["col", "1"]), statement(&["row", "2"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-column: 1;"));
    assert!(css.contains("grid-row: 2;"));
}

#[test]
fn generates_expanded_color_and_surface_tokens() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "Status",
            vec![
                statement(&["surface", "raised"]),
                statement(&["text", "primary"]),
                statement(&["background", "danger"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("--frame-color-primary"));
    assert!(css.contains("--frame-color-secondary"));
    assert!(css.contains("--frame-color-info"));
    assert!(css.contains("background: var(--frame-color-danger);"));
    assert!(css.contains("color: var(--frame-color-primary);"));
}

#[test]
fn generates_custom_colors_borders_and_animation() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![statement(&["color", "brand", "#7c3aed"])],
            ),
            declaration(
                DeclarationKind::Card,
                "BrandCard",
                vec![
                    statement(&["background", "brand"]),
                    statement(&["border", "brand"]),
                    statement(&["border", "width", "medium"]),
                    statement(&["transition", "smooth"]),
                    statement(&["animation", "fade-in"]),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("--frame-color-brand: #7c3aed;"));
    assert!(css.contains("background: var(--frame-color-brand);"));
    assert!(css.contains("border: 1px solid var(--frame-color-brand);"));
    assert!(css.contains("border-width: 2px;"));
    assert!(css.contains("transition: all 200ms ease;"));
    assert!(css.contains("animation: frame-fade-in 240ms ease both;"));
    assert!(css.contains("@keyframes frame-fade-in"));
}

#[test]
fn generates_custom_gradient_tokens_and_advanced_css() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![
                    statement(&["color", "brand-purple", "#7c3aed"]),
                    statement(&["color", "brand-bg", "#0f172a"]),
                    Node::Block(frame_core::Block {
                        name: "gradient hero-gradient".to_string(),
                        body: vec![
                            statement(&["type", "linear"]),
                            statement(&["angle", "135deg"]),
                            statement(&["stop", "brand-purple", "0%"]),
                            statement(&["stop", "brand-bg", "100%"]),
                        ],
                        span: Span::default(),
                    }),
                ],
            ),
            declaration(
                DeclarationKind::Card,
                "HeroCard",
                vec![
                    statement(&["background", "hero-gradient"]),
                    Node::Block(frame_core::Block {
                        name: "advanced".to_string(),
                        body: vec![statement(&["css", "\"backdrop-filter\"", "blur(12px)"])],
                        span: Span::default(),
                    }),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("--frame-gradient-hero-gradient: linear-gradient(135deg, var(--frame-color-brand-purple) 0%, var(--frame-color-brand-bg) 100%);"));
    assert!(css.contains("background: var(--frame-gradient-hero-gradient);"));
    assert!(css.contains("backdrop-filter: blur(12px);"));
}

#[test]
fn generates_custom_keyframes_and_animation_blocks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Keyframes,
                "FloatIn",
                vec![
                    Node::Block(frame_core::Block {
                        name: "from".to_string(),
                        body: vec![
                            statement(&["opacity", "0"]),
                            statement(&["transform", "translateY(12px)", "scale(0.98)"]),
                        ],
                        span: Span::default(),
                    }),
                    Node::Block(frame_core::Block {
                        name: "to".to_string(),
                        body: vec![
                            statement(&["opacity", "1"]),
                            statement(&["transform", "translateY(0)", "scale(1)"]),
                        ],
                        span: Span::default(),
                    }),
                ],
            ),
            declaration(
                DeclarationKind::Card,
                "Panel",
                vec![Node::Block(frame_core::Block {
                    name: "animation FloatIn".to_string(),
                    body: vec![
                        statement(&["duration", "240ms"]),
                        statement(&["ease", "smooth"]),
                        statement(&["fill", "both"]),
                    ],
                    span: Span::default(),
                })],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("@keyframes frame-FloatIn"));
    assert!(css.contains("from {"));
    assert!(css.contains("opacity: 0;"));
    assert!(css.contains("transform: translateY(12px) scale(0.98);"));
    assert!(css.contains("animation: frame-FloatIn 240ms ease 0ms 1 normal both;"));
}

#[test]
fn generates_responsive_and_container_rules() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "AppShell",
            vec![
                statement(&["columns", "sidebar", "content", "inspector"]),
                Node::Block(frame_core::Block {
                    name: "below tablet".to_string(),
                    body: vec![statement(&["columns", "content"])],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "container narrow".to_string(),
                    body: vec![statement(&["columns", "content"])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("@media (max-width: 1023px)"));
    assert!(css.contains("@container (max-width: 42rem)"));
    assert!(css.contains(".fr-AppShell"));
}

#[test]
fn generates_typed_supports_blocks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Supports,
                "display grid",
                vec![Node::Block(frame_core::Block {
                    name: "grid AppShell".to_string(),
                    body: vec![statement(&["columns", "sidebar", "content"])],
                    span: Span::default(),
                })],
            ),
            declaration(
                DeclarationKind::Supports,
                "selector has",
                vec![Node::Block(frame_core::Block {
                    name: "card ParentAware".to_string(),
                    body: vec![statement(&["border", "accent"])],
                    span: Span::default(),
                })],
            ),
            declaration(
                DeclarationKind::Supports,
                "subgrid",
                vec![Node::Block(frame_core::Block {
                    name: "grid NestedGrid".to_string(),
                    body: vec![statement(&["columns", "subgrid"])],
                    span: Span::default(),
                })],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("@supports (display: grid)"));
    assert!(css.contains(".fr-AppShell"));
    assert!(css.contains("@supports selector(:has(*))"));
    assert!(css.contains(".fr-ParentAware"));
    assert!(css.contains("@supports (grid-template-columns: subgrid)"));
    assert!(css.contains("grid-template-columns: subgrid;"));
}

#[test]
fn generates_style_groups_and_style_order() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::StyleOrder,
                "reset, base, components, utilities",
                Vec::new(),
            ),
            declaration(
                DeclarationKind::StyleGroup,
                "components",
                vec![Node::Block(frame_core::Block {
                    name: "button PrimaryButton".to_string(),
                    body: vec![
                        statement(&["surface", "panel"]),
                        statement(&["radius", "medium"]),
                    ],
                    span: Span::default(),
                })],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("@layer reset, base, components, utilities;"));
    assert!(css.contains("@layer components"));
    assert!(css.contains(".fr-PrimaryButton"));
    assert!(css.contains("background: var(--frame-surface-panel);"));
    assert!(css.contains("border-radius: var(--frame-radius-medium);"));
}

#[test]
fn generates_corner_gradient_layers_targeted_padding_and_anchor() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Tokens,
                "Brand",
                vec![
                    statement(&["color", "brand-purple", "#7c3aed"]),
                    statement(&["color", "brand-panel", "#181820"]),
                    Node::Block(frame_core::Block {
                        name: "gradient four-corners".to_string(),
                        body: vec![
                            statement(&["type", "layered"]),
                            statement(&["corner", "top-left", "brand-purple", "65%"]),
                            statement(&["corner", "bottom-right", "brand-panel", "70%"]),
                        ],
                        span: Span::default(),
                    }),
                ],
            ),
            declaration(
                DeclarationKind::Card,
                "PinnedHero",
                vec![
                    statement(&["background", "four-corners"]),
                    statement(&["padding", "top", "large"]),
                    statement(&["padding", "x", "medium"]),
                    statement(&["anchor", "top"]),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("radial-gradient(circle at top left"));
    assert!(css.contains("radial-gradient(circle at bottom right"));
    assert!(css.contains("padding-top: var(--frame-space-large);"));
    assert!(css.contains("padding-inline: var(--frame-space-medium);"));
    assert!(css.contains("position: sticky;"));
    assert!(css.contains("top: 0;"));
}

#[test]
fn page_body_emits_global_body_rule() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Body,
            "page-body",
            vec![
                statement(&["margin", "none"]),
                statement(&["background", "#0a0f1a"]),
                statement(&["color", "#e2e8f0"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("body {"));
    assert!(css.contains("margin: 0;"));
    assert!(css.contains("min-height: 100vh;"));
    assert!(css.contains("background: #0a0f1a;"));
    assert!(css.contains("color: #e2e8f0;"));
    assert!(!css.contains(".fr-page-body"));
}

#[test]
fn html_declaration_emits_global_html_rule() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Html,
            "html",
            vec![
                statement(&["background", "#0A0A0F"]),
                statement(&["color", "#F8FAFC"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("html {"));
    assert!(css.contains("background: #0A0A0F;"));
    assert!(css.contains("color: #F8FAFC;"));
    assert!(!css.contains(".fr-html"));
}

#[test]
fn html_and_page_body_do_not_emit_component_classes() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Html,
                "html",
                vec![statement(&["background", "#000"])],
            ),
            declaration(
                DeclarationKind::Body,
                "page-body",
                vec![statement(&["margin", "none"])],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(!css.contains(".fr-html"));
    assert!(!css.contains(".fr-page-body"));
    assert!(css.contains("html {"));
    assert!(css.contains("body {"));
}

#[test]
fn emits_opacity_with_abstract_values() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "FadeBox",
            vec![
                statement(&["opacity", "half"]),
                statement(&["opacity", "strong"]),
                statement(&["opacity", "none"]),
                statement(&["opacity", "full"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("opacity: 0.5;"));
    assert!(css.contains("opacity: 0.75;"));
    assert!(css.contains("opacity: 0;"));
    assert!(css.contains("opacity: 1.0;"));
}

#[test]
fn emits_shadow_with_css_variable() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "ShadowCard",
            vec![statement(&["shadow", "medium"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("box-shadow: var(--frame-shadow-medium);"));
}

#[test]
fn emits_radius_with_css_variable() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "RoundCard",
            vec![statement(&["radius", "large"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("border-radius: var(--frame-radius-large);"));
}

#[test]
fn emits_z_index_for_named_layers() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "ZStack",
            vec![
                statement(&["z", "overlay"]),
                statement(&["z", "modal"]),
                statement(&["z", "base"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("z-index: 50;"));
    assert!(css.contains("z-index: 100;"));
    assert!(css.contains("z-index: 0;"));
}

#[test]
fn emits_surface_background_for_named_surfaces() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "GlassCard",
            vec![statement(&["surface", "glass"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("background: var(--frame-surface-glass);"));
}

#[test]
fn emits_grid_columns_with_fr_proportions() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "ProportionalGrid",
            vec![statement(&["columns", "2fr", "1fr"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);"));
    assert!(!css.contains("grid-area: 2fr"));
    assert!(!css.contains("grid-area: 1fr"));
}

#[test]
fn stack_emits_flex_column_layout() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "NavGroup",
            vec![
                statement(&["gap", "small"]),
                statement(&["padding", "x", "small"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: flex;"));
    assert!(css.contains("flex-direction: column;"));
    assert!(css.contains("gap: var(--frame-space-small);"));
}

#[test]
fn row_emits_flex_row_layout() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "NavBar",
            vec![
                statement(&["gap", "large"]),
                statement(&["align", "center"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: flex;"));
    assert!(css.contains("flex-direction: row;"));
    assert!(css.contains("gap: var(--frame-space-large);"));
    assert!(css.contains("align-items: center;"));
}

#[test]
fn grid_emits_css_grid_layout() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "DashboardGrid",
            vec![
                statement(&["columns", "2fr", "1fr"]),
                statement(&["gap", "medium"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: grid;"));
    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);"));
    assert!(css.contains("gap: var(--frame-space-medium);"));
}

#[test]
fn card_emits_flex_column_with_surface() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Card,
            "MetricCard",
            vec![
                statement(&["padding", "medium"]),
                statement(&["surface", "raised"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: flex;"));
    assert!(css.contains("flex-direction: column;"));
    assert!(css.contains("padding: var(--frame-space-medium);"));
    assert!(css.contains("background: var(--frame-surface-raised);"));
}

#[test]
fn text_declaration_emits_common_properties() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Text,
            "MutedText",
            vec![
                statement(&["color", "text-muted"]),
                statement(&["size", "caption"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("color: var(--frame-color-text-muted);"));
    assert!(css.contains("font-size: 0.875rem;"));
}

#[test]
fn button_declaration_emits_common_properties() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Button,
            "PrimaryButton",
            vec![
                statement(&["background", "accent"]),
                statement(&["radius", "medium"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("background: var(--frame-color-accent);"));
    assert!(css.contains("border-radius: var(--frame-radius-medium);"));
}

#[test]
fn generated_css_includes_frame_text_default() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains(".fr-FrameText"));
    assert!(css.contains("display: inline;"));
    assert!(css.contains("white-space: pre-wrap;"));
}

#[test]
fn generated_css_includes_button_reset() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("appearance: none;"));
    assert!(css.contains("cursor: pointer;"));
    assert!(css.contains("button[class*=\"fr-\"]"));
    assert!(css.contains("flex-direction: row !important;"));
}

#[test]
fn extends_inheritance_preserves_base_properties() {
    let base = declaration(
        DeclarationKind::Stack,
        "NavGroupBase",
        vec![
            statement(&["gap", "small"]),
            statement(&["color", "text-secondary"]),
        ],
    );
    let child = Declaration {
        kind: DeclarationKind::Stack,
        name: Identifier::new("NavGroupDash", Span::default()),
        extends: Some(Identifier::new("NavGroupBase", Span::default())),
        body: vec![],
        span: Span::default(),
    };

    let document = Document {
        includes: Vec::new(),
        declarations: vec![base, child],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("gap: var(--frame-space-small);"));
    assert!(css.contains("color: var(--frame-color-text-secondary);"));
    // Both base and child should have the inherited properties
    let nav_group_base_pos = css.find(".fr-NavGroupBase").unwrap();
    let nav_group_dash_pos = css.find(".fr-NavGroupDash").unwrap();
    assert!(nav_group_dash_pos > nav_group_base_pos);
}

#[test]
fn emits_chart_height_for_content_sizing() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "ChartBars",
            vec![statement(&["height", "chart"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("height: 12rem;"));
    assert!(!css.contains("height: var(--frame-space-chart);"));
}

#[test]
fn emits_min_height_chart() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "ChartPanel",
            vec![statement(&["min-height", "chart"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("min-height: 12rem;"));
}

#[test]
fn emits_min_height_none_as_zero() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "FlexRow",
            vec![statement(&["min-height", "none"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("min-height: 0;"));
}

#[test]
fn emits_panel_height() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "SidePanel",
            vec![statement(&["height", "panel"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("height: 16rem;"));
}

#[test]
fn invalid_height_token_falls_back_to_spacing_variable() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "TestRow",
            vec![statement(&["height", "bogus"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("height: var(--frame-space-bogus);"));
}

#[test]
fn fractional_columns_emit_gap_safe_tracks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "PerformanceGrid",
            vec![
                statement(&["columns", "3fr", "2fr"]),
                statement(&["gap", "medium"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);"));
    assert!(!css.contains("60%"));
    assert!(!css.contains("40%"));
}

#[test]
fn percentage_columns_do_not_get_wrapped_in_minmax() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "PercentGrid",
            vec![statement(&["columns", "60%", "40%"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: 60% 40%;"));
}

#[test]
fn dashboard_grid_produces_gap_safe_tracks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "DashboardGrid",
            vec![
                statement(&["columns", "2fr", "1fr"]),
                statement(&["gap", "medium"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);"));
}

#[test]
fn action_declaration_emits_button_class() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Button,
            "PrimaryButton",
            vec![
                statement(&["surface", "panel"]),
                statement(&["color", "white"]),
                statement(&["radius", "medium"]),
                Node::Block(frame_core::Block {
                    name: "hover".to_string(),
                    body: vec![
                        statement(&["lift", "small"]),
                        statement(&["glow", "accent"]),
                    ],
                    span: Span::default(),
                }),
                Node::Block(frame_core::Block {
                    name: "active".to_string(),
                    body: vec![statement(&["press"])],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains(".fr-PrimaryButton"));
    assert!(css.contains("background: var(--frame-surface-panel);"));
    assert!(css.contains("color: var(--frame-color-white);"));
    assert!(css.contains(".fr-PrimaryButton:hover"));
    assert!(css.contains("transform: translateY(-4px);"));
    assert!(css.contains("box-shadow: var(--frame-glow-accent);"));
    assert!(css.contains(".fr-PrimaryButton:active"));
    assert!(css.contains("transform: translateY(1px);"));
}

#[test]
fn row_with_columns_emits_grid_display() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "RunRowBase",
            vec![
                statement(&["columns", "2fr", "1fr", "1fr", "1fr", "1fr", "1fr"]),
                statement(&["align", "center"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);
    eprintln!("GENERATED CSS:\n{css}");

    assert!(css.contains("display: grid;"));
    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr);"));
    assert!(!css.contains("flex-direction: row;"));
    assert!(css.contains("align-items: center;"));
}

#[test]
fn row_without_columns_emits_flex_display() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "NavBar",
            vec![
                statement(&["gap", "large"]),
                statement(&["align", "center"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: flex;"));
    assert!(css.contains("flex-direction: row;"));
    assert!(!css.contains("grid-template-columns:"));
}

#[test]
fn row_columns_support_fr_tracks() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "ModelRowBase",
            vec![statement(&["columns", "auto", "1fr", "auto"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(css.contains("display: grid;"));
    assert!(css.contains("grid-template-columns: auto minmax(0, 1fr) auto;"));
}

#[test]
fn inherited_row_columns_share_template() {
    let base = declaration(
        DeclarationKind::Row,
        "TableRowBase",
        vec![
            statement(&["columns", "2fr", "1fr", "1fr"]),
            statement(&["align", "center"]),
        ],
    );
    let child = Declaration {
        kind: DeclarationKind::Row,
        name: Identifier::new("RunRow1", Span::default()),
        extends: Some(Identifier::new("TableRowBase", Span::default())),
        body: vec![],
        span: Span::default(),
    };

    let document = Document {
        includes: Vec::new(),
        declarations: vec![base, child],
        components: Vec::new(),
    };

    let css = generate_css(&document);
    eprintln!("INHERITED CSS:\n{css}");

    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) minmax(0, 1fr);"));
    let base_pos = css.find(".fr-TableRowBase").unwrap();
    let child_pos = css.find(".fr-RunRow1").unwrap();
    assert!(child_pos > base_pos);
}

// ===== Visual / Layout Regression Tests =====
// These tests catch the specific visual issues identified in the dashboard review.

#[test]
fn chart_bars_emit_real_chart_height_not_spacing() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "ChartBars",
            vec![statement(&["height", "chart"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // Must emit a real content height, not a spacing variable
    assert!(css.contains("height: 12rem;"));
    assert!(
        !css.contains("var(--frame-space-chart)"),
        "chart height must not use spacing token"
    );
    assert!(
        !css.contains("var(--frame-space-large)"),
        "chart height must not fall back to spacing large"
    );
}

#[test]
fn performance_grid_does_not_emit_percentage_columns_with_gap() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Grid,
            "PerformanceGrid",
            vec![
                statement(&["columns", "3fr", "2fr"]),
                statement(&["gap", "medium"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // Must use gap-safe fr tracks, not percentages
    assert!(css.contains("grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);"));
    assert!(
        !css.contains("60%"),
        "must not use percentage columns that overflow with gaps"
    );
    assert!(
        !css.contains("40%"),
        "must not use percentage columns that overflow with gaps"
    );
}

#[test]
fn run_table_rows_emit_grid_columns() {
    let base = declaration(
        DeclarationKind::Row,
        "TableRowBase",
        vec![
            statement(&["columns", "2fr", "1fr", "1fr", "1fr", "1fr", "1fr"]),
            statement(&["align", "center"]),
        ],
    );
    let header = declaration(
        DeclarationKind::Row,
        "RunColHeaders",
        vec![statement(&[
            "columns", "2fr", "1fr", "1fr", "1fr", "1fr", "1fr",
        ])],
    );
    let child = Declaration {
        kind: DeclarationKind::Row,
        name: Identifier::new("RunRow1", Span::default()),
        extends: Some(Identifier::new("TableRowBase", Span::default())),
        body: vec![],
        span: Span::default(),
    };

    let document = Document {
        includes: Vec::new(),
        declarations: vec![base, header, child],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // All table rows must share the same grid column template
    assert!(css.contains(".fr-TableRowBase"));
    assert!(css.contains("display: grid;"));
    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr);"));

    // Inherited rows must also be grid
    let run_row_pos = css.find(".fr-RunRow1").unwrap();
    assert!(css[run_row_pos..].contains("display: grid;"));
}

#[test]
fn primary_button_emits_background_and_hover_state() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Button,
            "PrimaryButton",
            vec![
                statement(&["surface", "panel"]),
                statement(&["radius", "medium"]),
                Node::Block(frame_core::Block {
                    name: "hover".to_string(),
                    body: vec![
                        statement(&["lift", "small"]),
                        statement(&["glow", "accent"]),
                    ],
                    span: Span::default(),
                }),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // Must have background and hover state
    assert!(css.contains(".fr-PrimaryButton"));
    assert!(css.contains("background: var(--frame-surface-panel);"));
    assert!(css.contains("border-radius: var(--frame-radius-medium);"));
    assert!(css.contains(".fr-PrimaryButton:hover"));
    assert!(css.contains("transform: translateY(-4px);"));
    assert!(css.contains("box-shadow: var(--frame-glow-accent);"));
}

#[test]
fn tabs_are_emitted_as_button_action_nodes() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![
            declaration(
                DeclarationKind::Button,
                "TabAll",
                vec![
                    statement(&["surface", "overlay"]),
                    statement(&["color", "text-primary"]),
                    statement(&["radius", "small"]),
                ],
            ),
            declaration(
                DeclarationKind::Button,
                "TabLocal",
                vec![
                    statement(&["color", "text-secondary"]),
                    statement(&["radius", "small"]),
                ],
            ),
        ],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // Tabs must be emitted as button/action classes with styling
    assert!(css.contains(".fr-TabAll"));
    assert!(css.contains("background: var(--frame-surface-overlay);"));
    assert!(css.contains(".fr-TabLocal"));
    assert!(css.contains("color: var(--frame-color-text-secondary);"));
}

#[test]
fn dashboard_header_logo_text_is_visible() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Text,
            "LLMOps",
            vec![
                statement(&["color", "accent"]),
                statement(&["weight", "bold"]),
                statement(&["size", "heading"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    // Logo text must have explicit size and weight
    assert!(css.contains(".fr-LLMOps"));
    assert!(css.contains("color: var(--frame-color-accent);"));
    assert!(css.contains("font-weight: 700;"));
    assert!(css.contains("font-size: 2rem;"));
}

// ===== Sizing Token Regression Tests =====
// These tests verify that new max-width tokens produce correct CSS values.

#[test]
fn max_width_input_token_emits_32rem() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Row,
            "SearchBar",
            vec![statement(&["max-width", "input"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(
        css.contains("max-width: 32rem;"),
        "input token must emit 32rem, got: {css}"
    );
}

#[test]
fn max_width_wide_token_emits_32rem() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "WidePanel",
            vec![statement(&["max-width", "wide"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(
        css.contains("max-width: 32rem;"),
        "wide token must emit 32rem, got: {css}"
    );
}

#[test]
fn max_width_dashboard_token_emits_96rem() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "DashboardContent",
            vec![statement(&["max-width", "dashboard"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(
        css.contains("max-width: 96rem;"),
        "dashboard token must emit 96rem, got: {css}"
    );
}

#[test]
fn width_fill_emits_100_percent() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "FullWidth",
            vec![statement(&["width", "fill"])],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(
        css.contains("width: 100%;"),
        "width fill must emit 100%, got: {css}"
    );
}

#[test]
fn dashboard_content_does_not_emit_narrow_max_width() {
    let document = Document {
        includes: Vec::new(),
        declarations: vec![declaration(
            DeclarationKind::Stack,
            "DashboardContent",
            vec![
                statement(&["gap", "large"]),
                statement(&["width", "fill"]),
                statement(&["max-width", "dashboard"]),
            ],
        )],
        components: Vec::new(),
    };

    let css = generate_css(&document);

    assert!(
        !css.contains("max-width: 32rem;"),
        "DashboardContent must not emit narrow 32rem max-width, got: {css}"
    );
    assert!(
        css.contains("width: 100%;"),
        "DashboardContent must emit width fill (100%), got: {css}"
    );
    assert!(
        css.contains("max-width: 96rem;"),
        "DashboardContent must emit dashboard max-width (96rem), got: {css}"
    );
}
