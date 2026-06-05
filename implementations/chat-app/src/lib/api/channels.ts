import type { Channel } from '$lib/models/channel';

const channels: Channel[] = [
  {
    id: 'general',
    serverId: 'frame-labs',
    name: 'general',
    kind: 'text',
    unreadCount: 3,
    topic: 'Frame language design, examples, and release notes.'
  },
  {
    id: 'compiler',
    serverId: 'frame-labs',
    name: 'compiler',
    kind: 'text',
    unreadCount: 5,
    topic: 'Parser, diagnostics, code generation, and LSP behavior.'
  },
  {
    id: 'showcase',
    serverId: 'frame-labs',
    name: 'showcase',
    kind: 'text',
    unreadCount: 0,
    topic: 'Reference apps and component styling experiments.'
  },
  {
    id: 'ship-room',
    serverId: 'terminal-guild',
    name: 'ship-room',
    kind: 'text',
    unreadCount: 2,
    topic: 'Dense product work in terminal-inspired interfaces.'
  },
  {
    id: 'voice-standup',
    serverId: 'terminal-guild',
    name: 'standup',
    kind: 'voice',
    unreadCount: 0,
    topic: 'Reserved for a future voice channel implementation.'
  },
  {
    id: 'alerts',
    serverId: 'infra-ops',
    name: 'alerts',
    kind: 'system',
    unreadCount: 0,
    topic: 'Backend health and deployment alerts.'
  }
];

export async function getChannels(serverId: string): Promise<Channel[]> {
  return channels.filter((channel) => channel.serverId === serverId);
}
