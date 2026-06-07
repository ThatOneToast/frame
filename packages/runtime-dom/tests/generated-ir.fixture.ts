import { defineFrameIrDocument, type FrameIrDocument } from '../src/index.js';

export const generatedIr = defineFrameIrDocument({
  version: 1,
  components: [
    {
      name: 'TypedFixture',
      props: [
        {
          name: 'title',
          value_type: 'Text',
          readonly: true,
          binding: 'Input',
          source: { start: 0, end: 1 }
        }
      ],
      state: [
        {
          name: 'draft',
          value_type: 'Text',
          default: { Text: '' },
          source: { start: 0, end: 1 }
        },
        {
          name: 'sending',
          value_type: 'Bool',
          default: { Bool: false },
          source: { start: 1, end: 2 }
        },
        {
          name: 'count',
          value_type: 'Number',
          default: { Number: '0' },
          source: { start: 2, end: 3 }
        },
        {
          name: 'messages',
          value_type: 'List',
          default: 'List',
          source: { start: 3, end: 4 }
        }
      ],
      slots: [],
      nodes: [
        {
          Element: {
            kind: 'field',
            semantic_kind: 'field',
            render_kind: 'div',
            name: 'MessageField',
            style: {
              Explicit: {
                style: 'MessageFieldStyle',
                source: { start: 4, end: 5 }
              }
            },
            attributes: [],
            bindings: [
              {
                property: 'value',
                state: 'draft',
                source: { start: 5, end: 6 }
              }
            ],
            conditions: [
              {
                Show: {
                  state: 'title',
                  source: { start: 6, end: 7 }
                }
              },
              {
                Hidden: {
                  state: 'sending',
                  source: { start: 7, end: 8 }
                }
              },
              {
                Property: {
                  property: 'disabled',
                  state: 'sending',
                  source: { start: 8, end: 9 }
                }
              },
              {
                Style: {
                  state: 'sending',
                  style: 'SendingAction',
                  source: { start: 9, end: 10 }
                }
              }
            ],
            events: [
              {
                event: 'press',
                modifiers: ['prevent'],
                handler: 'sendMessage',
                source: { start: 10, end: 11 }
              }
            ],
            children: [
              {
                Text: {
                  value: { DataRef: 'draft' },
                  source: { start: 11, end: 12 }
                }
              },
              {
                Component: {
                  name: 'NestedBadge',
                  arguments: [
                    {
                      name: 'label',
                      value: { DataRef: 'title' },
                      source: { start: 12, end: 13 }
                    },
                    {
                      name: 'active',
                      value: { Bind: 'sending' },
                      source: { start: 13, end: 14 }
                    }
                  ],
                  source: { start: 12, end: 14 }
                }
              },
              {
                List: {
                  item: 'message',
                  collection: 'messages',
                  key: 'message.id',
                  children: [
                    {
                      Element: {
                        kind: 'item',
                        semantic_kind: 'item',
                        render_kind: 'div',
                        name: 'MessageItem',
                        style: {
                          Automatic: {
                            style: 'MessageItem',
                            source: { start: 14, end: 15 }
                          }
                        },
                        attributes: [
                          {
                            name: 'value',
                            value: { DataRef: 'message.text' },
                            source: { start: 15, end: 16 }
                          }
                        ],
                        bindings: [],
                        events: [],
                        conditions: [],
                        children: [],
                        source: { start: 14, end: 16 }
                      }
                    }
                  ],
                  source: { start: 14, end: 17 }
                }
              }
            ],
            source: { start: 4, end: 17 }
          }
        }
      ],
      capabilities: [
        'ComponentComposition',
        'ConditionalRendering',
        'ConditionalStyles',
        'EventBinding',
        'ListRendering',
        'TwoWayBinding'
      ],
      source: { start: 0, end: 17 }
    },
    {
      name: 'NestedBadge',
      props: [
        {
          name: 'label',
          value_type: 'Text',
          readonly: true,
          binding: 'Input',
          source: { start: 17, end: 18 }
        },
        {
          name: 'active',
          value_type: 'Bool',
          readonly: false,
          binding: 'TwoWayAllowed',
          source: { start: 18, end: 19 }
        }
      ],
      state: [],
      slots: [],
      nodes: [
        {
          Element: {
            kind: 'badge',
            semantic_kind: 'badge',
            render_kind: 'span',
            name: 'Badge',
            style: {
              Automatic: {
                style: 'Badge',
                source: { start: 19, end: 20 }
              }
            },
            attributes: [
              {
                name: 'value',
                value: { DataRef: 'label' },
                source: { start: 20, end: 21 }
              }
            ],
            bindings: [],
            events: [],
            conditions: [],
            children: [],
            source: { start: 19, end: 21 }
          }
        }
      ],
      capabilities: [],
      source: { start: 17, end: 21 }
    }
  ]
} as const) satisfies FrameIrDocument;
