import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { framePlugin } from '@frame/svelte/vite';

const frameBin = 'cargo run -p frame_cli --quiet --';

export default defineConfig({
  plugins: [
    framePlugin({
      input: 'src/lib/frame/app.frame',
      outDir: 'src/lib/frame',
      frameBin
    }),
    sveltekit()
  ]
});
