import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [svelte()],
  // prefix match, no rewrite: /api/* reaches Axum unchanged
  server: { port: 5025, strictPort: true, cors: false, proxy: { '/api': 'http://localhost:3000' } },
})
