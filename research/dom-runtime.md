# DOM Runtime

The DOM runtime is the first serious renderer for Frame IR.

It should be small, explicit, and easy to debug.

## Responsibilities

- Mount a Frame component into a DOM container.
- Create elements and text nodes.
- Apply attributes and DOM properties.
- Bind events to external handlers.
- Apply style classes from Frame styling output.
- Apply reactive style changes.
- Render conditions and loops.
- Support form bindings.
- Clean up listeners and runtime resources.

## Runtime Boundary

Frame does not contain inline JavaScript. Runtime behavior is linked through handler references such as:

```frame
on click @sendMessage
```

The runtime receives a handler map from user code.

```ts
createFrameApp(definition, {
  handlers
}).mount(document.getElementById("app"));
```

## Patch Model

The runtime should eventually apply patches instead of rebuilding whole trees.

Patch examples:

- set text
- set attribute
- remove attribute
- set property
- set class
- insert node
- remove node
- move keyed node
- attach listener
- detach listener

## Testing

Runtime tests should cover:

- element creation
- escaped text output
- event binding
- state updates
- conditional rendering
- keyed list updates
- form input binding
- cleanup after unmount
