# Declart

**Declare what to show. The engine decides how it looks.**

Declart is a declarative diagram engine. You write a TOML file describing the structure of your diagram, and Declart renders it to SVG. No layout coordinates. No styling choices. Just content.

## Quick Start

### CLI

```bash
# Install
cargo install declart-cli

# Scaffold a starter diagram
declart init pyramid > diagram.toml

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
kind = "pyramid"
title = "Maslow's Hierarchy"
[[items]]
label = "Self-actualization"
[[items]]
label = "Safety"
`);
```

### Rust (library)

```rust
use declart_core::{parse, render};
use declart_core::render::DEFAULT_THEME;

let diagram = parse(input)?;
let svg = render(&diagram, &DEFAULT_THEME)?;
```

## Supported Kinds

| Kind | Use case |
|------|----------|
| `pyramid` | Hierarchies, Maslow, priority layers |
| `process` | Sequential steps, workflows |
| `cycle` | Closed loops, PDCA, lifecycles |
| `matrix` | 2×2 prioritization, Eisenhower |
| `hub_spoke` | Central concept with related items |
| `venn` | Set intersections, overlapping groups |
| `timeline` | Date-anchored events |
| `fishbone` | Cause-and-effect, root cause analysis |
| `org_chart` | Hierarchical trees, organizational structures |
| `funnel` | Conversion funnels, sales pipelines |

## Design Philosophy

See [Principles](principles.md) for the full design rationale. The key idea: declarations express *what* exists, not *how* it looks. The engine owns visual decisions.
