import { fileURLToPath, URL } from 'node:url'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [svelte()],
  // the SvelteKit `$lib` convention, without SvelteKit; tsconfig.app.json mirrors it for svelte-check
  resolve: { alias: { $lib: fileURLToPath(new URL('./src/lib', import.meta.url)) } },
  // prefix match, no rewrite: /api/* reaches Axum unchanged
  server: { port: 5025, strictPort: true, cors: false, proxy: { '/api': 'http://localhost:3000' } },
})
