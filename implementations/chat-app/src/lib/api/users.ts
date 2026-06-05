import type { User } from '$lib/models/user';

const users: User[] = [
  { id: 'ada', displayName: 'Ada Chen', handle: 'ada', role: 'admin', presence: 'online' },
  { id: 'mika', displayName: 'Mika Torres', handle: 'mika', role: 'maintainer', presence: 'busy' },
  { id: 'noor', displayName: 'Noor Patel', handle: 'noor', role: 'member', presence: 'idle' },
  { id: 'soren', displayName: 'Soren Vale', handle: 'soren', role: 'member', presence: 'online' },
  { id: 'jules', displayName: 'Jules Marin', handle: 'jules', role: 'guest', presence: 'offline' }
];

export async function getUsers(): Promise<User[]> {
  return users;
}
