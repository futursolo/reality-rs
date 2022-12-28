import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    target: 'esnext',
    lib: {
      entry: 'src/index.ts',
      name: 'reality-exports',
      formats: ['es', 'cjs', 'umd'],
    },
    rollupOptions: {
      external: [
        'node:util',
        'node:buffer',
        'node:stream',
        'node:net',
        'node:url',
        'node:fs',
        'node:path',
        'perf_hooks',
      ],
      output: {
        globals: {
          'node:stream': 'stream',
          'node:buffer': 'buffer',
          'node:util': 'util',
          'node:net': 'net',
          'node:url': 'url',
          perf_hooks: 'perf_hooks',
        },
        inlineDynamicImports: true,
      },
    },
  },
});
