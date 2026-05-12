// Build script: wasm-pack → media/wasm/ (web) + media/node-wasm/ (nodejs)
// Uses absolute paths so --out-dir resolves correctly regardless of CWD.
const { execFileSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const crate = path.join(__dirname, '..', '..', 'crates', 'declart-wasm');
const webOutDir = path.join(__dirname, 'media', 'wasm');
const nodeWasmSrc = path.join(__dirname, '..', 'declart-node', 'wasm');
const nodeWasmDst = path.join(__dirname, 'media', 'node-wasm');

// 1. Build web target for the preview webview
execFileSync('wasm-pack', ['build', crate, '--target', 'web', '--out-dir', webOutDir], {
  stdio: 'inherit',
});

// 2. Copy Node.js target from declart-node/wasm/ for the markdown extension host
//    (uses existing nodejs wasm build rather than rebuilding)
if (fs.existsSync(nodeWasmSrc)) {
  fs.mkdirSync(nodeWasmDst, { recursive: true });
  for (const file of fs.readdirSync(nodeWasmSrc)) {
    fs.copyFileSync(path.join(nodeWasmSrc, file), path.join(nodeWasmDst, file));
  }
  process.stdout.write(`Copied node-wasm from ${nodeWasmSrc}\n`);
} else {
  process.stderr.write(`Warning: declart-node/wasm not found at ${nodeWasmSrc}. Run 'npm run build' in packages/declart-node first.\n`);
}
