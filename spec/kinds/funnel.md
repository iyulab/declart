# Funnel

A funnel diagram renders a tapered sequence of labeled stages, widest at the top and narrowest at the bottom. It is the natural visual for conversion funnels, sales pipelines, and filtering processes.

## TOML Structure

```toml
kind = "funnel"
title = "Marketing Funnel"   # optional

[[items]]
label = "Awareness"

[[items]]
label = "Interest"

[[items]]
label = "Consideration"

[[items]]
label = "Intent"

[[items]]
label = "Conversion"
```

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | ✅ | Must be `"funnel"` |
| `title` | string | — | Optional diagram title |
| `[[items]]` | array | ✅ | At least 2 labeled stages, from widest (top) to narrowest (bottom) |
| `items[].label` | string | ✅ | Stage display label |
| `items[].emphasis` | string | — | `"primary"` or `"secondary"` |

## Constraints

- At least 2 items are required; maximum 10.
- **10 stages or fewer is recommended.** Beyond 10 stages, lower stages reach the minimum width and cease to narrow, losing the funnel shape.

## Rendering

- Stages are stacked vertically, widest at the top.
- Each stage is a trapezoid (wider top edge, narrower bottom edge).
- The top stage fills the full canvas width (minus padding). Each subsequent stage narrows by a fixed fraction.
- Colors follow the theme gradient from apex (top) to base (bottom).
- `emphasis: primary` adds an outline stroke and bold label.
- `emphasis: secondary` uses a lighter tint.

## Example: Sales Pipeline

```toml
kind = "funnel"
title = "Sales Pipeline"

[[items]]
label = "Leads"
emphasis = "primary"

[[items]]
label = "Qualified"

[[items]]
label = "Proposal"

[[items]]
label = "Negotiation"

[[items]]
label = "Closed Won"
```
