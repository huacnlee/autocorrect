import react from '@vitejs/plugin-react-swc';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: 'esnext',
  },
  base: '/autocorrect/',
  plugins: [react(), wasm()],
});
