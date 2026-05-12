# Sequence

A sequence diagram represents an ordered list of labeled items. The `view` field determines how the sequence is interpreted and rendered.

## Fields

| Field   | Required | Type          | Description                          |
|---------|----------|---------------|--------------------------------------|
| `kind`  | yes      | `"sequence"`  | Must be exactly `"sequence"`         |
| `view`  | no       | string        | Rendering intent. Default: `process` |
| `title` | no       | string        | Title rendered above the diagram     |
| `items` | yes      | array of Item | At least one item required           |

## Item fields

| Field      | Required | Type   | Description                               |
|------------|----------|--------|-------------------------------------------|
| `label`    | yes      | string | Text displayed in the item                |
| `emphasis` | no       | string | `"primary"` or `"secondary"`. See schema. |

## View values

| value     | Meaning                                          | Min items | Max items |
|-----------|--------------------------------------------------|-----------|-----------|
| `process` | Linear left-to-right steps (default)             | 1         | —         |
| `cycle`   | Closed loop — last item connects to first        | 2         | —         |
| `funnel`  | Tapering stages (conversion/filtering)           | 2         | 10        |
| `pyramid` | Stacked layers from apex (first) to base (last)  | 1         | —         |

When `view` is omitted, the engine uses `process`.

## Example

```declart
kind = "sequence"
view = "cycle"
title = "PDCA"

[[items]]
label = "Plan"

[[items]]
label = "Do"

[[items]]
label = "Check"

[[items]]
label = "Act"
```
