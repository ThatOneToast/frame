import { defineFrameIrDocument, type FrameIrDocument } from '../src/index.js';

export const generatedIr = defineFrameIrDocument({
  version: 1,
  components: [
    {
      name: 'TypedFixture',
      props: [],
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
            kind: 'action',
            semantic_kind: 'action',
            render_kind: 'button',
            name: 'Send',
            style: {
              Explicit: {
                style: 'PrimaryAction',
                source: { start: 4, end: 5 }
              }
            },
            attributes: [],
            bindings: [],
            events: [
              {
                event: 'press',
                modifiers: [],
                handler: 'sendMessage',
                source: { start: 5, end: 6 }
              }
            ],
            conditions: [
              {
                Style: {
                  state: 'sending',
                  style: 'SendingAction',
                  source: { start: 6, end: 7 }
                }
              }
            ],
            children: [],
            source: { start: 4, end: 7 }
          }
        }
      ],
      capabilities: ['EventBinding', 'ConditionalStyles'],
      source: { start: 0, end: 7 }
    }
  ]
} as const) satisfies FrameIrDocument;
