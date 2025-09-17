import commonjs from '@rollup/plugin-commonjs'
import resolve from '@rollup/plugin-node-resolve'
import terser from '@rollup/plugin-terser'
import { defineConfig } from 'rollup'

export default defineConfig({
  input: 'packages/faster-md/src/index.ts',
  output: [
    {
      file: 'dist-release/faster-md/bundle.cjs',
      format: 'cjs',
      sourcemap: true,
    },
    {
      file: 'dist-release/faster-md/bundle.mjs',
      format: 'esm',
      sourcemap: true,
    },
  ],
  plugins: [resolve(), commonjs(), terser()],
})
