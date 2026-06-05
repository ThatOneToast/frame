export type UserPresence = 'online' | 'idle' | 'busy' | 'offline';

export interface User {
  id: string;
  displayName: string;
  handle: string;
  role: 'admin' | 'maintainer' | 'member' | 'guest';
  presence: UserPresence;
}
