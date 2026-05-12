import assert from 'node:assert/strict';
import { test } from 'node:test';
import plugin from './index.js';

const PYRAMID = `kind = "sequence"
view = "pyramid"
title = "Test"

[[items]]
label = "Top"

[[items]]
label = "Bottom"`;

function makeCodeTree(lang, value) {
  return {
    type: 'root',
    children: [{ type: 'code', lang, value, position: null }],
  };
}

test('renders declart code block to SVG figure', () => {
  const tree = makeCodeTree('declart', PYRAMID);
  plugin()(tree);
  const node = tree.children[0];
  assert.strictEqual(node.type, 'html');
  assert.ok(node.value.startsWith('<figure class="declart">'));
  assert.ok(node.value.includes('<svg'));
  assert.ok(node.value.endsWith('</figure>'));
});

test('ignores non-declart code blocks', () => {
  const tree = makeCodeTree('js', 'console.log(1)');
  plugin()(tree);
  assert.strictEqual(tree.children[0].type, 'code');
});

test('ignores code blocks with no lang', () => {
  const tree = makeCodeTree(null, 'hello');
  plugin()(tree);
  assert.strictEqual(tree.children[0].type, 'code');
});

test('emits error figure on invalid input', () => {
  const tree = makeCodeTree('declart', 'not valid toml @@@@');
  plugin()(tree);
  const node = tree.children[0];
  assert.strictEqual(node.type, 'html');
  assert.ok(node.value.includes('class="declart-error"'));
});

test('respects theme option', () => {
  const tree = makeCodeTree('declart', PYRAMID);
  plugin({ theme: 'monochrome' })(tree);
  assert.ok(tree.children[0].value.includes('<svg'));
});

test('respects width option', () => {
  const tree = makeCodeTree('declart', PYRAMID);
  plugin({ width: 400 })(tree);
  assert.ok(tree.children[0].value.includes('<svg'));
});

test('escapes HTML in error messages', () => {
  const tree = makeCodeTree('declart', 'kind = "<script>"');
  plugin()(tree);
  const node = tree.children[0];
  assert.ok(!node.value.includes('<script>'));
});
