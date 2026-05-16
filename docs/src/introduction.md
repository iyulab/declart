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

> **Vite / ESM 환경:** `moduleResolution: "bundler"` (Vite 기본값)에서 타입이 자동으로 해결됩니다 (v0.17.2+). 이전 버전 사용 시 `tsconfig.json`에 `"moduleResolution": "node"` 또는 `vite-env.d.ts`에 수동 선언이 필요합니다.
>
> WASM 번들(~1.3 MB) 로드를 Vite가 최적화하도록 하려면:
> ```ts
> // vite.config.ts
> export default defineConfig({
>   optimizeDeps: { include: ['@iyulab/declart'] }
> });
> ```

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
| `hierarchy` | `org_chart` (auto), `fishbone` (auto) |
| `timeline` | — |
| `matrix` | — |
| `hub_spoke` | — |
| `venn` | — |
| `comparison` | — |
| `state` | — |

## Design Philosophy

See [Principles](principles.md) for the full design rationale. The key idea: declarations express *what* exists, not *how* it looks. The engine owns visual decisions.
