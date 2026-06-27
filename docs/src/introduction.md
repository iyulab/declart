# Declart

**Declare what to show. The engine decides how it looks.**

Declart is a declarative diagram engine. You write a TOML file describing the structure of your diagram, and Declart renders it to SVG. No layout coordinates. No styling choices. Just content.

## Quick Start

### CLI

```bash
# Install
cargo install declart-cli

# Scaffold a starter diagram
declart init flow > diagram.toml

# Render to SVG
declart render diagram.toml

# Render to PNG
declart render diagram.toml --format png

# Watch and auto-rebuild on changes
declart watch diagram.toml

# Validate without rendering
declart validate diagram.toml
```

### Node.js

```js
const declart = require('@iyulab/declart');

const svg = declart.render(`
kind = "flow"
view = "cycle"
title = "PDCA"
[[items]]
label = "Plan"
[[items]]
label = "Do"
`);
```

> **Vite / ESM (type resolution):** With `moduleResolution: "bundler"` (Vite default), types resolve automatically (v0.18.0+). For older setups add `"moduleResolution": "node"` to `tsconfig.json` or declare types manually in `vite-env.d.ts`.
>
> **Note:** This package is a Node.js-runtime WASM build. It works in Vite SSR / Node.js environments. Importing it directly in a browser bundle will throw `Cannot find module 'fs'`.

### Rust (library)

```rust
use declart_core::{parse, render};
use declart_core::render::DEFAULT_THEME;

let diagram = parse(input)?;
let svg = render(&diagram, &DEFAULT_THEME)?;
```

## Supported Kinds

| Kind | Views |
|------|-------|
| `flow` | `process` (default), `cycle`, `funnel`, `swimlane` |
| `tier` | `pyramid` (default), `concentric` |
| `hierarchy` | `org_chart` (auto), `fishbone` (auto), `mind_map` |
| `timeline` | — |
| `matrix` | — |
| `hub_spoke` | — |
| `venn` | — |
| `comparison` | — |
| `state` | — |

## Semantic item attributes

Items carry *meaning*, not styling. Two optional attributes are shared across kinds:

- **`emphasis`** — `primary` / `secondary` (importance).
- **`status`** — `success` / `normal` / `warning` / `critical` (health signal). Rendered as a corner
  marker dual-encoded by **color and shape** (circle / triangle / diamond) so it reads in monochrome
  and for colorblind readers. Supported on `flow`, `tier`, `hub_spoke`, and `matrix` items.

`matrix` additionally supports classifying items into quadrants (BCG / Gartner style) via an optional
`[[items]]` array with a `quadrant` position. See the [schema](schema.md) and per-kind pages.

## Design Philosophy

See [Principles](principles.md) for the full design rationale. The key idea: declarations express *what* exists, not *how* it looks. The engine owns visual decisions.
