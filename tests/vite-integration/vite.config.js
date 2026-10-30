import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

// Top-level await is native at this target, so no plugin is needed for it.
export default defineConfig({
  plugins: [wasm()],
  build: {
    target: 'esnext',
  },
});
