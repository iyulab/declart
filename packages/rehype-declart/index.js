import { visit, SKIP } from 'unist-util-visit';
import { fromHtml } from 'hast-util-from-html';
import { render, renderWithThemeToml } from '@iyulab/declart';

/**
 * rehype plugin: transforms <pre><code class="language-declart"> blocks into SVG figures.
 *
 * @param {object} [options]
 * @param {string} [options.theme='default'] - Built-in theme name
 * @param {number} [options.width] - Canvas width in pixels
 * @param {string} [options.themeToml] - Custom TOML theme string (overrides theme)
 */
export default function rehypeDeclart(options = {}) {
  const { theme = 'default', width, themeToml } = options;

  return (tree) => {
    visit(tree, 'element', (node, index, parent) => {
      if (node.tagName !== 'pre') return;
      if (!parent || index == null) return;

      const codeNode = node.children.find(
        (child) =>
          child.type === 'element' &&
          child.tagName === 'code' &&
          Array.isArray(child.properties?.className) &&
          child.properties.className.includes('language-declart'),
      );
      if (!codeNode) return;

      const text = extractText(codeNode);

      let figureHtml;
      try {
        const svg = themeToml
          ? renderWithThemeToml(text, themeToml, width)
          : render(text, theme, width);
        figureHtml = `<figure class="declart">${svg}</figure>`;
      } catch (err) {
        const msg = escapeHtml(String(err?.message ?? err));
        figureHtml = `<figure class="declart-error"><pre>${msg}</pre></figure>`;
      }

      const fragment = fromHtml(figureHtml, { fragment: true });
      parent.children.splice(index, 1, ...fragment.children);
      return [SKIP, index];
    });
  };
}

function extractText(node) {
  let text = '';
  for (const child of node.children ?? []) {
    if (child.type === 'text') text += child.value;
    else if (child.type === 'element') text += extractText(child);
  }
  return text;
}

function escapeHtml(str) {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}
