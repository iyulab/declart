import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { createEngine } from '../src/engine.ts';

const WASM = new URL('../src/wasm/declart_wasm_bg.wasm', import.meta.url);

const TIER = `kind = "tier"
title = "Test"

[[items]]
label = "Top"

[[items]]
label = "Bottom"`;

const engine = createEngine(new Uint8Array(readFileSync(WASM)));

test('render returns an svg for a valid declaration', () => {
  const svg = engine.render(TIER, 'default', undefined, true);
  assert.match(svg, /<svg/);
});

test('describe returns a non-empty text equivalent', () => {
  const alt = engine.describe(TIER);
  assert.equal(typeof alt, 'string');
  assert.ok(alt.length > 0);
});

test('themes lists the dark theme', () => {
  assert.ok(engine.themes().includes('dark'));
});

test('render throws on an invalid declaration', () => {
  assert.throws(() => engine.render('kind = ', 'default'));
});
