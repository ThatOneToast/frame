use docs::*;
use frame_core::{knowledge, symbols::SymbolIndex};
use helpers::line_at;
use values::contextual_value_doc;

mod docs;
mod helpers;
mod values;

pub use helpers::word_at;

#[allow(dead_code)]
pub fn hover_doc_at(source: &str, offset: usize) -> Option<String> {
    hover_doc_at_with_symbols(source, offset, None)
}

pub fn hover_doc_at_with_symbols(
    source: &str,
    offset: usize,
    symbols: Option<&SymbolIndex>,
) -> Option<String> {
    let word = word_at(source, offset)?;
    let line = line_at(source, offset);
    let words = line.split_whitespace().collect::<Vec<_>>();

    if line.starts_with("#include") {
        return Some(INCLUDE_DOC.to_string());
    }

    match words.as_slice() {
        ["surface", "panel"] if word == "panel" || word == "surface" => {
            return Some(doc_for("surface panel", SURFACE_PANEL_DOC));
        }
        ["surface", "main"] if word == "main" || word == "surface" => {
            return Some(doc_for("surface main", SURFACE_MAIN_DOC));
        }
        ["surface", "glass"] if word == "glass" || word == "surface" => {
            return Some(doc_for("surface glass", SURFACE_GLASS_DOC));
        }
        ["surface", "gradient", ..] if word == "gradient" => {
            return Some(SURFACE_GRADIENT_DOC.to_string())
        }
        ["width" | "height", value] if value.ends_with('%') => {
            return Some(doc_for("width 25%", WIDTH_PERCENT_DOC))
        }
        _ => {}
    }

    if let Some(symbols) = symbols {
        if let Some(color) = symbols.colors.get(word) {
            return Some(format!(
                "## `{}`\n\nCustom color token.\n\nValue:\n\n```css\n{}\n```\n\nUse it anywhere Frame accepts color intent, including `background`, `color`, `border`, `glow`, and `ring`.\n\n### Frame\n\n```frame\ncard BrandCard {{\n  background {}\n  color {}\n}}\n```",
                color.name,
                color.value.as_deref().unwrap_or("custom color"),
                color.name,
                color.name
            ));
        }

        if let Some(gradient) = symbols.gradients.get(word) {
            return Some(format!(
                "## `{}`\n\nCustom gradient token.\n\nGenerated behavior:\n\n```css\n{}\n```\n\nUse it for hero cards, highlighted dashboard cards, panels, and sign-in screens.\n\n### Frame\n\n```frame\ncard HeroCard {{\n  background {}\n  color white\n}}\n```",
                gradient.name,
                gradient.value.as_deref().unwrap_or("linear-gradient(...)"),
                gradient.name
            ));
        }

        if let Some(keyframes) = symbols.keyframes.get(word) {
            return Some(format!(
                "## `{}`\n\nCustom keyframes animation.\n\nGenerated CSS:\n\n```css\n{}\n```\n\nUse it with a structured animation block:\n\n```frame\ncard Panel {{\n  animation {} {{\n    duration 240ms\n    ease smooth\n    fill both\n  }}\n}}\n```",
                keyframes.name,
                keyframes
                    .value
                    .as_deref()
                    .unwrap_or("@keyframes frame-Name"),
                keyframes.name
            ));
        }
    }

    if let Some(doc) = contextual_value_doc(word, &words, symbols) {
        return Some(doc);
    }

    hover_doc(word)
}

pub fn hover_doc(word: &str) -> Option<String> {
    if let Some(doc) = native_hover_doc(word) {
        return Some(doc.to_string());
    }

    if let Some(doc) = knowledge::completion_doc(word) {
        return Some(doc);
    }

    Some(match word {
        "slot" => "Defines a named content region inside a component.\nSlots allow parent components to inject content. The default slot is named `Default`.\n\nExample:\n\nslot Default {\n  text \"Fallback content\"\n}",
        "for" => "Starts renderer-neutral list rendering.\n\nUse `for item in $items { ... }` for positional lists or `for item in $items key $id { ... }` when stable identity is available. The compiler lowers this to Frame IR list metadata; renderers decide how to realize updates.",
        "key" => "Declares stable identity for a Frame list.\n\nKeyed lists allow renderers to reuse item instances by identity. Non-keyed lists use positional update behavior.",
        "on" => "Binds a UI event to an external handler reference.\nFrame stores `@handlerName`, not inline JavaScript or TypeScript bodies.",
        "bind" => "`bind $state`, `draft bind $state`, and similar forms record two-way state binding intent without exposing browser form controls.",
        "when" => "Introduces a state-driven condition such as `disabled when $sending` or `style when $sending = LoadingButton`.",
        "value" => "Stores a visible value for content primitives or a value binding target for input-like primitives.",
        "goto" => "Declares navigation intent for `link`. Renderers validate the destination and lower it to their platform.",
        "source" => "Declares data or media source intent. Lists use `source $items`; images use `source $image`.",
        "send" => "References the external handler that sends or submits a composer-like primitive.",
        "draft" => "Binds composer draft text to component state.",
        "selected" => "`selected bind $state` connects select-like controls to typed component state.",
        "style" => "`style when $state = StyleName` records conditional style switching for a UI node.",
        "show" => "`show when $state` records conditional rendering intent. The runtime tracks the dependency; renderers decide whether to create, skip, or serialize the node.",
        "hidden" => "`hidden when $state` records visibility intent without requiring a specific renderer mechanism.",
        "alt" => "Provides alternate text for image-like primitives. Prefer Frame image semantics; renderers generate accessibility metadata.",
        "description" => "Adds descriptive accessibility metadata without manual ARIA wiring.",
        "hint" => "Adds helper text for form-like primitives without manual ARIA wiring.",
        "decorative" => "Marks image-like content as visual-only when set to `true`.",
        "poster" => "Media destination. Renderers validate URL-like values before writing platform sinks.",
        "aria-label" | "aria-labelledby" | "aria-hidden" | "role" | "href" | "src" | "srcset" | "rel" => "Browser-centric syntax. Frame authoring should use semantic properties such as `label`, `description`, `decorative`, `goto`, and `source`.",
        "data-test-id" => "Example `data-*` attribute. Frame preserves `data-*` attributes for application metadata and tests.",
        "html" | "innerHTML" | "outerHTML" => "Unsafe HTML injection sink. Frame text escapes by default; raw HTML requires an explicit unsafe capability before renderer consumption.",
        "$value" => "$value reads typed component state or props. Text insertion is escaped by default in future renderers.",
        "@handler" => "@handler references an external handler. Frame does not store script bodies inside UI declarations.",
        "tokens" => "Defines reusable design tokens for a Frame file.\nUse tokens to name shared visual decisions before applying them to components.",
        "area" => "Defines a child region inside a named grid.\nUse `in` to reference the parent grid and `place` to claim a named grid column or area.\n\nExample:\n\narea Sidebar {\n  in AppShell\n  place sidebar\n}",
        "text" => "Defines reusable typography intent.\nUse size, weight, font, and color tokens instead of raw font CSS.",
        "center" => "Defines a container that centers its content.\nUse it for empty states, loading states, and focused prompts.",
        "split" => SPLIT_DOC,
        "overlay" => OVERLAY_DOC,
        "dock" => DOCK_DOC,
        "keyframes" => KEYFRAMES_DOC,
        "columns" => COLUMNS_DOC,
        "rows" => ROWS_DOC,
        "tracks" => "Defines readable grid track sizes for app shells.\nUse `tracks columns rail panel fill side` or `tracks rows header fill composer` instead of raw grid-template CSS.",
        "areas" => "Defines one row of a named grid area template.\nRepeat `areas ...` lines to build multi-row app shells without raw `grid-template-areas`.",
        "gap" => "Sets spacing between children using Frame spacing tokens like small, medium, and large.",
        "display" => DISPLAY_DOC,
        "layout" => "Applies a dense internal layout preset for repeated app patterns such as icon/content/action rows and avatar/content message rows.",
        "place" => PLACE_DOC,
        "in" => "References the parent grid for an area.\nExample: `in AppShell`.",
        "col" => COL_DOC,
        "span" => "Makes an area span multiple grid tracks.\nUse it for headers, footers, or wide content regions.",
        "surface" => "Sets the visual surface of a component.\nUse named surfaces like `panel`, `main`, `glass`, or gradients like `gradient dusk`.\n\nExample:\n\nsurface gradient dusk",
        "theme" => "Applies semantic color intent such as danger, success, or warning.",
        "background" => "Sets background intent using Frame surface or color tokens.",
        "gradient" => "Selects a named gradient surface such as dusk, midnight, or aurora.",
        "padding" => "Adds inner spacing using Frame spacing tokens.",
        "margin" => "Adds outer spacing using Frame spacing tokens.",
        "radius" => "Sets corner shape with named values like small, large, pill, or none.",
        "border" => "Sets border intent.\n\nUse semantic border colors such as `border accent`, thickness with `border width medium`, line styles with `border style dashed`, or `border none`.\n\nGenerated CSS writes border color, width, radius, or style rules.",
        "outline" => "Sets outline intent.\n\nUse `outline none`, a semantic color such as `outline accent`, or `outline offset small`.\n\nGenerated CSS writes `outline` or `outline-offset`.",
        "overflow" => "Controls overflow intent for panels and app shells. Use `overflow hidden` for clipped regions.",
        "scroll" => "Makes a region scroll on an axis. Use `scroll y` for channel panels, message lists, and member lists.",
        "scrollbar" => "Sets scrollbar density for app panels. Use `scrollbar dense` for compact terminal-inspired surfaces.",
        "box" => "Sets box sizing intent. Use `box border` for app surfaces where borders should be included in dimensions.",
        "visibility" => "Sets CSS visibility through structured values.\n\nUse `visibility hidden` when the element should keep its layout slot but not render visibly.",
        "flex" => FLEX_DOC,
        "square" => "Applies a named equal width and height for icons, avatars, server buttons, and presence dots.",
        "shadow" => "Sets depth using named shadow values like soft, medium, or deep.",
        "transition" => TRANSITION_DOC,
        "duration" => "Sets motion duration intent. Use `fast`, `normal`, or `slow` with transitions and animations.",
        "ease" => "Sets easing intent. Use `linear`, `smooth`, `bounce`, or `sharp` to describe motion feel.",
        "animation" | "animate" => ANIMATION_DOC,
        "delay" => "Sets the delay before an animation starts. Use named timing or CSS time values like `120ms`.",
        "iteration" => "Sets animation repeat count. Use a number or `infinite`.",
        "direction" => "Sets animation playback direction such as `normal`, `reverse`, or `alternate`.",
        "play-state" => "Controls whether an animation is `running` or `paused`.",
        "below" => RESPONSIVE_DOC,
        "above" => RESPONSIVE_DOC,
        "between" => RESPONSIVE_DOC,
        "container" => CONTAINER_DOC,
        "supports" => "Starts a typed feature query block.\n\nUse predicates like `supports display grid`, `supports backdrop blur`, `supports color oklch`, `supports selector has`, `supports container queries`, or `supports subgrid`.\n\nGenerated CSS emits an `@supports` rule.",
        "style-group" => "Starts a named style group.\n\nStyle groups map to CSS cascade layers while keeping Frame syntax intent-focused.\n\nExample:\n\nstyle-group components {\n  button PrimaryButton {\n    surface accent\n  }\n}",
        "style-order" => "Declares deterministic style group order.\n\nExample:\n\nstyle-order reset, base, components, utilities\n\nGenerated CSS emits a cascade layer order rule.",
        "from" | "to" => KEYFRAME_SELECTOR_DOC,
        "opacity" => "Animates opacity in keyframes.\n\nGenerated CSS writes `opacity: ...`.",
        "transform" => "Animates transform functions in keyframes.\n\nGenerated CSS writes `transform: ...`.",
        "height" => "Sets height intent with values such as screen, fill, content, or percentages.\nGenerated CSS writes `height`, with `screen` becoming `100vh`.",
        "width" => "Sets width intent with values such as fill, content, screen, sidebar, or percentages.\nGenerated CSS writes `width`.",
        "inline-size" => "Sets logical inline size. In horizontal writing modes this usually behaves like width.\nGenerated CSS writes `inline-size`.",
        "block-size" => "Sets logical block size. In horizontal writing modes this usually behaves like height.\nGenerated CSS writes `block-size`.",
        "min-inline-size" => "Sets minimum logical inline size.\nGenerated CSS writes `min-inline-size`.",
        "max-inline-size" => "Sets maximum logical inline size.\nGenerated CSS writes `max-inline-size`.",
        "min-block-size" => "Sets minimum logical block size.\nGenerated CSS writes `min-block-size`.",
        "max-block-size" => "Sets maximum logical block size.\nGenerated CSS writes `max-block-size`.",
        "min-width" => "Sets minimum width intent using named sizes or percentages.",
        "max-width" => "Sets maximum width intent using named sizes or percentages.",
        "min-height" => "Sets minimum height intent using named sizes or percentages.",
        "max-height" => "Sets maximum height intent using named sizes or percentages.",
        "align" => ALIGN_DOC,
        "justify" => JUSTIFY_DOC,
        "self" => "Aligns this item within its parent in both axes. Use `self center` for centered modal panels.",
        "nudge" => "Applies a small positional adjustment for badges and status dots.",
        "truncate" => "Keeps text on one line and adds ellipsis when it overflows. Use it for dense labels in sidebars and headers.",
        "wrap" => "Controls text wrapping. Use `wrap anywhere` for chat message bodies and narrow content.",
        "case" => "Controls text casing intent. Use `case uppercase` for compact section labels.",
        "align-text" => "Aligns text inside controls and rows. Use `align-text left` for dense navigation buttons.",
        "decoration" => "Sets text decoration line intent.\n\nUse `decoration underline`, `decoration overline`, `decoration line-through`, or `decoration none`.\n\nGenerated CSS writes `text-decoration-line`.",
        "whitespace" => "Controls white-space preservation and wrapping.\n\nUse `whitespace pre-wrap` for user-entered multiline text or `whitespace break-spaces` when spaces should be preserved.\n\nGenerated CSS writes `white-space`.",
        "word-break" => "Controls how words break in narrow layouts.\n\nUse `word-break break-word` for long unspaced content.\n\nGenerated CSS writes `word-break`.",
        "hyphenate" => "Controls hyphenation behavior.\n\nUse `hyphenate auto` for prose where browser hyphenation is acceptable.\n\nGenerated CSS writes `hyphens`.",
        "control" => "Applies control affordance intent. Use `control reset` to remove browser-specific button or input appearance.",
        "interactive" => "Marks a surface as pointer-interactive and emits cursor affordance.",
        "hover" => "Defines effects applied when the user hovers this component.\n\nExample:\n\nhover {\n  lift small\n  glow accent\n}",
        "focus" => "Defines effects applied when keyboard or assistive focus reaches this component.",
        "focus-visible" => "Defines effects applied when focus should be visibly indicated.\n\nGenerated CSS emits `:focus-visible`.",
        "focus-within" => "Defines effects applied when this element or any descendant has focus.\n\nGenerated CSS emits `:focus-within`.",
        "active" => "Defines effects applied while this component is being pressed.",
        "disabled" => "Defines visual treatment for unavailable controls.",
        "checked" => "Defines effects applied to checked controls.\n\nGenerated CSS emits `:checked`.",
        "invalid" => "Defines effects applied to invalid form controls.\n\nGenerated CSS emits `:invalid`.",
        "required" => "Defines effects applied to required form controls.\n\nGenerated CSS emits `:required`.",
        "target" => "In UI attributes, controls where a link or form opens. Use `rel \"noopener\"` or `rel \"noreferrer\"` with `target \"_blank\"`.\n\nIn style declarations, defines effects applied when this element matches the URL fragment target and emits `:target`.",
        "lift" => "Moves a component upward to express elevation.\n\nUse movement amounts `tiny`, `small`, `medium`, `large`, or `huge`. Add `%0` through `%100` to tune toward the next stronger amount, for example `lift small%44`.\n\nGenerated CSS composes this into `transform: translateY(...)`.",
        "sink" => "Moves a component downward.\n\nUse movement amounts like `small` or tuned values like `small%44`. Generated CSS composes this into `transform: translateY(...)`.",
        "shift" => "Moves a component in a direction.\n\nUse `shift left small`, `shift right small`, `shift up small`, or `shift down small`. Movement amounts can use percent tuning.",
        "grow" => "Scales a component up by intent.\n\nUse visual amounts `slight`, `subtle`, `normal`, `strong`, or `dramatic`. Add `%0` through `%100` for fine tuning, for example `grow slight%5`.",
        "shrink" => "Scales a component down by intent using visual amounts and optional percent tuning.",
        "tilt" => "Rotates a component by intent.\n\nUse `tilt left subtle` or `tilt right subtle`. Visual amounts can be tuned with suffix percentages, for example `tilt right subtle%23`.",
        "glow" => "Adds a semantic glow, commonly using accent, danger, or success.",
        "brighten" => "Slightly increases visual brightness for interactive feedback.",
        "dim" => "Reduces visual emphasis for disabled or inactive states.",
        "blur" => "Applies blur intent, usually for overlays or state effects.",
        "press" => "Adds a pressed movement for active controls.",
        "pop" => "Adds a small positive scale movement for appearing or selected states.",
        "ring" => "Adds an accessible focus ring using a semantic color.",
        "smooth" => "Expresses smooth transition intent for interaction effects.",
        "responsive" => "Requests viewport-aware behavior, such as responsive card grids.",
        "cards" => "Used with `columns responsive` to create an auto-fitting card grid.",
        "fill" => "Sizes an element to fill available space. Inside an `animation` block, `fill` sets animation fill mode such as `both`.",
        "panel" => SURFACE_PANEL_DOC,
        "main" => SURFACE_MAIN_DOC,
        "glass" => SURFACE_GLASS_DOC,
        "danger" => "Semantic color intent for destructive actions, errors, and dangerous status.\nUse it for delete buttons, invalid states, and error badges.",
        "success" => "Semantic color intent for successful or positive states.\nUse it for completed tasks, saved states, and positive status.",
        "warning" => "Semantic color intent for cautionary states.\nUse it for warnings, pending work, and attention states.",
        "accent" => "Use accent for important interactive UI:\n- primary buttons\n- active nav items\n- focus rings\n- highlighted cards",
        "muted" => "Semantic color intent for secondary text or subdued UI.\nUse it for captions, helper text, and lower-priority metadata.",
        "primary" => "Primary color intent for the most important interactive elements and highlighted content.",
        "secondary" => "Secondary color intent for supporting actions and secondary emphasis.",
        "info" => "Informational color intent for neutral notices, tips, and status messages.",
        "font" => "Selects a typography family intent such as mono.",
        "size" => "Selects a typography size intent such as heading, body, or caption.",
        "weight" => "Selects type emphasis such as normal, semibold, or bold.",
        _ if word.starts_with('$') => "$value reads typed component state or props. Text insertion is escaped by default in future renderers.",
        _ if word.starts_with('@') => "@handler references an external handler. Frame does not store script bodies inside UI declarations.",
        _ => return None,
    }.to_string())
}

fn native_hover_doc(word: &str) -> Option<&'static str> {
    Some(match word {
        "component" => "## `component`\n\nDefines a Frame UI component.\n\nUse it for reusable interface units with typed inputs, local state, and a semantic view tree. Components may contain `props`, `state`, `view`, and `slot` blocks.\n\nProduces compiler AST, Frame IR component metadata, TypeScript contracts, and runtime mount targets.\n\n```frame\ncomponent Counter {\n  state { count number = 0 }\n  view { action Increment { on press @increment } }\n}\n```",
        "props" => "## `props`\n\nDeclares typed inputs accepted by a component.\n\nUse props for data supplied by a parent component. Props are read by `$name` references and lower to IR prop descriptors plus TypeScript prop contracts.\n\n```frame\nprops {\n  title text\n  selected bool\n}\n```",
        "state" => "## `state`\n\nDeclares local mutable component data.\n\nUse state for values changed by handlers or bindings. State lowers to IR state descriptors with serialized defaults and runtime state slots.\n\n```frame\nstate {\n  draft text = \"\"\n  sending bool = false\n}\n```",
        "view" => "## `view`\n\nDeclares the component UI tree with Frame primitives.\n\nUse semantic primitives such as `screen`, `panel`, `stack`, `field`, `input`, `list`, and `action`. Renderers lower that intent to their target platform.\n\n```frame\nview {\n  stack Content {\n    text $title\n    action Save { on press @save }\n  }\n}\n```",
        "style" => "## `style`\n\nApplies state-driven style switching inside a UI node.\n\nUse it when a node should gain a style class while state is true. It lowers to an IR conditional style and the DOM runtime patches classes.\n\n```frame\naction Send:PrimaryAction {\n  style SendingAction when $sending\n}\n```",
        "style-group" => "## `style-group`\n\nGroups style declarations into named cascade layers.\n\nUse it to keep Frame styles ordered without writing raw CSS layers. Generated CSS emits the corresponding layer wrapper.\n\n```frame\nstyle-group components {\n  stack ComposerShell { gap tight }\n}\n```",
        "screen" => "## `screen`\n\nRepresents a full UI surface.\n\nUse it as a view root for pages, tools, and app screens. It lowers to renderer root/container metadata and defaults to a DOM container in the DOM runtime.\n\n```frame\nscreen AppScreen:AppShell {\n  stack Content { text \"Hello\" }\n}\n```",
        "panel" => "## `panel`\n\nRepresents a named region of interface content.\n\nUse it for sidebars, panes, inspectors, and grouped app regions. It lowers to a neutral container while preserving semantic region intent.\n\n```frame\npanel Sidebar {\n  title \"Channels\"\n}\n```",
        "stack" => "## `stack`\n\nRepresents ordered one-direction layout.\n\nUse it for vertical or grouped content flow without naming flexbox. Style declarations define spacing and alignment.\n\n```frame\nstack MessageBody {\n  text $author\n  text $body\n}\n```",
        "row" => "## `row`\n\nRepresents horizontal grouping intent.\n\nUse it for toolbar rows, message rows, and compact control groups. Renderers choose the target layout implementation.\n\n```frame\nrow MessageRow {\n  avatar AuthorAvatar { source $avatar alt $author }\n  stack Body { text $body }\n}\n```",
        "grid" => "## `grid`\n\nRepresents two-dimensional layout intent.\n\nUse it for app shells and dashboards. It produces structured layout metadata and generated CSS for DOM targets.\n\n```frame\ngrid AppShell {\n  columns sidebar content\n  gap medium\n}\n```",
        "field" => "## `field`\n\nGroups a label, help text, validation state, and one control.\n\nUse it around `input`, `editor`, `toggle`, `choice`, or `select` so accessibility and layout stay semantic. It lowers to a neutral field container.\n\n```frame\nfield EmailField {\n  label \"Email\"\n  input EmailInput { value bind $email }\n}\n```",
        "input" => "## `input`\n\nRepresents single-value text entry.\n\nUse `value bind $state` to connect it to component state. It lowers to the renderer's text-input control.\n\n```frame\ninput MessageInput {\n  value bind $draft\n  placeholder \"Message\"\n}\n```",
        "editor" => "## `editor`\n\nRepresents multi-line text editing.\n\nUse it for comments, messages, and document text. It lowers to a multi-line editing control in DOM.\n\n```frame\neditor BodyEditor {\n  value bind $body\n  on keydown.ctrl.enter @save\n}\n```",
        "action" => "## `action`\n\nRepresents a user-triggered command.\n\nUse `on press @handler` for activation instead of browser event attributes. It lowers to an accessible action control.\n\n```frame\naction Send:PrimaryAction {\n  text \"Send\"\n  on press @sendMessage\n}\n```",
        "link" => "## `link`\n\nRepresents navigation intent.\n\nUse `goto` for the destination. Renderers validate the target and lower to platform navigation.\n\n```frame\nlink Docs {\n  goto \"/docs\"\n  text \"Docs\"\n}\n```",
        "menu" => "## `menu`\n\nRepresents navigation or command choices.\n\nUse it for app navigation and command groups. It lowers to renderer navigation/menu structures.\n\n```frame\nmenu MainNav {\n  link Docs { goto \"/docs\" text \"Docs\" }\n}\n```",
        "toolbar" => "## `toolbar`\n\nRepresents a compact group of related actions.\n\nUse it for editor commands and app chrome. It lowers to a command region with grouped actions.\n\n```frame\ntoolbar EditorTools {\n  action Save { on press @save }\n}\n```",
        "tabs" => "## `tabs`\n\nRepresents switching between related panels.\n\nUse it when the user selects one visible panel from a small set. It records tab intent for renderer accessibility behavior.\n\n```frame\ntabs SettingsTabs {\n  action General { on press @openGeneral }\n}\n```",
        "toggle" => "## `toggle`\n\nRepresents a binary setting.\n\nUse `checked bind $state` for two-way boolean state. It lowers to a checkable control.\n\n```frame\ntoggle CompactMode {\n  label \"Compact mode\"\n  checked bind $compact\n}\n```",
        "choice" => "## `choice`\n\nRepresents a small option choice.\n\nUse it for radio-like or segmented choices. Renderers decide the exact control shape.\n\n```frame\nchoice ThemeChoice {\n  selected bind $theme\n}\n```",
        "select" => "## `select`\n\nRepresents selection from a larger or dynamic option set.\n\nUse `selected bind $state` and `options $items`. It lowers to a selection control.\n\n```frame\nselect ChannelSelect {\n  selected bind $channel\n  options $channels\n}\n```",
        "composer" => "## `composer`\n\nRepresents input collection and submission intent.\n\nUse it for message composers, forms, and submit flows without writing browser form syntax.\n\n```frame\ncomposer MessageComposer {\n  draft bind $draft\n  send @sendMessage\n}\n```",
        "title" => "## `title`\n\nRepresents semantic title text.\n\nUse it for headings without choosing a browser heading level. Renderers choose the appropriate output.\n\n```frame\ntitle \"Settings\"\n```",
        "label" => "## `label`\n\nRepresents visible naming text or a control label.\n\nUse it inside fields and controls so renderers can preserve accessibility relationships.\n\n```frame\nlabel \"Email\"\n```",
        "badge" => "## `badge`\n\nRepresents compact status or metadata.\n\nUse it for counts, states, and short labels.\n\n```frame\nbadge Status { text \"New\" }\n```",
        "avatar" => "## `avatar`\n\nRepresents a person or entity image.\n\nUse `source` and `alt` unless the image is decorative. It lowers to image-like renderer output.\n\n```frame\navatar AuthorAvatar { source $avatar alt $author }\n```",
        "icon" => "## `icon`\n\nRepresents symbolic visual content.\n\nUse it for decorative or named symbols. Decorative icons lower with hidden accessibility metadata.\n\n```frame\nicon SearchIcon { label \"Search\" }\n```",
        "image" => "## `image`\n\nRepresents meaningful imagery.\n\nUse `source` or `sources` and `alt`. Renderers validate URL-like sinks.\n\n```frame\nimage Cover { source $cover alt \"Cover\" }\n```",
        "media" => "## `media`\n\nRepresents audio or video playback intent.\n\nUse `sources`, `poster`, and labels. The DOM runtime lowers it to media controls.\n\n```frame\nmedia Preview { sources $video poster $poster }\n```",
        "list" => "## `list`\n\nRepresents repeated content.\n\nUse `for item in $items key $item.id` for stable identity and `empty` for fallback content.\n\n```frame\nlist Messages {\n  for message in $messages key $message.id {\n    item Message { text $message.body }\n  }\n}\n```",
        "feed" => "## `feed`\n\nRepresents chronological or activity-stream content.\n\nUse it for messages, events, and updates where order matters.\n\n```frame\nfeed Activity {\n  for event in $events key $event.id { item Event { text $event.title } }\n}\n```",
        "data" => "## `data`\n\nRepresents structured records.\n\nUse it for rows and fields without exposing table syntax in Frame source.\n\n```frame\ndata Invoices {\n  for invoice in $invoices key $invoice.id { row Invoice { text $invoice.total } }\n}\n```",
        "item" => "## `item`\n\nRepresents one repeated collection entry.\n\nUse it inside `list`, `feed`, or `data` loops.\n\n```frame\nitem Message { text $message.body }\n```",
        "empty" => "## `empty`\n\nRepresents fallback content for an empty collection.\n\nUse it inside collection primitives.\n\n```frame\nempty NoMessages { text \"No messages yet\" }\n```",
        "card" => "## `card`\n\nRepresents grouped object or preview content.\n\nUse it for repeated records, summaries, and selectable objects. It is fully styleable.\n\n```frame\ncard ProjectCard:SelectableCard { text $project.name }\n```",
        "dialog" => "## `dialog`\n\nRepresents a modal or attention surface.\n\nUse `show when $state` and explicit close actions. Renderers handle focus and modal behavior as support matures.\n\n```frame\ndialog SettingsDialog { show when $open }\n```",
        "popover" => "## `popover`\n\nRepresents lightweight contextual content.\n\nUse it for small overlays tied to another interaction.\n\n```frame\npopover HelpPopover { text \"More detail\" }\n```",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_hover_docs_for_concepts() {
        let doc = hover_doc("grid").expect("grid should have docs");

        assert!(doc.contains("two-dimensional layout intent"));
        assert!(doc.contains("```frame"));

        assert!(hover_doc("component")
            .expect("component docs")
            .contains("Frame UI component"));
        assert!(hover_doc("field")
            .expect("field docs")
            .contains("Groups a label"));
        assert!(hover_doc("action")
            .expect("action docs")
            .contains("on press @handler"));
        assert!(hover_doc("$draft")
            .expect("data ref docs")
            .contains("escaped by default"));
        assert!(hover_doc("@sendMessage")
            .expect("handler docs")
            .contains("external handler"));
    }

    #[test]
    fn finds_word_at_offset() {
        let source = "card ProjectCard {\n  surface panel\n}\n";
        let offset = source.find("surface").unwrap() + 2;

        assert_eq!(word_at(source, offset), Some("surface"));
    }

    #[test]
    fn returns_surface_value_hover_docs() {
        let source = "area Sidebar {\n  surface panel\n}\n";
        let offset = source.find("panel").unwrap() + 1;
        let doc = hover_doc_at(source, offset).expect("panel should have docs");

        assert!(doc.contains("surface"));
        assert!(doc.contains("panel"));
    }

    #[test]
    fn returns_columns_and_alignment_hover_docs() {
        assert!(hover_doc("columns").unwrap().contains("25% 50% 25%"));
        assert!(hover_doc("align").unwrap().contains("cross-axis"));
        assert!(hover_doc("justify").unwrap().contains("main-axis"));
        assert!(hover_doc("display").unwrap().contains("display: ..."));
        assert!(hover_doc("flex").unwrap().contains("flex direction"));
        assert!(hover_doc("inline-size").unwrap().contains("logical inline"));
        assert!(hover_doc("decoration")
            .unwrap()
            .contains("text-decoration-line"));
        assert!(hover_doc("whitespace").unwrap().contains("white-space"));
        assert!(hover_doc("word-break").unwrap().contains("word-break"));
        assert!(hover_doc("hyphenate").unwrap().contains("hyphens"));
        assert!(hover_doc("focus-visible")
            .unwrap()
            .contains(":focus-visible"));
        assert!(hover_doc("focus-within").unwrap().contains(":focus-within"));
        assert!(hover_doc("checked").unwrap().contains(":checked"));
        assert!(hover_doc("invalid").unwrap().contains(":invalid"));
        assert!(hover_doc("required").unwrap().contains(":required"));
        assert!(hover_doc("target").unwrap().contains(":target"));
        assert!(hover_doc("lift").unwrap().contains("small%44"));
        assert!(hover_doc("tilt").unwrap().contains("subtle%23"));
        assert!(hover_doc("supports").unwrap().contains("@supports"));
        assert!(hover_doc("style-group").unwrap().contains("cascade layers"));
        assert!(hover_doc("style-order")
            .unwrap()
            .contains("style group order"));
    }

    #[test]
    fn returns_percentage_hover_docs() {
        let source = "card A {\n  width 25%\n}\n";
        let offset = source.find("25%").unwrap() + 1;

        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("available width"));
    }

    #[test]
    fn returns_contextual_value_hover_docs() {
        let source = "grid Dashboard {\n  columns sidebar content\n\n  below tablet {\n    columns content\n  }\n}\n";

        let offset = source.find("sidebar").unwrap() + 1;
        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("Grid column value"));

        let offset = source.find("tablet").unwrap() + 1;
        assert!(hover_doc_at(source, offset)
            .unwrap()
            .contains("Responsive breakpoint"));
    }

    #[test]
    fn returns_project_keyframes_hover_docs() {
        let source = "keyframes FloatIn {\n  from {\n    opacity 0\n  }\n}\ncard Panel {\n  animation FloatIn\n}\n";
        let document = frame_parser::parse(source).expect("parse");
        let symbols = frame_core::symbols::index_document(source, &document);
        let offset = source.rfind("FloatIn").unwrap() + 1;
        let doc = hover_doc_at_with_symbols(source, offset, Some(&symbols)).expect("hover doc");

        assert!(doc.contains("Custom keyframes animation"));
        assert!(doc.contains("@keyframes frame-FloatIn"));
    }
}
