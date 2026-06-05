import { derived, get, writable } from 'svelte/store';
import { getChannels } from '$lib/api/channels';
import { getMessages, sendMessage } from '$lib/api/messages';
import { getServers } from '$lib/api/server';
import { getUsers } from '$lib/api/users';
import type { Channel } from '$lib/models/channel';
import type { Message } from '$lib/models/message';
import type { Server } from '$lib/models/server';
import type { User } from '$lib/models/user';

export const servers = writable<Server[]>([]);
export const channels = writable<Channel[]>([]);
export const messages = writable<Message[]>([]);
export const users = writable<User[]>([]);

export const activeServerId = writable<string>('');
export const activeChannelId = writable<string>('');

export const activeServer = derived([servers, activeServerId], ([$servers, $activeServerId]) =>
  $servers.find((server) => server.id === $activeServerId)
);

export const activeChannel = derived([channels, activeChannelId], ([$channels, $activeChannelId]) =>
  $channels.find((channel) => channel.id === $activeChannelId)
);

let initialized = false;

export async function initializeChatApp(): Promise<void> {
  if (initialized) {
    return;
  }

  initialized = true;

  const [loadedServers, loadedUsers] = await Promise.all([getServers(), getUsers()]);
  servers.set(loadedServers);
  users.set(loadedUsers);

  const firstServer = loadedServers[0];
  if (firstServer) {
    await selectServer(firstServer.id);
  }
}

export async function selectServer(serverId: string): Promise<void> {
  activeServerId.set(serverId);

  const loadedChannels = await getChannels(serverId);
  channels.set(loadedChannels);

  const nextChannel = loadedChannels[0];
  if (nextChannel) {
    await selectChannel(nextChannel.id);
  } else {
    activeChannelId.set('');
    messages.set([]);
  }
}

export async function selectChannel(channelId: string): Promise<void> {
  activeChannelId.set(channelId);
  messages.set(await getMessages(channelId));
}

export async function submitMessage(body: string): Promise<void> {
  const trimmed = body.trim();
  const channelId = get(activeChannelId);

  if (!trimmed || !channelId) {
    return;
  }

  const message = await sendMessage(channelId, 'ada', trimmed);
  messages.update((current) => [...current, message]);
}
