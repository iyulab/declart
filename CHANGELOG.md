# Changelog

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
