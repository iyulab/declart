# Declart

[![CI](https://github.com/iyulab/declart/actions/workflows/ci.yml/badge.svg)](https://github.com/iyulab/declart/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@iyulab/declart)](https://www.npmjs.com/package/@iyulab/declart)
[![Crates.io](https://img.shields.io/crates/v/declart-core)](https://crates.io/crates/declart-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Prose diagrams for written content. Declare what to show — the engine decides how it looks.

Declart is a **prose diagram** library: diagrams that belong in blog posts, reports, documentation, and slide decks — not in codebases. You describe the structure. The engine handles layout, typography, spacing, and styling.

## Why Declart

**Mermaid for engineers. Declart for writers.**

If you need technical diagrams — ERD, sequence, class diagrams, state machines — use Mermaid. It handles those better.

Declart solves a different problem: the diagram you sketch on a whiteboard to explain an idea in a blog post, report, or slide deck. The kind that should take seconds to write and look consistent without any design decisions.

Today, that kind of diagram is stuck in two places:

- **PowerPoint SmartArt** — binary files, unreachable by Git, automation, and LLM workflows
- **Design tools (Canva, Figma)** — require design skill, break reproducibility

Declart makes prose diagrams reproducible, version-controlled, and LLM-generatable. We deliberately stop where comprehension stops: no edge labels, no arbitrary graphs, no pixel control. Complexity that exceeds a reader's comprehension is out of scope by design.

## Principles

These are anchors. Every future design decision must be consistent with them.

1. **Data, not pixels.** Declarations contain only data and the kind of diagram. No colors, coordinates, dimensions, fonts, or visual styles in source files.
2. **Semantic over visual.** Meaning belongs in the source (`status: critical`, `emphasis: primary`). Visual mapping belongs in the engine.
3. **Beauty is the engine's responsibility.** Users should never need design skill to produce a publishable diagram.
4. **LLM-first.** The declaration format must be the simplest possible thing a language model can generate correctly and a human can review at a glance.
5. **One representation per diagram.** Each `kind + view` pair has exactly one valid schema. No aliases, structural variants, or flags that change interpretation. Two declarations expressing the same diagram are byte-for-byte identical except for whitespace.

## Kind and View

Declart organizes declarations on two axes.

**Kind** is the *data contract*. It defines which fields exist, what they mean, and how they are structured. Two declarations belong to the same kind if and only if their schemas are structurally identical — same fields, same types, same constraints.

**View** is the *semantic intent*. It declares how the engine should interpret and render the data within its kind. View is optional: when omitted, the engine selects the most appropriate rendering automatically.

```toml
kind = "flow"     # data contract: items that pass through stages
view = "cycle"    # intent: interpret this flow as a repeating loop

[[items]]
label = "Plan"
```

`view = "cycle"` is a semantic declaration, not a visual instruction. It means "this flow is intended to be understood as a closed loop." The engine decides what that looks like. View values occupy the same space as `emphasis = "primary"`: they express *meaning*, not *appearance*.

### Naming principle

> **Kind names describe the relationship between items — not their visual form.**

The test: *"What is the relationship between these items?"* If the answer changes, the kind changes. If only the visual changes, the view changes.

| Kind | Relationship | Decision question |
|------|-------------|-------------------|
| `flow` | Sequential stages | Do items pass through stages? |
| `tier` | Ranked levels | Are items ranked by importance or abstraction? |
| `hierarchy` | Parent-child tree | Do items have explicit parent references? |
| `timeline` | Temporal position | Do items have dates? |
| `matrix` | Two-axis position | Are items placed in a 2×2 grid? |
| `hub_spoke` | Radial connection | Do items radiate from a center? |
| `venn` | Set intersection | Are items overlapping groups? |
| `comparison` | Criteria evaluation | Are items evaluated against columns? |

### Decision rule

> **New kind** when the data structure is genuinely different from all existing kinds and cannot be expressed as a view of one.
> **New view** when the kind's existing schema is sufficient and only the rendering intent differs.

This is the boundary that prevents kind proliferation. Layout variants, visual alternatives, and semantic reframings of the same data are always new views, never new kinds.

### Criteria: adding a new kind

1. The data structure is structurally distinct from all existing kinds.
2. The difference cannot be bridged by a view of an existing kind.
3. Multiple concrete use cases justify the new schema.

### Criteria: adding a new view

1. The view name expresses semantic intent, not visual description (`"cycle"`, not `"circular"`).
2. The kind's existing schema is sufficient — no new fields required.
3. The engine produces a meaningfully distinct rendering.

---

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

- **Not for engineering diagrams.** Flowcharts, sequence diagrams, ER diagrams, state machines — use Mermaid, D2, or PlantUML. Declart is for prose, not code.
- **Not a free-form drawing tool.** No arbitrary node-edge graphs. Declart renders declared structure, not drawn shapes.
- **No pixel-level control.** No positions, no per-element colors, no font overrides in source.
- **Not interactive.** Declart produces static visuals. Animation and interactivity are out of scope.
- **No WYSIWYG editing.** Declart is source-first. Visual editors are not part of the project.
- **No PowerPoint round-trip.** Declart outputs embeddable SVG but does not read or write `.pptx` files.

## Status

**Current: v0.16.0** — kind+view 2단계 아키텍처 (11 kinds → 7 kinds + view layer). 152 tests.

| Capability | State |
|------------|-------|
| 7 diagram kinds (`sequence`, `hierarchy`, `timeline`, `matrix`, `hub_spoke`, `venn`, `comparison`) + view layer | ✅ |
| 4 built-in themes + user-defined TOML themes | ✅ |
| TOML + JSON input (`parse_auto`) | ✅ |
| CLI (`render`, `validate`, `init`, `watch`, `--format png`) | ✅ |
| WASM bindings (`render`, `render_json`, `render_with_theme_toml`) | ✅ |
| C ABI (`declart-ffi` + `declart.h`) | ✅ |
| Node.js package (`@iyulab/declart`, CJS + ESM) | ✅ |
| GitHub Releases binaries (Linux musl, macOS, Windows) | ✅ |
| Interactive playground (live WASM, URL permalink, zoom/pan) | ✅ |
| Spec site (mdBook + GitHub Actions CI) | ✅ (GitHub Pages: enable in repo Settings) |
| LLM guide (prompt templates for all 7 kinds) | ✅ |
| VS Code Extension (live preview, diagnostics, Markdown code blocks) | ✅ (.vsix) |
| mdbook-declart preprocessor (inline SVG in mdBook) | ✅ |

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