// Build script: wasm-pack (web target) -> src/wasm/
// The wasm binary is inlined into main.js by esbuild.js, since Obsidian installs
// only main.js, manifest.json and styles.css.
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const crate = path.join(here, '..', '..', 'crates', 'declart-wasm');
const outDir = path.join(here, 'src', 'wasm');

execFileSync('wasm-pack', ['build', crate, '--target', 'web', '--out-dir', outDir], {
  stdio: 'inherit',
});
