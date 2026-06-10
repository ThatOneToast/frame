import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
  resolve: {
    alias: {
      '@frame/runtime-dom': resolve(__dirname, '../../packages/runtime-dom/src/index.ts')
    }
  }
});
