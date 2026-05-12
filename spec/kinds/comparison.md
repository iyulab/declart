# Comparison Table Kind — Specification

## Purpose

Presents a matrix of items (rows) evaluated against criteria (columns). Each cell value is declared inline within the row entry.

## TOML Schema

```
kind = "comparison"
title = <string>?         # optional diagram title

[[columns]]
label = <string>          # evaluation criterion (required, unique)

[[rows]]
label = <string>          # item being compared (required, unique)
<ColumnLabel> = <string>  # cell value for that column (optional, omit = empty cell)
```

### Constraints

| Constraint | Rule |
|-----------|------|
| Rows | 1–10 |
| Columns | 1–8 |
| Column label | Must not be `"label"` (reserved for the row identifier) |
| Cell keys in rows | Must match a declared `columns[].label`; unknown keys are an error |
| Missing cells | Treated as empty (allowed) |

## Rendering Specification

- **Canvas**: 800px wide, height = title_area + 45px column header + rows × 50px + 20px padding
- **Row header column**: 180px wide, apex-tinted background
- **Column header row**: 45px tall, apex color background, bold white/dark text
- **Data cells**: 50px tall, equal width distributing remaining canvas; alternating even/odd row backgrounds
- **Grid borders**: 1px lines using apex-background interpolated color
- **Text**: Noto Sans, 13px; truncated with `…` if wider than available cell width

## Example

```toml
kind = "comparison"
title = "JavaScript Framework Comparison"

[[columns]]
label = "Performance"

[[columns]]
label = "Ecosystem"

[[rows]]
label = "React"
Performance = "★★★★"
Ecosystem = "★★★★★"

[[rows]]
label = "Vue"
Performance = "★★★★"
Ecosystem = "★★★"
```

## Example Files

- `valid/basic.toml` — 2 rows × 2 columns, inline cells
- `valid/no_title.toml` — title omitted
- `valid/empty_cells.toml` — no cells defined (sparse table)
- `valid/basic.json` — JSON format
- `invalid/no_rows.toml` — missing rows → error
- `invalid/no_columns.toml` — missing columns → error
- `invalid/invalid_cell_ref.toml` — row uses undeclared column key → error
- `invalid/too_many_columns.toml` — 9 columns exceeds limit → error
