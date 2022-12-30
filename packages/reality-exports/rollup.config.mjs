import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import replace from '@rollup/plugin-replace';

export default {
  input: 'src/index.ts',
  output: {
    file: 'dist/reality-exports.cjs',
    format: 'cjs',
    generatedCode: 'es2015',
  },
  plugins: [
    nodeResolve(),
    typescript(),
    commonjs({
      ignore: ['canvas'],
    }),
    json(),
    replace({
      values: {
        'require.resolve ? require.resolve("./xhr-sync-worker.js") : null': 'null',
      },
      preventAssignment: true,
    }),
  ],
};
