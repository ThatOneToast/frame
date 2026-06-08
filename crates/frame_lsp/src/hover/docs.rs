pub const SURFACE_PANEL_DOC: &str = r#"surface panel

A panel surface is for secondary UI areas like sidebars, inspectors, cards, and tool panels.
It usually uses a slightly raised or separated background color.

Generated CSS: `background: var(--frame-surface-panel);`

Use it for:
- sidebars
- right panels
- cards
- menu surfaces

Svelte example:

<aside class="fr-Sidebar">
  Channels
</aside>

<style lang="frame">
  area Sidebar {
    in Dashboard
    place sidebar
    surface panel
    padding medium
  }
</style>"#;

pub const SURFACE_MAIN_DOC: &str = r#"surface main

The main surface is for the primary page/content background.
Use it for the main content region, large pages, and app shells.

Generated CSS: `background: var(--frame-surface-main);`

Svelte example:

<main class="fr-Content">
  Main content
</main>

<style lang="frame">
  area Content {
    in Dashboard
    place content
    surface main
    padding large
  }
</style>"#;

pub const SURFACE_GLASS_DOC: &str = "surface glass\n\nA translucent surface for overlays, floating panels, and command palettes.\nGenerated CSS uses `background: var(--frame-surface-glass);`.";
pub const WIDTH_PERCENT_DOC: &str = "width 25%\n\nMakes this item take a percentage of the available width.\nUseful for sidebars and split layouts.\nGenerated CSS writes values like `width: 25%;` or `height: 50%;`.";

pub const INCLUDE_DOC: &str = r#"#include

Includes another Frame file before the current declarations.

Use it to split large style systems into focused files such as `tokens.frame`, `layout.frame`, and `cards.frame`.

Frame:

#include tokens
#include ./styles/cards.frame

card LocalCard {
  surface panel
  padding medium
}

CLI:

frame compile src/lib/frame/app.frame --out src/lib/frame --include src/lib/frame

Docs: `docs/imports.md`"#;
