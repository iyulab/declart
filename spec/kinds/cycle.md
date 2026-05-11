# Cycle

A cycle diagram. Items are arranged as steps in a closed loop, connected by directional arrows going clockwise. Used for recurring processes such as PDCA, feedback loops, and lifecycle models.

## Fields

| Field   | Required | Type        | Description                          |
|---------|----------|-------------|--------------------------------------|
| kind    | yes      | `"cycle"`   | Must be exactly `"cycle"`            |
| title   | no       | string      | Title rendered above the diagram     |
| items   | yes      | array of Item | At least two items required         |

## Item fields

| Field    | Required | Type    | Description                               |
|----------|----------|---------|-------------------------------------------|
| label    | yes      | string  | Text displayed in the step node           |
| emphasis | no       | string  | `"primary"` or `"secondary"`. See schema. |

## Rendering rules

- Items are arranged clockwise in a circle, starting at the top (12 o'clock).
- The last item connects back to the first item, forming a closed loop.
- Directional arrows point clockwise between adjacent nodes.
- Each node is a rectangular box centered on its position.
- Colors cycle through the apex-to-base gradient across all nodes.

## Example

```toml
kind = "cycle"
title = "PDCA Cycle"

[[items]]
label = "Plan"

[[items]]
label = "Do"

[[items]]
label = "Check"

[[items]]
label = "Act"
```
