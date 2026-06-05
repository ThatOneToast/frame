import type { Message } from '$lib/models/message';

const messages: Message[] = [
  {
    id: 'msg-001',
    channelId: 'general',
    authorId: 'ada',
    body: 'The chat app should read like a product team lives in it, not like a demo page.',
    sentAt: '2026-06-05T13:12:00.000Z',
    edited: false,
    reactions: [{ label: 'ack', count: 4 }]
  },
  {
    id: 'msg-002',
    channelId: 'general',
    authorId: 'soren',
    body: 'I split the Frame files by surface area. LSP navigation feels much better when styles follow component ownership.',
    sentAt: '2026-06-05T13:18:00.000Z',
    edited: true,
    reactions: [{ label: 'ship', count: 2 }]
  },
  {
    id: 'msg-003',
    channelId: 'general',
    authorId: 'noor',
    body: 'Composer focus states are visible now. The terminal look still needs to respect keyboard users.',
    sentAt: '2026-06-05T13:24:00.000Z',
    edited: false,
    reactions: []
  },
  {
    id: 'msg-004',
    channelId: 'compiler',
    authorId: 'mika',
    body: 'Responsive blocks are still future-facing, so this implementation uses desktop-first grid classes and documents the intended path.',
    sentAt: '2026-06-05T14:03:00.000Z',
    edited: false,
    reactions: [{ label: 'note', count: 3 }]
  },
  {
    id: 'msg-005',
    channelId: 'showcase',
    authorId: 'ada',
    body: 'Reference apps should push Frame hard: grid, stack, row, cards, buttons, states, typography, and tokens.',
    sentAt: '2026-06-05T14:37:00.000Z',
    edited: false,
    reactions: [{ label: 'ref', count: 6 }]
  }
];

export async function getMessages(channelId: string): Promise<Message[]> {
  return messages.filter((message) => message.channelId === channelId);
}

export async function sendMessage(channelId: string, authorId: string, body: string): Promise<Message> {
  const message: Message = {
    id: `msg-${Date.now()}`,
    channelId,
    authorId,
    body,
    sentAt: new Date().toISOString(),
    edited: false,
    reactions: []
  };

  messages.push(message);
  return message;
}
