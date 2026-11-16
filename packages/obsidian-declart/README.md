# Declart for Obsidian

Renders `declart` code blocks as SVG diagrams in Obsidian, the way Mermaid blocks render.
In Live Preview a block shows the diagram by default; use the `</>` ("Edit this block") button
to edit the declaration, and move the cursor out to see the diagram again.

````markdown
```declart
kind = "tier"
title = "Maslow"

[[items]]
label = "Self-actualization"

[[items]]
label = "Esteem"
```
````

TOML and JSON declarations are both supported. A declaration that fails to parse is shown as an
error message inside the block; the rest of the note is unaffected.

## Install (manual)

1. Build the plugin (see below) or take `main.js`, `manifest.json` and `styles.css` from a build.
2. Copy the three files to `<your vault>/.obsidian/plugins/declart/`.
3. In Obsidian, open **Settings → Community plugins** and enable **Declart**.

The WebAssembly renderer is inlined into `main.js`, so no other files are needed. The plugin is
verified on desktop; it does not use desktop-only APIs and is not marked desktop-only, but it has
not been tested on mobile yet.

## Settings

- **Theme** — `Auto` follows Obsidian's light and dark mode (`default` / `dark`); or pick a
  built-in theme. Diagram backgrounds are transparent.
- **Width** — canvas width in pixels; leave empty for automatic.

## Build

Requires Node.js 22.18 or later, [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) and a Rust toolchain.

```bash
npm install
npm run build   # wasm-pack build, then bundle main.js
npm test
```

`npm run build:wasm` rebuilds only the WebAssembly module (into `src/wasm/`).
