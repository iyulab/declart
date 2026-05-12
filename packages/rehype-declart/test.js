import assert from 'node:assert/strict';
import { test } from 'node:test';
import plugin from './index.js';

const PYRAMID_TEXT = `kind = "sequence"
view = "pyramid"
title = "Test"

[[items]]
label = "Top"

[[items]]
label = "Bottom"`;

function makePreCodeTree(className, value) {
  return {
    type: 'root',
    children: [
      {
        type: 'element',
        tagName: 'pre',
        properties: {},
        children: [
          {
            type: 'element',
            tagName: 'code',
            properties: { className: className ? [className] : [] },
            children: [{ type: 'text', value }],
          },
        ],
      },
    ],
  };
}

test('renders language-declart block to SVG figure', () => {
  const tree = makePreCodeTree('language-declart', PYRAMID_TEXT);
  plugin()(tree);
  const node = tree.children[0];
  assert.strictEqual(node.type, 'element');
  assert.strictEqual(node.tagName, 'figure');
  assert.ok(node.properties.className?.includes('declart'));
  // SVG is somewhere in children
  const html = serializeSimple(node);
  assert.ok(html.includes('<svg'));
});

test('ignores non-declart code blocks', () => {
  const tree = makePreCodeTree('language-js', 'console.log(1)');
  plugin()(tree);
  assert.strictEqual(tree.children[0].tagName, 'pre');
});

test('ignores code blocks with no class', () => {
  const tree = makePreCodeTree(null, 'hello');
  plugin()(tree);
  assert.strictEqual(tree.children[0].tagName, 'pre');
});

test('emits error figure on invalid input', () => {
  const tree = makePreCodeTree('language-declart', 'not valid toml @@@@');
  plugin()(tree);
  const node = tree.children[0];
  assert.strictEqual(node.tagName, 'figure');
  assert.ok(node.properties.className?.includes('declart-error'));
});

test('respects theme option', () => {
  const tree = makePreCodeTree('language-declart', PYRAMID_TEXT);
  plugin({ theme: 'accessible' })(tree);
  const html = serializeSimple(tree.children[0]);
  assert.ok(html.includes('<svg'));
});

test('respects width option', () => {
  const tree = makePreCodeTree('language-declart', PYRAMID_TEXT);
  plugin({ width: 300 })(tree);
  const html = serializeSimple(tree.children[0]);
  assert.ok(html.includes('<svg'));
});

// Minimal hast serializer for test assertions
function serializeSimple(node) {
  if (node.type === 'text') return node.value;
  if (node.type === 'raw') return node.value;
  if (node.type !== 'element') return '';
  const attrs = Object.entries(node.properties ?? {})
    .map(([k, v]) => ` ${k}="${Array.isArray(v) ? v.join(' ') : v}"`)
    .join('');
  const inner = (node.children ?? []).map(serializeSimple).join('');
  return `<${node.tagName}${attrs}>${inner}</${node.tagName}>`;
}
