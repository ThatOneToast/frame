export interface Message {
  id: string;
  channelId: string;
  authorId: string;
  body: string;
  sentAt: string;
  edited: boolean;
  reactions: MessageReaction[];
}

export interface MessageReaction {
  label: string;
  count: number;
}
