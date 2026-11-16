import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { createEngine } from '../src/engine.ts';
import { renderBlock, resolveTheme, sanitizeWidth } from '../src/render.ts';

const WASM = new URL('../src/wasm/declart_wasm_bg.wasm', import.meta.url);
const engine = createEngine(new Uint8Array(readFileSync(WASM)));

const TIER = `kind = "tier"
title = "Test"

[[items]]
label = "Top"

[[items]]
label = "Bottom"`;

test('resolveTheme maps auto to dark on a dark app', () => {
  assert.equal(resolveTheme('auto', true), 'dark');
});

test('resolveTheme maps auto to default on a light app', () => {
  assert.equal(resolveTheme('auto', false), 'default');
});

test('resolveTheme keeps an explicit theme regardless of app theme', () => {
  assert.equal(resolveTheme('warm', true), 'warm');
});

test('sanitizeWidth keeps a positive integer', () => {
  assert.equal(sanitizeWidth(800), 800);
});

test('sanitizeWidth floors a fractional width', () => {
  assert.equal(sanitizeWidth(640.9), 640);
});

test('sanitizeWidth treats zero, negative, NaN, Infinity and non-numbers as unset', () => {
  for (const raw of [0, -5, NaN, Infinity, 'abc', null, undefined, 0.4]) {
    assert.equal(sanitizeWidth(raw), undefined, String(raw));
  }
});

test('renderBlock renders a valid declaration with alt text', () => {
  const r = renderBlock(engine, TIER, { theme: 'default' });
  assert.ok(r.ok);
  assert.match(r.svg, /<svg/);
  assert.ok(r.alt.length > 0);
});

test('renderBlock reports an empty or whitespace-only block without throwing', () => {
  for (const src of ['', '   \n ']) {
    const r = renderBlock(engine, src, { theme: 'default' });
    assert.equal(r.ok, false);
    assert.ok(!r.ok && r.message.length > 0);
  }
});

test('renderBlock renders a JSON declaration', () => {
  const json = '{"kind":"tier","items":[{"label":"A"},{"label":"B"}]}';
  assert.ok(renderBlock(engine, json, { theme: 'default' }).ok);
});

test('renderBlock renders a TOML declaration with CRLF line endings', () => {
  const r = renderBlock(engine, TIER.replace(/\n/g, '\r\n'), { theme: 'default' });
  assert.ok(r.ok);
});

test('renderBlock reports an invalid declaration with the engine message', () => {
  const r = renderBlock(engine, 'kind = ', { theme: 'default' });
  assert.equal(r.ok, false);
  assert.ok(!r.ok && r.message.length > 0);
});

test('renderBlock falls back for an unknown theme instead of throwing', () => {
  assert.ok(renderBlock(engine, TIER, { theme: 'no-such-theme' }).ok);
});

test('renderBlock leaves the backdrop unpainted', () => {
  const r = renderBlock(engine, TIER, { theme: 'default' });
  assert.ok(r.ok);
  assert.notEqual(r.svg, engine.render(TIER, 'default', undefined, false));
});
