import adapter from '@sveltejs/adapter-auto';
import { framePreprocess } from '@frame/svelte';

const frameBin = 'cargo run -p frame_cli --quiet --';

export default {
  kit: {
    adapter: adapter()
  },
  preprocess: [
    framePreprocess({
      frameBin
    })
  ]
};
