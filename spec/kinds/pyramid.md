# Pyramid

A pyramid diagram. Items are arranged in horizontal layers from apex (top, first item) to base (bottom, last item). Conventionally used for hierarchies where importance or scarcity increases toward the apex.

## Fields

| Field   | Required | Type              | Description                     |
|---------|----------|-------------------|---------------------------------|
| kind    | yes      | `"pyramid"`       | Must be exactly `"pyramid"`     |
| title   | no       | string            | Title rendered above the pyramid |
| items   | yes      | array of Item     | At least one item required      |

## Item fields

| Field    | Required | Type    | Description                               |
|----------|----------|---------|-------------------------------------------|
| label    | yes      | string  | Text displayed in the layer               |
| emphasis | no       | string  | `"primary"` or `"secondary"`. See schema. |

## Rendering rules

- First item (`items[0]`) is the apex (smallest, top).
- Last item (`items[n-1]`) is the base (widest, bottom).
- Layer widths increase linearly from apex to base.
- Apex layer degenerates to a triangle when it is the narrowest point.

## Example

```toml
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
