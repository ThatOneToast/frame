// Generated TypeScript contracts. Do not edit; regenerate with `frame build`.
// Source: src/app.frame
// Ownership: generated-only

import type { FrameStateController } from '@frame/runtime-dom';

export type FrameEventContext<TState, TProps> = {
  state: FrameStateController;
  props: Readonly<TProps>;
  event: Event;
  readonly stateShape?: TState;
};

export type FramePressEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameInputEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameToggleEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameKeyboardEvent<TState, TProps> = FrameEventContext<TState, TProps>;
export type FrameFormEvent<TState, TProps> = FrameEventContext<TState, TProps>;

export type LLMDashboardState = {
};

export type LLMDashboardHandlers = {
  filterRuns(ctx: FramePressEvent<LLMDashboardState, {}>): void | Promise<void>;
  runTest(ctx: FramePressEvent<LLMDashboardState, {}>): void | Promise<void>;
};

