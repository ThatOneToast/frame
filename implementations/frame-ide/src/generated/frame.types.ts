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

export type IdeAppState = {
  files: unknown[];
  currentPath: string;
  currentContent: string;
  status: string;
  lspConnected: boolean;
};

export type IdeAppHandlers = {
  editorChange(ctx: FrameInputEvent<IdeAppState, {}>): void | Promise<void>;
  editorKeydown(ctx: FrameKeyboardEvent<IdeAppState, {}>): void | Promise<void>;
  newFile(ctx: FramePressEvent<IdeAppState, {}>): void | Promise<void>;
  openFile(ctx: FramePressEvent<IdeAppState, {}>): void | Promise<void>;
  openFileByPath(ctx: FramePressEvent<IdeAppState, {}>): void | Promise<void>;
  saveFile(ctx: FramePressEvent<IdeAppState, {}>): void | Promise<void>;
  toggleLsp(ctx: FramePressEvent<IdeAppState, {}>): void | Promise<void>;
};

