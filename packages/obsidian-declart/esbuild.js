// Bundles src/main.ts into a single main.js. The wasm binary is inlined as base64
// because Obsidian installs only main.js, manifest.json and styles.css.
import esbuild from 'esbuild';

const dev = process.argv.includes('--sourcemap');

await esbuild.build({
  entryPoints: ['src/main.ts'],
  bundle: true,
  outfile: 'main.js',
  format: 'cjs',
  platform: 'browser',
  target: 'es2020',
  external: ['obsidian', 'electron'],
  loader: { '.wasm': 'base64' },
  // wasm-bindgen's default loader references import.meta.url; we always pass bytes, so it is never used.
  define: { 'import.meta.url': '"file:///declart"' },
  sourcemap: dev ? 'inline' : false,
  minify: !dev,
  logLevel: 'info',
});
