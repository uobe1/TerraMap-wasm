import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  optimizeDeps: {
    exclude: ['terra-map-wasm'],
  },
  resolve: {
    alias: {
      '@terra-map-wasm/core': '/pkg',
    },
  },
  server: {
    port: 8000,
    strictPort: false,
    host: true,
  },
});