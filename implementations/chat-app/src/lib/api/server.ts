import type { Server } from '$lib/models/server';

const servers: Server[] = [
  { id: 'frame-labs', name: 'Frame Labs', abbreviation: 'FL', unreadCount: 8, status: 'online' },
  { id: 'terminal-guild', name: 'Terminal Guild', abbreviation: 'TG', unreadCount: 2, status: 'quiet' },
  { id: 'infra-ops', name: 'Infra Ops', abbreviation: 'IO', unreadCount: 0, status: 'maintenance' }
];

export async function getServers(): Promise<Server[]> {
  return servers;
}
