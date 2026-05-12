# Comparison Table Kind — Specification

## Purpose

Presents a matrix of items (rows) evaluated against criteria (columns). Each cell can hold an optional text value (rating, label, or metric).

## TOML Schema

```
kind = "comparison"
title = <string>?         # optional diagram title

[[rows]]
label = <string>          # item being compared (required, unique)

[[columns]]
label = <string>          # evaluation criterion (required, unique)

[[cells]]
row    = <string>         # references a row label (required)
column = <string>         # references a column label (required)
value  = <string>         # cell display value (optional, default = "")
```

### Constraints

| Constraint | Rule |
|-----------|------|
| Rows | 1–10 |
| Columns | 1–8 |
| Cell row reference | Must match an existing `rows[].label` |
| Cell column reference | Must match an existing `columns[].label` |
| Missing cells | Treated as empty (allowed) |

## Rendering Specification

- **Canvas**: 800px wide, height = title_area + 45px column header + rows × 50px + 20px padding
- **Row header column**: 180px wide, apex-tinted background
- **Column header row**: 45px tall, apex color background, bold white/dark text
- **Data cells**: 50px tall, equal width distributing remaining canvas; alternating even/odd row backgrounds
- **Grid borders**: 1px lines using apex-background interpolated color
- **Text**: Noto Sans, 13px; truncated with `…` if wider than available cell width

## Example Files

- `valid/basic.toml` — 2 rows × 2 columns, 4 cells
- `valid/no_title.toml` — title omitted
- `valid/empty_cells.toml` — no cells defined (sparse table)
- `valid/basic.json` — JSON format
- `invalid/no_rows.toml` — missing rows → error
- `invalid/no_columns.toml` — missing columns → error
- `invalid/invalid_cell_ref.toml` — cell references nonexistent row → error
- `invalid/too_many_columns.toml` — 9 columns exceeds limit → error
