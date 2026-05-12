# Declart — VS Code Extension

Live diagram preview for [Declart](https://github.com/iyulab/declart) TOML/JSON files.

## Features

- **Side-by-side preview**: Open any `.toml` or `.json` Declart file and click the preview icon in the editor title bar (or run **Declart: Open Preview to the Side**).
- **Live updates**: The preview re-renders as you type.
- **Theme switching**: Choose from `default`, `monochrome`, `accessible`, or `warm` via the toolbar or `declart.theme` setting.
- **Inline diagnostics**: Parse errors are shown as red underlines in the editor.
- **Markdown preview**: `declart` code blocks in `.md` files render as inline diagrams.

## Supported diagram kinds

`pyramid` · `process` · `cycle` · `matrix` · `hub_spoke` · `venn` · `timeline` · `fishbone` · `org_chart` · `funnel` · `comparison`

## Usage

### TOML file preview

```toml
kind = "pyramid"
title = "Maslow's Hierarchy"

[[items]]
label = "Self-Actualization"

[[items]]
label = "Esteem"
emphasis = "primary"

[[items]]
label = "Love & Belonging"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
```

Open the file and press the **⊞ Preview** button in the editor title bar.

### Markdown code blocks

````markdown
```declart
kind = "process"
title = "CI/CD Pipeline"

[[items]]
label = "Build"

[[items]]
label = "Test"
emphasis = "primary"

[[items]]
label = "Deploy"
```
````

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `declart.theme` | `"default"` | Diagram rendering theme |

## Installation

This extension is not yet published to the VS Code Marketplace.

**Install from `.vsix`:**

1. Download `declart-0.14.0.vsix` from the [GitHub Releases](https://github.com/iyulab/declart/releases) page.
2. In VS Code, open the Extensions view (`Ctrl+Shift+X`).
3. Click **···** → **Install from VSIX…** and select the downloaded file.

## Requirements

None — the diagram engine is bundled as WebAssembly.

## License

MIT
