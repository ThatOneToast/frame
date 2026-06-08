# App-Driven Vocabulary

The chat reference app exposed repeated cases where `advanced { css ... }` was doing normal product-interface work. These patterns are now available as Frame-native intent.

## App Shell Tracks

Use `tracks` and repeated `areas` statements for multi-row app shells:

```frame
grid AppShell {
  columns header sidebar channels chat users composer
  tracks columns rail panel fill side
  tracks rows header fill composer
  areas header header header header
  areas sidebar channels chat users
  areas composer composer composer composer
  height screen
  overflow hidden
  box border
}
```

Named track values:

- `rail`: compact server/tool rail.
- `panel`: navigation side panel.
- `side`: inspector or member panel.
- `header`: top application band.
- `composer`: bottom composer band.
- `fill`: remaining available space.
- `auto` and `content`: intrinsic tracks.

## Dense Component Layouts

Use `layout` presets for repeated internal component structures:

```frame
card ChannelButton {
  layout icon-content-action
  gap small
  control reset
  interactive
  align-text left
  width fill
}

card MessageItem {
  layout avatar-content
  gap medium
}
```

Supported presets:

- `icon-content-action`
- `avatar-content`
- `header`
- `composer`
- `center`

## Panel Behavior

Use panel intent instead of raw overflow and box-sizing CSS:

```frame
area ChatPanel {
  scroll y
  scrollbar dense
  outline none
}

overlay AppViewport {
  overflow hidden
  box border
}
```

## Edge Borders

Use directional borders for separators:

```frame
area Sidebar {
  border right terminal-border
}

area Composer {
  border top terminal-border
}
```

## Text Behavior

Dense application labels often need truncation, wrapping, and casing:

```frame
text ChannelName {
  truncate
}

text MessageText {
  margin none
  wrap anywhere
}

text SectionLabel {
  case uppercase
  letter normal
}
```

## Controls And Sizing

Use control and sizing intent for interactive elements:

```frame
card ServerButton {
  control reset
  interactive
  square server
  layout center
}

center PresenceDot {
  square presence
}

stack UserIdentity {
  min-width zero
}

card SettingsPanel {
  width modal
  self center
}
```

These features keep `advanced` available for true escape-hatch cases while making common app layout work native to Frame.
