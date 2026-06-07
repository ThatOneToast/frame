export type FrameEventContext<TState, TProps> = {
  state: TState;
  props: TProps;
  event: Event;
};

export type FramePressEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameInputEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameToggleEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameKeyboardEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameFormEvent<TState, TProps> = FrameEventContext<TState, TProps>;

export type TodoAppState = {
  items: unknown[];
  draft: string;
  nextId: number;
};

export type TodoAppHandlers = {
  addTask(ctx: FrameKeyboardEvent<TodoAppState, {}>): void | Promise<void>;
  toggleTask(ctx: FramePressEvent<TodoAppState, {}>): void | Promise<void>;
};

