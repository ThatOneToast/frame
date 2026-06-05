# Syntax Notes

These notes describe planned Frame UI syntax. They are not implemented yet.

## Element Names and Styles

Automatic style lookup:

```frame
button Send {
  text "Send"
}
```

Explicit style binding:

```frame
button Send:PrimaryButton {
  text "Send"
}
```

`Send` is the UI node name. `PrimaryButton` is the style name.

## Events

```frame
button Send {
  on click @sendMessage
  on keydown.enter @submitMessage
  on keydown.ctrl.enter @submitMessage
}
```

Handlers live outside Frame.

## Data

```frame
text $username
```

Data references should be typed and escaped by default.

## Bindings

```frame
input DraftBox {
  value bind $draft
}
```

Bindings should create clear runtime state dependencies.

## Reactive Styles

```frame
button Send:PrimaryButton {
  style when $sending = LoadingButton
}
```

Reactive styles should become IR rules that the runtime can patch.

## Conditions

```frame
panel UserMenu {
  show when $isLoggedIn
}
```

## Lists

```frame
repeat message in $messages keyed message.id {
  MessageBubble {
    author $message.author
    content $message.content
  }
}
```

Dynamic lists should prefer keys unless the compiler can prove the list is static.
