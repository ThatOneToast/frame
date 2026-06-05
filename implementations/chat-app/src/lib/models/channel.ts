export type ChannelKind = 'text' | 'voice' | 'system';

export interface Channel {
  id: string;
  serverId: string;
  name: string;
  kind: ChannelKind;
  unreadCount: number;
  topic: string;
}
