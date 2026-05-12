const esbuild = require('esbuild');

const dev = process.argv.includes('--sourcemap');

esbuild.build({
  entryPoints: ['src/extension.ts'],
  bundle: true,
  outfile: 'dist/extension.js',
  platform: 'node',
  target: 'node18',
  external: ['vscode'],
  format: 'cjs',
  sourcemap: dev,
  minify: !dev,
}).catch(() => process.exit(1));
