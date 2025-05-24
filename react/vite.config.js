import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  server: {
    fs: {
      allow: [
        'C:/Users/Developer/game/wararar/rust/pkg',  // дозвіл на цю папку
        'C:/Users/Developer/game/wararar/react',     // і на основну папку
      ],
    },
  },
});
