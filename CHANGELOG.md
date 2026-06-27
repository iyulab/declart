# Changelog

## v0.19.0

Common `status` semantic attribute — qualitative "traffic-light" health signal for reports.

- **`status` item field** (new): `success` / `normal` / `warning` / `critical`. Orthogonal to `emphasis` — an item can be both `primary` and `critical`. Supported on `flow` (process/cycle/funnel/swimlane), `tier`, `hub_spoke`, and `matrix` (quadrants). Declaring it on other kinds is a parse error.
- **Dual-encoded marker**: rendered as a small corner marker encoded by **both color and shape** (success=circle, warning=triangle, critical=diamond) so it stays readable in monochrome and under color-vision deficiency. `normal` and omitted `status` render no marker. The marker is visually independent of `emphasis`.
- **Theme status palettes**: all four built-in themes define a status palette; `accessible` uses colorblind-safe Okabe-Ito colors. Custom TOML themes accept an optional `[status]` section (falls back to the default palette).
- **Docs & playground**: `spec/schema.md`, `spec/kinds/{flow,tier,hub_spoke,matrix}.md`, the LLM guide, and a new playground example all cover `status`.
- **Matrix item placement** (new): `matrix` gains an optional `[[items]]` array to classify items into quadrants (BCG growth-share, Gartner Magic Quadrant). Each item declares `quadrant` by position (`top-left`/`top-right`/`bottom-left`/`bottom-right`) and may carry `emphasis`/`status`. Assignment is by category, never coordinates. Max 6 items per quadrant. Label-only matrices are unchanged (backward compatible); when a quadrant has items its label renders as a header.
- **Code quality**: resolved the remaining `too_many_arguments` clippy warning in `mind_map.rs`.
- **Tests**: 177 core unit + 20 spec-suite pass; clippy clean.

## v0.17.1

Documentation and code quality improvements:

- **`hierarchy` spec docs**: Added `id` and `effect` fields to field tables in `spec/kinds/hierarchy.md` and `docs/src/kinds/hierarchy.md`. Added org_chart-with-id example. Updated fishbone view description to reference `effect` field (with `title` fallback).
- **LLM guide**: Updated fishbone, org chart, and Tips table to reflect `effect` field and `id`-based `parent` references.
- **Playground**: Fishbone example now includes `effect = "High Latency"`.
- **Code quality**: Resolved 6 clippy warnings in `declart-core` (dead code, range check, sort_by_key, self_convention).

## v0.17.0

Design integrity release — naming correctness, id-based references, fishbone effect field, CLI warnings.

- **`sequence` → `flow`** (breaking): `kind = "sequence"` renamed to `kind = "flow"`. Using `kind = "sequence"` now produces a migration hint: `` `sequence` was renamed to `flow` in v0.17 ``.
- **`tier` kind** (new): `kind = "tier"` + `view = "pyramid"` for ranked/layered diagrams. Previously `sequence + view = "pyramid"` — now semantically correct. Using `kind = "flow" + view = "pyramid"` gives a migration hint.
- **Hierarchy `id` field**: `HierarchyNode` gains optional `id: Option<String>`. Set `id = "cto"` to allow `parent = "cto"` references that survive label renames. Backward compatible — label-based parent still works.
- **Fishbone `effect` field**: `HierarchyDiagram` gains `effect: Option<String>`. When set, used as the spine-end label instead of `title`. Separates chart title from the effect being analyzed.
- **Hierarchy auto-select CLI warning**: `declart render`/`declart validate` emits `warning: hierarchy view auto-selected: org_chart (1 root node)` + hint when `view` is omitted. Rendering proceeds normally.
- **Kind taxonomy** (8 kinds): `flow`, `tier`, `hierarchy`, `timeline`, `matrix`, `hub_spoke`, `venn`, `comparison`.
- **Tests**: 167 pass.

## v0.16.0

kind+view 2단계 아키텍처 — 11 kinds → 7 kinds + view layer.

- **kind+view architecture**: `kind` (data contract) + `view` (semantic declaration) 2-layer design. Kind determines field structure; view declares meaning, engine decides visuals.
- **11 → 7 kinds**: `sequence` absorbs process/cycle/funnel/pyramid views; `hierarchy` absorbs org_chart/fishbone views. Remaining 5 kinds unchanged.
- **`view` field**: optional semantic declaration. Omit to let the engine auto-select (sequence → `process`; hierarchy → `org_chart` when root=1, `fishbone` when root≥2).
- **`declart init <kind>`**: updated templates use new kind names.
- **Spec suite**: rewritten for new kind+view structure (152 tests).
- **Downstream crates**: declart-wasm, declart-ffi, mdbook-declart, declart-cli — all updated.
- **Spec docs**: sequence.md + hierarchy.md added; old kind-specific docs removed.

## v0.15.1

Patch: fishbone sub-item overlap fix + npm publish CI improvements.

- **Fishbone sub-item layout**: sub-item labels no longer overlap cause boxes. Sub-branches now attach to the lower 50% of the cause branch and extend horizontally; labels are positioned above/below the sub-branch line rather than in the cause box area.
- **npm publish CI**: fixed `--out-dir` absolute path for wasm-pack, added skip-if-already-published guard for all three packages.

## v0.15.0

Markdown ecosystem plugins — remark/rehype integration for Astro, Next.js, Docusaurus, MDX, VitePress.

- **`remark-declart`** (`packages/remark-declart`): remark plugin for Markdown pipelines
  - Transforms ` ```declart ` code blocks to inline SVG figures at build time
  - Options: `theme`, `width`, `themeToml`
  - Error-resilient: invalid declarations emit `<figure class="declart-error">` instead of crashing the build
  - Works with Astro, Next.js MDX, Docusaurus, VitePress, and any remark-based pipeline
- **`rehype-declart`** (`packages/rehype-declart`): rehype plugin for HTML AST pipelines
  - Transforms `<pre><code class="language-declart">` blocks to inline SVG figures
  - Uses `hast-util-from-html` for proper hast integration (no `rehype-raw` required)
  - Same options as `remark-declart`
- **npm publish CI** (`.github/workflows/publish-npm.yml`): publishes `@iyulab/declart`, `remark-declart`, `rehype-declart` on tag push
- **Spec pages**: all 11 diagram kind pages now use ` ```declart ` examples — rendered as live SVG via `mdbook-declart` in the spec site
- **151 tests** (unchanged — remark/rehype tests use node:test, separate from Cargo workspace)

## v0.14.0

VS Code Extension + mdbook-declart preprocessor — editor integration and Markdown ecosystem support.

- **VS Code Extension** (`packages/vscode-declart`): new extension for VS Code 1.90+
  - Side-by-side SVG preview: click ⊞ in editor title bar (or **Declart: Open Preview to the Side**)
  - Preview follows the active Declart file automatically
  - Live re-render as you type (no save required)
  - Theme toolbar: default / monochrome / accessible / warm
  - Inline diagnostics: parse errors underlined in editor with line/column info
  - Markdown integration: ` ```declart ` code blocks render as inline SVG in VS Code Markdown Preview
  - Install via `.vsix` (GitHub Releases); Marketplace coming after first user confirmation
- **mdbook-declart** (`crates/mdbook-declart`): mdBook preprocessor that renders ` ```declart ` code blocks to inline SVG at build time. No JS required at read time.
  - Spec site (`docs/`) integrated: `[preprocessor.declart]` in `book.toml`
  - `docs/build.ps1` + `docs/build.sh` + `spec-site.yml` CI updated
- **151 tests** (+5 mdbook-declart unit tests)

## v0.13.0

Input simplification — three breaking changes that reduce declaration verbosity.

- **Timeline partial dates**: `date` now accepts `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. Partial forms are treated as the first day of that year/month. Existing `YYYY-MM-DD` files are unchanged.
- **Org Chart label-as-id**: `id` field removed from `[[nodes]]`. `label` is now the unique node identifier; `parent` references the parent node's `label` directly. All existing `id = "..."` fields must be removed.
- **Comparison row-inline cells**: `[[cells]]` array removed. Cell values are now declared inline within each `[[rows]]` entry, keyed by column label (e.g., `Performance = "★★★★"`). `[[columns]]` is still required for column ordering. Column label must not be `"label"`.
- **146 tests**

## v0.12.0

Ecosystem expansion: distribution automation, JSON input, ESM npm package, Comparison table kind.

- **GitHub Releases** (`release.yml`): cross-platform binaries for Linux musl, macOS (x86_64 + arm64), Windows — tag a `v*` release to trigger
- **install.sh / install.ps1**: one-line install scripts for each platform
- **cargo-binstall**: `[package.metadata.binstall]` manifest for `cargo binstall declart`
- **JSON input** (`parse_json`, `parse_auto`): accepts JSON equivalents of all TOML declarations; CLI and WASM auto-detect format
- **WASM `render_json()`**: explicit JSON rendering entry point
- **`@iyulab/declart` ESM**: `exports.import` → `index.mjs`; `renderJson`, `renderWithThemeToml` exposed
- **Comparison table kind** (`kind = "comparison"`): rows × columns grid with optional cell values; 1–10 rows, 1–8 columns
- **Spec JSON examples**: `valid/basic.json` for all 11 kinds
- **141 tests**

## v0.11.0

Visualization quality hardening (Phase 11-A/B) + LLM-friendly documentation.

- **Fishbone**: causes > 20 → validation error; ≤8 causes recommended in spec
- **Funnel**: items > 10 → validation error
- **Timeline**: label available width now scales with event density (was hardcoded 100px)
- **Matrix**: x/y axis labels scale down and truncate on overflow
- **LLM guide**: `docs/src/llm-guide.md` — 10 kinds × prompt templates, validate workflow, tips
- **Spec suite**: invalid examples for fishbone/funnel density limits added
- **Docs**: `introduction.md` now lists all 10 kinds; SUMMARY.md links to LLM guide

## v0.10.0

User-defined TOML themes + rendering quality pass.

- **`Theme::from_toml()`**: declare custom themes in TOML (`[colors]` + optional `[typography]`)
- **CLI `--theme file.toml`**: load a custom theme file at render time
- **WASM `render_with_theme_toml()`**: browser rendering with custom theme string
- **Playground**: Custom theme TOML editor panel
- **`truncate_text()`**: integrated into all 10 renderers (cycle, timeline, fishbone, hub_spoke)
- **`is_dark()` threshold**: 140 → 128 (WCAG AA)
- **Org Chart centering**: asymmetric subtree centering fix
- **spec/themes/**: valid (corporate, minimal) + invalid (missing_apex, bad_hex, short_hex) suite

## v0.9.0

Two new diagram kinds + four themes + rendering improvements.

- **Org Chart kind**: hierarchical tree, flat TOML with `parent` references, elbow connectors
- **Funnel kind**: tapered trapezoid stages, marketing/sales pipelines
- **accessible theme**: Okabe-Ito Blue palette, color-blind safe
- **warm theme**: terracotta gradient for consulting decks
- **`truncate_text()` helper**: ellipsis for over-width labels (Process, Funnel)
- **Hub-Spoke**: min 2 spokes validation
- **Phase 8-C research**: theme system design + expressiveness levers documented

## v0.8.0

Renderer quality improvements + Playground UX.

- **Fishbone**: fish head arrowhead + 45° diagonal sub-item branches
- **Matrix**: `position` field for explicit quadrant mapping (top-left/right, bottom-left/right)
- **Timeline**: dynamic canvas width (n × 55px minimum spacing)
- **label_size_min**: 10px minimum (WCAG accessibility)
- **Playground URL permalink**: base64 `?d=` parameter for sharing
- **Playground Header/Footer**: branding, GitHub, Docs, license links
- **Playground Preview zoom/pan**: wheel zoom, drag pan, fit-to-view

## v0.7.0

Playground v2 + rendering bug fixes.

- **Hub-Spoke / Cycle**: connector rectangle clipping (no gap between arrow and node)
- **Pyramid apex label**: leader line for narrow apex layers; dynamic canvas expansion
- **Matrix**: Low/High axis direction indicators
- **Playground**: draft auto-save (localStorage), SVG export button, error line numbers, drag resize

## v0.6.0 / v0.6.1

PNG export + watch mode + rendering fixes.

- **PNG export**: `declart render --format png` via `resvg`
- **Watch mode**: `declart watch diagram.toml` — auto-rebuild on save
- **Hub-Spoke / Cycle connector fix**: `rect_clip()` for accurate edge start/end points

## v0.5.0 / v0.5.1

Spec site + WASM + bindings + Playground.

- **Spec site**: mdBook skeleton, GitHub Actions CI (`spec-site.yml`)
- **`declart-wasm`**: wasm-pack WASM bindings (render, validate, kinds, themes)
- **`declart-ffi`**: C ABI shared library + `declart.h` header
- **`@iyulab/declart`**: Node.js npm package (CommonJS + TypeScript types)
- **Interactive Playground**: all 8 original kinds, live WASM rendering, theme toggle

## v0.4.0

Rendering polish + DX.

- **Emphasis rendering**: `emphasis: primary/secondary` for all 8 kinds
- **`declart init <kind>`**: scaffold starter TOML for any kind
- **Monochrome theme**: second built-in theme; `--theme` CLI flag
- **`--width` flag**: viewBox-preserving proportional scale
- **Improved error locations**: TOML parse errors surface line/column numbers

## v0.3.0

Four relationship diagram kinds.

- Hub-and-Spoke, Venn, Timeline, Fishbone / Ishikawa

## v0.2.0

Three process/structural kinds.

- Process (sequential steps), Cycle (closed loop), Matrix 2×2

## v0.1.0

Foundation.

- Pyramid renderer, TOML parse pipeline, spec suite infrastructure, default theme
