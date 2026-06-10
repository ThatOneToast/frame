import type { LLMDashboardHandlers } from './generated/frame.types';

export const handlers: LLMDashboardHandlers = {
  runTest(ctx) {
    const query = String(ctx.state.get('searchQuery'));
    console.log('Running inference test with query:', query);
  },

  filterRuns(ctx) {
    const provider = String(ctx.state.get('selectedProvider'));
    console.log('Filtering runs by provider:', provider);
  }
};
