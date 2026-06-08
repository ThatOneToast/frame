# Contracts

Frame should let developers write UI before implementation code exists.

The compiler should generate contracts for external code.

## Goals

- Type props.
- Type state.
- Type handlers.
- Type handler context.
- Avoid overwriting user files.

## Planned Outputs

Possible generated files:

- `Component.frame.contract.ts`
- `Component.frame.generated.ts`
- `Component.handlers.ts` when missing

## Handler References

Frame syntax:

```frame
action Send {
  on press @sendMessage
}
```

Generated contract shape:

```ts
export type ChatInputHandlers = {
  sendMessage(ctx: FrameEventContext): void | Promise<void>;
};
```

## State References

Frame syntax:

```frame
text $username
```

Generated contracts should preserve the expected type of `username`.

## File Safety

Generated files should be safe by default:

- generated files can be overwritten
- user files should not be overwritten
- skeleton files should only be created when missing
- force flags should be explicit
