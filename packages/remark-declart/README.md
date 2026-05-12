# remark-declart

[remark](https://github.com/remarkjs/remark) plugin that renders ` ```declart ` code blocks as inline SVG diagrams.

## Install

```sh
npm install remark-declart
```

## Usage

```js
import { remark } from 'remark';
import remarkDeclart from 'remark-declart';
import remarkHtml from 'remark-html';

const result = await remark()
  .use(remarkDeclart)
  .use(remarkHtml, { sanitize: false })
  .process(`
\`\`\`declart
kind = "pyramid"
title = "Maslow's Hierarchy"

[[items]]
label = "Self-Actualization"

[[items]]
label = "Esteem"
emphasis = "primary"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
\`\`\`
`);

console.log(String(result));
// → <figure class="declart"><svg ...>...</svg></figure>
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `theme` | `string` | `'default'` | Built-in theme: `'default'`, `'monochrome'`, `'accessible'`, `'warm'` |
| `width` | `number` | — | Canvas width in pixels (height scales proportionally) |
| `themeToml` | `string` | — | Custom TOML theme string (overrides `theme`) |

```js
remark().use(remarkDeclart, { theme: 'accessible', width: 600 })
```

### Error handling

Invalid diagram declarations produce a `<figure class="declart-error">` element with the error message, so the rest of the document continues to render.

## Frameworks

Works with any remark-based pipeline:

| Framework | Config |
|-----------|--------|
| **Astro** | `astro.config.mjs` → `remarkPlugins: [remarkDeclart]` |
| **Next.js** | `next.config.js` → MDX `remarkPlugins: [remarkDeclart]` |
| **Docusaurus** | `docusaurus.config.js` → `remarkPlugins: [remarkDeclart]` |
| **VitePress** | `config.mts` → `markdown.remarkPlugins: [remarkDeclart]` |

## Supported diagram kinds

`pyramid` · `process` · `cycle` · `matrix` · `hub_spoke` · `venn` · `timeline` · `fishbone` · `org_chart` · `funnel` · `comparison`

## License

MIT © [iyulab](https://github.com/iyulab)
