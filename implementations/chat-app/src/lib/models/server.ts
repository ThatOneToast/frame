export interface Server {
  id: string;
  name: string;
  abbreviation: string;
  unreadCount: number;
  status: 'online' | 'maintenance' | 'quiet';
}
