import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { test } from 'node:test';

const ROOT = new URL('../', import.meta.url);
const MAIN = new URL('main.js', ROOT);

// Bundle once; the wasm-pack step is covered by `npm run build:wasm`.
execFileSync(process.execPath, ['esbuild.js'], { cwd: ROOT, stdio: 'inherit' });

test('bundle produces main.js', () => {
  assert.ok(existsSync(MAIN));
});

test('bundle requires nothing but obsidian', () => {
  const src = readFileSync(MAIN, 'utf8');
  const required = new Set([...src.matchAll(/require\(\s*["']([^"']+)["']\s*\)/g)].map((m) => m[1]));
  assert.deepEqual([...required], ['obsidian']);
});

test('bundle stays under 3MB with the wasm inlined', () => {
  assert.ok(statSync(MAIN).size < 3 * 1024 * 1024);
});
