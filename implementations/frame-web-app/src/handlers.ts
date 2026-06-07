import type { TodoAppHandlers } from './generated/frame.types';

export const handlers: TodoAppHandlers = {
  addTask(ctx) {
    const draft = String(ctx.state.get('draft'));
    if (!draft.trim()) return;

    const items = ctx.state.get('items') as Array<{ id: number; label: string; done: boolean }>;
    const nextId = Number(ctx.state.get('nextId'));

    ctx.state.set('items', [
      ...items,
      { id: nextId, label: draft.trim(), done: false }
    ]);
    ctx.state.set('draft', '');
    ctx.state.set('nextId', nextId + 1);
  },

  toggleTask(ctx) {
    // In a real app, the task ID would be passed through the event context.
    // For this minimal example, we toggle the first undone task.
    const items = [...(ctx.state.get('items') as Array<{ id: number; label: string; done: boolean }>)];
    const index = items.findIndex((item) => !item.done);
    if (index >= 0) {
      items[index] = { ...items[index], done: true };
      ctx.state.set('items', items);
    }
  }
};
