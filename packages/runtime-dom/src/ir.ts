export type FrameIrDocument = {
  version: number;
  components: readonly FrameIrComponent[];
};

export type FrameIrComponent = {
  name: string;
  props: readonly FrameIrProp[];
  state: readonly FrameIrState[];
  slots?: readonly FrameIrSlot[];
  nodes: readonly FrameIrNode[];
  capabilities: readonly string[];
  source: FrameIrSourceSpan;
};

export type FrameIrProp = {
  name: string;
  value_type: FrameIrValueType;
  readonly?: boolean;
  binding?: string;
  source: FrameIrSourceSpan;
};

export type FrameIrState = {
  name: string;
  value_type: FrameIrValueType;
  default: FrameIrStateDefault;
  source: FrameIrSourceSpan;
};

export type FrameIrValueType = 'Text' | 'Bool' | 'Number' | 'List' | { Unknown: string };

export type FrameIrStateDefault =
  | { Text: string }
  | { Bool: boolean }
  | { Number: string }
  | 'List'
  | { Invalid: string };

export type FrameIrSlot = {
  name: string;
  fallback: readonly FrameIrNode[];
  source: FrameIrSourceSpan;
};

export type FrameIrNode =
  | { Element: FrameIrElement }
  | { Text: FrameIrText }
  | { Component: FrameIrComponentInvocation }
  | { List: FrameIrList };

export type FrameIrElement = {
  kind: string;
  semantic_kind?: string;
  render_kind?: string;
  name: string;
  style: FrameIrStyleBinding;
  attributes: readonly FrameIrAttribute[];
  bindings: readonly FrameIrBinding[];
  events: readonly FrameIrEvent[];
  conditions: readonly FrameIrCondition[];
  children: readonly FrameIrNode[];
  source: FrameIrSourceSpan;
};

export type FrameIrText = {
  value: FrameIrTextValue;
  source: FrameIrSourceSpan;
};

export type FrameIrTextValue = { Literal: string } | { DataRef: string };

export type FrameIrComponentInvocation = {
  name: string;
  arguments: readonly FrameIrComponentArgument[];
  source: FrameIrSourceSpan;
};

export type FrameIrComponentArgument = {
  name: string;
  value: FrameIrComponentArgumentValue;
  source: FrameIrSourceSpan;
};

export type FrameIrComponentArgumentValue =
  | { DataRef: string }
  | { Bind: string }
  | { Literal: string };

export type FrameIrList = {
  item: string;
  collection: string;
  key?: string | null;
  children: readonly FrameIrNode[];
  source: FrameIrSourceSpan;
};

export type FrameIrAttribute = {
  name: string;
  value: FrameIrAttributeValue;
  source: FrameIrSourceSpan;
};

export type FrameIrAttributeValue = { Literal: string } | { DataRef: string };

export type FrameIrBinding = {
  property: string;
  state: string;
  source: FrameIrSourceSpan;
};

export type FrameIrEvent = {
  event: string;
  modifiers: readonly string[];
  handler: string;
  source: FrameIrSourceSpan;
};

export type FrameIrStyleBinding =
  | { Explicit: { style: string; source: FrameIrSourceSpan } }
  | { Automatic: { style: string; source: FrameIrSourceSpan } };

export type FrameIrCondition =
  | { Show: { state: string; source: FrameIrSourceSpan } }
  | { Hidden: { state: string; source: FrameIrSourceSpan } }
  | { Property: { property: string; state: string; source: FrameIrSourceSpan } }
  | { Style: { state: string; style: string; source: FrameIrSourceSpan } };

export type FrameIrSourceSpan = {
  start: number;
  end: number;
};
