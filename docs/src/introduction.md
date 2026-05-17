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

> **Vite / ESM 환경 (타입 해석):** `moduleResolution: "bundler"` (Vite 기본값)에서 타입이 자동으로 해결됩니다 (v0.17.2+). 이전 버전 사용 시 `tsconfig.json`에 `"moduleResolution": "node"` 또는 `vite-env.d.ts`에 수동 선언이 필요합니다.
>
> **주의:** 이 패키지는 Node.js 런타임용 WASM 빌드입니다. Vite SSR / Node.js 환경에서 사용 가능합니다. 브라우저 번들에서 직접 `import`하면 `Cannot find module 'fs'` 에러가 발생합니다.

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
| `tier` | `pyramid` (default) |
| `hierarchy` | `org_chart` (auto), `fishbone` (auto), `mind_map` |
| `timeline` | — |
| `matrix` | — |
| `hub_spoke` | — |
| `venn` | — |
| `comparison` | — |
| `state` | — |

## Design Philosophy

See [Principles](principles.md) for the full design rationale. The key idea: declarations express *what* exists, not *how* it looks. The engine owns visual decisions.
