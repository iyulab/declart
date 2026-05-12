# rehype-declart

[rehype](https://github.com/rehypejs/rehype) plugin that renders `<pre><code class="language-declart">` blocks as inline SVG diagrams.

## Install

```sh
npm install rehype-declart
```

## Usage

```js
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeDeclart from 'rehype-declart';
import rehypeStringify from 'rehype-stringify';

const result = await unified()
  .use(remarkParse)
  .use(remarkRehype)
  .use(rehypeDeclart)
  .use(rehypeStringify)
  .process(`
\`\`\`declart
kind = "sequence"
title = "Release Pipeline"

[[items]]
label = "Build"

[[items]]
label = "Test"
emphasis = "primary"

[[items]]
label = "Deploy"
\`\`\`
`);

console.log(String(result));
// → <figure class="declart"><svg ...>...</svg></figure>
```

> **Note**: If you are using a remark-based pipeline, prefer [`remark-declart`](https://www.npmjs.com/package/remark-declart) which operates at the Markdown AST level.

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `theme` | `string` | `'default'` | Built-in theme: `'default'`, `'monochrome'`, `'accessible'`, `'warm'` |
| `width` | `number` | — | Canvas width in pixels (height scales proportionally) |
| `themeToml` | `string` | — | Custom TOML theme string (overrides `theme`) |

### Error handling

Invalid diagram declarations produce a `<figure class="declart-error">` element with the error message, so the rest of the document continues to render.

## Supported diagram kinds

`pyramid` · `process` · `cycle` · `matrix` · `hub_spoke` · `venn` · `timeline` · `fishbone` · `org_chart` · `funnel` · `comparison`

## License

MIT © [iyulab](https://github.com/iyulab)
