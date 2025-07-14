import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  server: {
    fs: {
      allow: [
        'D:/wormy/rust-wasm-wormy/react',
        'C:/Users/Developer/game/wararar/react',
        'C:/Users/Developer/game/wararar',
        'D:/wormy/rust-wasm-wormy'
      ],
    },
  },
});
