# Frame UI Implementation Guide

This guide helps coding agents build production-quality UIs with Frame. It covers correct patterns, common mistakes, and known limitations.

## Quick Reference

### Valid Style Declarations (file root)
```
grid Name { columns ...; rows ...; gap ...; height screen; tracks ... }
area Name { in GridName; place ...; col ...; row ... }
card Name { surface ...; padding ...; radius ...; border ...; shadow ...; color ... }
row Name { gap ...; align ...; justify ...; padding ... }
stack Name { gap ...; padding ...; align ... }
button Name { surface ...; padding ...; radius ...; color ... }
text Name { size ...; weight ...; line ... }
tokens Name { color ...; gradient ... }
keyframes Name { from { ... } to { ... } }
```

### Valid UI Primitives (inside component view)
```
screen, panel, stack, row, grid, card, action, button, text, list, data,
link, editor, input, toggle, select, choice, field, image, media, dialog,
popover, title, badge, icon, label, nav, menu, header, footer, section
```

### Properties That Emit CSS
```
surface, background, color, text, theme, padding, margin, gap, radius,
shadow, glow, border, outline, flex, align, justify, height, width,
min-height, max-height, min-width, max-width, overflow, scroll,
position, z, transition, duration, ease, animation, lift, sink,
grow, shrink, tilt, press, pop, brighten, dim, blur, ring, fade,
scale, font mono, truncate, wrap, case, align-text, decoration,
whitespace, word-break, hyphenate, size, weight, line, letter,
display, visibility, box, layout, self, nudge, offset, interactive
```

## Common Mistakes

### 1. Using `tracks` alongside `columns`
**Wrong:**
```
grid AppShell {
  columns sidebar content
  tracks columns rail panel fill
}
```
**Why it breaks:** Both emit `grid-template-columns`. The second overrides the first, creating a 3-column grid with only 2 named areas.

**Correct:** Use one or the other:
```
grid AppShell {
  columns sidebar content
}
```

### 2. Using duplicate column names
**Wrong:**
```
grid PerformanceGrid {
  columns content content
}
```
**Why it breaks:** Auto-generated grid areas assign both children to the same `grid-area: content`, causing overlap.

**Correct:** Use distinct names:
```
grid PerformanceGrid {
  columns chart prompt
}
```

### 3. Explicit style bindings to undeclared styles
**Wrong:**
```
card MetricCard:ActiveModel {
  text "Active Model"
}
```
**Why it breaks:** `ActiveModel` is not declared as a style. The compiler generates `fr-ActiveModel` class which doesn't exist in CSS. The card gets no visual styling.

**Correct:** Use the base style name:
```
card MetricCard {
  text "Active Model"
  surface raised
  border soft
}
```

### 4. Missing text color on dark backgrounds
**Wrong:**
```
card MyCard {
  surface raised
}
```
**Why it breaks:** Browser default text color is black (#000). On dark backgrounds, text is invisible.

**Correct:** Always set text color:
```
card MyCard {
  surface raised
  color text-primary
}
```

### 5. Missing border/shadow on cards
**Wrong:**
```
card MyCard {
  surface raised
  padding medium
}
```
**Why it breaks:** Cards blend into the background with no visual distinction.

**Correct:** Add visual separation:
```
card MyCard {
  surface raised
  padding medium
  border soft
  shadow small
}
```

### 6. Using `surface` in hover blocks
**Wrong:**
```
card MyCard {
  hover {
    surface overlay
  }
}
```
**Why it breaks:** Hover blocks only accept interaction effects (`lift`, `glow`, `brighten`, `dim`, `press`, `ring`), not `surface`.

**Correct:**
```
card MyCard {
  hover {
    lift small
    glow soft
  }
}
```

### 7. Using `transition` without a value
**Wrong:**
```
card MyCard {
  transition
}
```
**Why it breaks:** `transition` requires a value: `smooth`, `fast`, `slow`, or `none`.

**Correct:**
```
card MyCard {
  transition smooth
}
```

## Known Frame Limitations

### No body/root styling
Frame cannot set styles on `body` or `html`. The `:root` block only emits CSS custom properties. The `advanced { css }` escape hatch targets the current class selector, not `body`/`html`.

**Impact:** Full-page dark-themed apps have white body background and black default text.

**Workaround:** Set `color text-primary` on all visible containers. The body background remains white but is hidden by the full-viewport app shell.

### No table layout primitives
Frame has no `table`, `thead`, `tbody`, `tr`, `td`, `th` primitives. Tables must be built with `row` + `card` or `stack` + `row` patterns.

**Impact:** No automatic column alignment between rows. Manual gap/padding alignment required.

**Workaround:** Use `row` with consistent `gap` and `padding` values. Use `justify between` for spacing.

### No fixed/sticky positioning
Frame has `position sticky` and `position absolute` but no `position fixed` for elements like modals or fixed headers.

**Impact:** Cannot create fixed navigation bars or floating panels.

**Workaround:** Use `advanced { css "position" "fixed" }` if needed.

### No responsive breakpoints beyond below/above/between
Frame supports `below mobile`, `above desktop`, `between tablet desktop` but not custom breakpoints or `@media` queries.

**Impact:** Limited responsive design control.

**Workaround:** Use `advanced { css "@media" "..." }` for custom breakpoints.

## How to Validate Your Frame Code

### 1. Build and check for errors
```bash
cd implementations/llm-dashboard && ../../target/release/frame build
```

Look for:
- **Error**: Syntax or property issues that prevent compilation
- **Warning**: Missing style declarations (may indicate typos)
- **Info**: External handler references, unused state (usually OK)

### 2. Inspect generated CSS
```bash
cat src/generated/generated.css | grep "fr-YourStyle"
```

Check that:
- The CSS class exists
- Expected properties are present (background, color, border, etc.)
- No duplicate properties (indicates conflicting declarations)

### 3. Check for common issues
```bash
# Check for grid-template-columns conflicts
grep "grid-template-columns" src/generated/generated.css | sort | uniq -d

# Check for undefined style classes
grep "fr-" src/generated/generated.css | sort | uniq > /tmp/css_classes.txt
grep -oP 'fr-[A-Za-z]+' src/generated/app.ir.json | sort | uniq > /tmp/ir_classes.txt
diff /tmp/css_classes.txt /tmp/ir_classes.txt
```

### 4. Run workspace validation
```bash
cargo fmt && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

## Testing Checklist

Before claiming a Frame UI implementation is complete:

- [ ] `frame build` succeeds with no errors
- [ ] All expected CSS classes are generated
- [ ] No duplicate `grid-template-columns` in generated CSS
- [ ] All text elements have explicit color
- [ ] All cards have border or shadow for visual separation
- [ ] All hover effects use valid interaction effects
- [ ] All transitions have explicit values
- [ ] Grid column names are unique
- [ ] No explicit style bindings to undeclared styles
- [ ] `cargo fmt`, `cargo clippy`, `cargo test` all pass
