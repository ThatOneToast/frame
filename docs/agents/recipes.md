# Frame Recipes For Agents

Use these recipes as reliable starting points. Keep names PascalCase so generated class exports are easy to use.

## Dashboard With Left Sidebar, Content, And Right Inspector

Use this for chat apps, admin dashboards, project workspaces, and docs apps.

```svelte
<div class="fr-Dashboard">
  <aside class="fr-Sidebar">Channels</aside>
  <main class="fr-Content">Messages</main>
  <section class="fr-Inspector">Details</section>
</div>

<style lang="frame">
  grid Dashboard {
    columns 25% 50% 25%
    gap medium
    height screen
  }

  area Sidebar {
    in Dashboard
    col 1
    surface panel
    padding medium
  }

  area Content {
    in Dashboard
    col 2
    surface main
    padding large
  }

  area Inspector {
    in Dashboard
    col 3
    surface panel
    padding medium
  }
</style>
```

Why this works:

- percentage columns create explicit layout proportions
- `col 1`, `col 2`, and `col 3` match those percentage columns
- `surface panel` separates side regions
- `surface main` marks primary content

## Named Area Dashboard

Use this when exact percentages are less important than readable placement names.

```frame
grid Dashboard {
  columns sidebar content inspector
  gap medium
  height screen
}

area Sidebar {
  in Dashboard
  place sidebar
  surface panel
  padding medium
}

area Content {
  in Dashboard
  place content
  surface main
  padding large
}

area Inspector {
  in Dashboard
  place inspector
  surface panel
  padding medium
}
```

Why this works:

- named columns generate equal flexible columns
- `place sidebar` is valid because `sidebar` exists in `columns`
- agents can add more areas by adding more names to `columns`

## Responsive Card Grid

Use this for quick links, project cards, dashboards, and galleries.

```frame
grid QuickLinks {
  columns responsive cards
  gap medium
}

card QuickLinkCard {
  surface panel
  padding large
  radius large
  shadow soft
  text bright

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

Svelte markup with external generated classes:

```svelte
<div class={ui.QuickLinks}>
  <a class={ui.QuickLinkCard} href="/docs">Docs</a>
  <a class={ui.QuickLinkCard} href="/api">API</a>
  <a class={ui.QuickLinkCard} href="/examples">Examples</a>
</div>
```

Svelte markup with inline Frame:

```svelte
<div class="fr-QuickLinks">
  <a class="fr-QuickLinkCard" href="/docs">Docs</a>
  <a class="fr-QuickLinkCard" href="/api">API</a>
  <a class="fr-QuickLinkCard" href="/examples">Examples</a>
</div>
```

## Toolbar With Spaced Actions

Use this for headers, settings rows, and command bars.

```frame
row Toolbar {
  align center
  justify between
  gap small
  padding medium
  surface panel
}

card PrimaryButton {
  surface raised
  padding medium
  radius pill
  text bright

  focus {
    ring accent
  }

  active {
    press
  }
}
```

Svelte:

```svelte
<header class="fr-Toolbar">
  <h2>Workspace</h2>
  <button class="fr-PrimaryButton">Create</button>
</header>
```

Why this works:

- `align center` vertically centers toolbar items
- `justify between` pushes title and action apart
- card states use Frame effects rather than raw CSS

## Settings Panel

Use this for forms, preferences, and side panels.

```frame
stack SettingsPanel {
  surface main
  padding large
  gap medium
  align stretch
}

card SettingsSection {
  surface panel
  padding large
  radius large
  shadow soft
}

text SettingsHint {
  color muted
  size caption
}
```

Svelte:

```svelte
<section class="fr-SettingsPanel">
  <article class="fr-SettingsSection">
    <h2>Profile</h2>
    <p class="fr-SettingsHint">Changes apply to this workspace.</p>
  </article>
</section>
```

## Centered Empty State

Use this for no-data states and onboarding prompts.

```frame
center EmptyState {
  height screen
  surface main
  text muted
  padding large
}

card EmptyStateCard {
  surface panel
  padding large
  radius large
  shadow soft
  text bright
}
```

Svelte:

```svelte
<section class="fr-EmptyState">
  <article class="fr-EmptyStateCard">
    <h2>No projects yet</h2>
    <p>Create a project to get started.</p>
  </article>
</section>
```

## Gradient Hover Card

Use this for feature highlights and important navigation cards.

```frame
card HoverCard {
  surface gradient dusk
  padding large
  radius large
  shadow medium
  text bright

  hover {
    lift small
    glow accent
    brighten subtle
  }
}
```

Svelte:

```svelte
<a class="fr-HoverCard" href="/launch">
  Launch workspace
</a>
```

## Modal Overlay

Use this for blocking dialogs and confirmation flows.

```frame
overlay ModalLayer {
  surface glass
  padding large
  position center
  z modal
}

card ModalCard {
  surface panel
  padding large
  radius large
  shadow deep
  width 50%
}
```

Svelte:

```svelte
<div class="fr-ModalLayer">
  <section class="fr-ModalCard">
    <h2>Delete project?</h2>
    <p>This action cannot be undone.</p>
  </section>
</div>
```

## Status Cards

Use semantic colors for status.

```frame
card SuccessCard {
  surface panel
  padding medium
  radius large
  border success
  text success
}

card WarningCard {
  surface panel
  padding medium
  radius large
  border soft
  text warning
}

card DangerCard {
  surface panel
  padding medium
  radius large
  border danger
  text danger
}
```

## Split Content Layout

Use this for master/detail layouts or editor/preview layouts.

```frame
grid SplitWorkspace {
  columns 33% 66%
  gap medium
  height screen
}

area NavigationPane {
  in SplitWorkspace
  col 1
  surface panel
  padding medium
}

area DetailPane {
  in SplitWorkspace
  col 2
  surface main
  padding large
}
```

## Agent Revision Checklist

Before returning Frame code, verify:

- Every top-level block has a valid declaration keyword.
- Every `area` includes `in GridName`.
- Every `place value` matches a named value in the referenced grid.
- Percentage columns use numeric `col` placement.
- Inline Svelte examples use raw `fr-Name` classes.
- External `.frame` examples import `ui` and `generated.css`.
- Nested blocks are only `hover`, `focus`, `focus-visible`, `focus-within`, `active`, `disabled`, `checked`, `invalid`, `required`, `target`.
- State blocks contain effects, not declarations.
- Spacing values are named tokens, not pixels.
- Percentages are `0%` through `100%`.
- Component args use `:` not `=`.
- Loop keys start with `$`.
- `show when` is inside an element, not standalone.
- `slot` is at component level, not inside declarations.

## UI Component Recipes

### Simple Component With State

```frame
component Counter {
  state {
    count number = 0
  }
  view {
    text $count
    action Increment {
      text "+"
      on press @increment
    }
  }
}
```

### Component With Props and Events

```frame
component Greeting {
  props {
    name text
  }
  view {
    text "Hello, {name}"
  }
}

component ChatApp {
  props {
    title text
  }
  state {
    draft text = ""
    sending bool = false
  }
  view {
    text $title
    MessageComposer(draft bind $draft)
    action Send {
      text "Send"
      on press @sendMessage
      disabled when $sending
    }
  }
}
```

### Component With Slots

```frame
component Dialog {
  view {
    card Body {
      text "Default content"
    }
  }
  slot Default {
    text "Fallback"
  }
}
```

### Component With Conditional Rendering

```frame
component NotificationBell {
  state {
    showDropdown bool = false
    unreadCount number = 0
  }
  view {
    action Bell {
      text "Bell"
      on press @toggleDropdown
    }
    panel Dropdown {
      show when $showDropdown
      text $unreadCount
    }
  }
}
```

### Component With Loops

```frame
component MessageList {
  state {
    messages list = []
    selected text = ""
  }
  view {
    list Container {
      for message in $messages key $selected {
        item Message {
          text $message
        }
      }
      empty EmptyState {
        text "No messages"
      }
    }
  }
}
```

### Component With Style Bindings

```frame
component SubmitButton {
  state {
    sending bool = false
  }
  view {
    action Send:PrimaryButton {
      text "Send"
      on press @sendMessage
      disabled when $sending
      style when $sending = LoadingButton
    }
  }
}
```

### Component With Multiple Event Modifiers

```frame
component SearchInput {
  state {
    query text = ""
  }
  view {
    field Search {
      input QueryInput {
        value bind $query
        on keydown.enter @search
        on keydown.escape @clear
      }
    }
  }
}
```
