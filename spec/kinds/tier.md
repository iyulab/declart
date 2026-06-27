# Tier

A tier diagram represents a ranked set of labeled items — levels differentiated by importance, abstraction, or priority. The `view` field determines how the tiers are rendered.

## Fields

| Field   | Required | Type       | Description                          |
|---------|----------|------------|--------------------------------------|
| `kind`  | yes      | `"tier"`   | Must be exactly `"tier"`             |
| `view`  | no       | string     | Rendering intent. Default: `pyramid` |
| `title` | no       | string     | Title rendered above the diagram     |
| `items` | yes      | array of Item | At least one item required        |

## Item fields

| Field      | Required | Type   | Description                               |
|------------|----------|--------|-------------------------------------------|
| `label`    | yes      | string | Text displayed in the tier level          |
| `emphasis` | no       | string | `"primary"` or `"secondary"`. See schema. |
| `status`   | no       | string | `"success"`, `"normal"`, `"warning"`, or `"critical"`. See schema. |

## View values

| value     | Meaning                                         | Min items | Max items |
|-----------|-------------------------------------------------|-----------|-----------|
| `pyramid` | Stacked layers from apex (first) to base (last) | 1         | —         |

When `view` is omitted, the engine uses `pyramid`.

## Example

```declart
kind = "tier"
title = "Maslow's Hierarchy of Needs"

[[items]]
label = "Self-Actualization"

[[items]]
label = "Esteem"

[[items]]
label = "Love & Belonging"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
```
