# Declart

> Declare what to show. The engine decides how it looks.

Declart is a declarative diagram library for **business and conceptual visuals** — the kind found in PowerPoint SmartArt, consulting decks, and strategy documents. You describe *what data belongs to which kind of diagram*. The engine handles layout, typography, spacing, and styling.

## Why

Existing diagram-as-code tools — Mermaid, D2, PlantUML, Graphviz — excel at **engineering diagrams**: flowcharts, sequence diagrams, ER diagrams, state machines. They are weak or absent for **business and conceptual diagrams** such as pyramids, 2×2 matrices, fishbone, PDCA cycles, hub-and-spoke, Venn, and timelines.

These diagrams remain trapped inside binary PowerPoint files (SmartArt), unreachable by Git, code review, automation, and LLM-driven workflows. Declart fills that gap.

## Principles

These are anchors. Every future design decision must be consistent with them.

1. **Data, not pixels.** Declarations contain only data and the kind of diagram. No colors, coordinates, dimensions, fonts, or visual styles in source files.
2. **Semantic over visual.** Meaning belongs in the source (`status: critical`, `emphasis: primary`). Visual mapping belongs in the engine.
3. **Beauty is the engine's responsibility.** Users should never need design skill to produce a publishable diagram.
4. **LLM-first.** The declaration format must be the simplest possible thing a language model can generate correctly and a human can review at a glance.
5. **One way to describe each kind.** Avoid configurability that lets users encode the same idea in many different shapes.

## Scope

### Diagram kinds (in scope)

Anchored on the categories PowerPoint SmartArt established, prioritizing what existing diagram-as-code tools leave uncovered.

- Process — linear steps
- Cycle — PDCA, lifecycle loops
- Pyramid
- Matrix 2×2
- Hub-and-Spoke — radial relationships
- Venn — set intersections
- Timeline — date-anchored events
- Fishbone / Ishikawa — cause-and-effect
- Org Chart — hierarchical tree of nodes
- Funnel — tapered conversion stages (marketing/sales)

Additional kinds (Roadmap, Swimlane, etc.) may be considered after the core kinds stabilize.

### Output

- **SVG is the primary output.** Vector-first, embeddable in documents, web, presentations, and PDFs.
- Other formats (PNG, PDF rasterization) are downstream concerns, not part of the core engine.

### Distribution

- **CLI** — `declart-cli` binary
- **Rust library** — `declart-core` crate (parse + render API)
- **WebAssembly** — `declart-wasm` (wasm-pack, browser + Node.js)
- **C ABI** — `declart-ffi` (shared library + `declart.h` header, for P/Invoke and ctypes callers)
- **Node.js** — `@iyulab/declart` npm package (WASM wrapper, CommonJS + TypeScript types)

## Non-goals

Explicit boundaries that protect focus:

- **Not a free-form drawing tool.** Declart does not compete with Mermaid, D2, PlantUML, or Graphviz for arbitrary node-edge graphs.
- **Not for engineering diagrams.** Flowcharts, sequence diagrams, ER diagrams, and state machines are well-served elsewhere and are not on the roadmap.
- **No pixel-level control.** No positions, no per-element colors, no font overrides in source.
- **Not interactive.** Declart produces static visuals. Animation and interactivity are out of scope.
- **No WYSIWYG editing.** Declart is source-first. Visual editors are not part of the project.
- **No PowerPoint round-trip.** Declart outputs embeddable SVG but does not read or write `.pptx` files.

## Status

**Current: v0.11.0** — 10 diagram kinds, 4+custom themes, CLI/WASM/FFI/Node.js, interactive playground, LLM guide. 123 tests.

| Capability | State |
|------------|-------|
| 10 diagram kinds (pyramid → funnel) | ✅ |
| 4 built-in themes + user-defined TOML themes | ✅ |
| CLI (`render`, `validate`, `init`, `watch`, `--format png`) | ✅ |
| WASM bindings (`declart-wasm`) | ✅ |
| C ABI (`declart-ffi` + `declart.h`) | ✅ |
| Node.js package (`@iyulab/declart`) | ✅ |
| Interactive playground (live WASM, URL permalink, zoom/pan) | ✅ |
| Spec site (mdBook + GitHub Actions CI) | ✅ (GitHub Pages: enable in repo Settings) |
| LLM guide (prompt templates for all 10 kinds) | ✅ |

See [CHANGELOG.md](CHANGELOG.md) for version history.

**This README is the design anchor.** All future implementation decisions must remain consistent with the principles, scope, and non-goals stated above. Changes to this document require deliberate revision, not drift.

## Install

```bash
cargo install --git https://github.com/iyulab/declart
```

Or build locally:

```bash
git clone https://github.com/iyulab/declart
cd declart
cargo build --release
# binary at target/release/declart
```

## Usage

Create a declaration file:

```toml
# hierarchy.toml
kind = "pyramid"
title = "Maslow's Hierarchy of Needs"

[[items]]
label = "Self-actualization"

[[items]]
label = "Esteem"

[[items]]
label = "Love & Belonging"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
```

Render to SVG:

```bash
declart render hierarchy.toml
# writes hierarchy.svg
```

Validate without rendering:

```bash
declart validate hierarchy.toml
```

Export to PNG:

```bash
declart render hierarchy.toml --format png
# writes hierarchy.png
```

Watch and auto-rebuild on changes:

```bash
declart watch hierarchy.toml
# watches hierarchy.toml, rewrites hierarchy.svg on every save
# use --format png for PNG output
```

Pipe to stdout:

```bash
declart render hierarchy.toml --stdout > diagram.svg
```

## License

MIT

---

Built by [iyulab](https://github.com/iyulab).