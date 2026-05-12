import { visit, SKIP } from 'unist-util-visit';
import { render, renderWithThemeToml } from '@iyulab/declart';

/**
 * remark plugin: transforms ```declart code blocks into inline SVG figures.
 *
 * @param {object} [options]
 * @param {string} [options.theme='default'] - Built-in theme name
 * @param {number} [options.width] - Canvas width in pixels
 * @param {string} [options.themeToml] - Custom TOML theme string (overrides theme)
 */
export default function remarkDeclart(options = {}) {
  const { theme = 'default', width, themeToml } = options;

  return (tree) => {
    visit(tree, 'code', (node, index, parent) => {
      if (node.lang !== 'declart') return;
      if (!parent || index == null) return;

      let htmlNode;
      try {
        const svg = themeToml
          ? renderWithThemeToml(node.value, themeToml, width)
          : render(node.value, theme, width);
        htmlNode = { type: 'html', value: `<figure class="declart">${svg}</figure>` };
      } catch (err) {
        const msg = escapeHtml(String(err?.message ?? err));
        htmlNode = {
          type: 'html',
          value: `<figure class="declart-error"><pre>${msg}</pre></figure>`,
        };
      }

      parent.children.splice(index, 1, htmlNode);
      return [SKIP, index];
    });
  };
}

function escapeHtml(str) {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}
