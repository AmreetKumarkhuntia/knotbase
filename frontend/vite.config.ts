import { fileURLToPath, URL } from 'node:url'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [svelte()],
  // the SvelteKit `$lib` convention, without SvelteKit; tsconfig.app.json mirrors it for svelte-check
  resolve: { alias: { $lib: fileURLToPath(new URL('./src/lib', import.meta.url)) } },
  // prefix match, no rewrite: /api/* reaches Axum unchanged. PORT moves the Rust server and this proxy follows it
  // (dev.sh hands the environment to both), so `PORT=3100 npm start` moves both.
  server: {
    port: 5025,
    strictPort: true,
    cors: false,
    proxy: { '/api': `http://localhost:${process.env.PORT ?? 3000}` },
  },
})
