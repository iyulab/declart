# Hub-and-Spoke

A hub-and-spoke diagram. A central node radiates connections to surrounding spoke nodes. Used for showing a central concept with related elements, or a hub with connected services.

## Fields

| Field    | Required | Type           | Description                          |
|----------|----------|----------------|--------------------------------------|
| kind     | yes      | `"hub_spoke"`  | Must be exactly `"hub_spoke"`        |
| title    | no       | string         | Title rendered above the diagram     |
| center   | yes      | string         | Label for the central hub node       |
| spokes   | yes      | array of Spoke | At least one spoke required          |

## Spoke fields

| Field    | Required | Type    | Description                               |
|----------|----------|---------|-------------------------------------------|
| label    | yes      | string  | Text displayed in the spoke node          |
| emphasis | no       | string  | `"primary"` or `"secondary"`. See schema. |

## Rendering rules

- The center node is rendered at the center of the diagram.
- Spoke nodes are arranged evenly around the center in a circle.
- Lines connect the center node to each spoke node.
- The center node is visually distinguished from spoke nodes (larger, different color).
- Spoke nodes use the base color; the center uses the apex color.

## Example

```toml
kind = "hub_spoke"
title = "Cloud Architecture"

center = "API Gateway"

[[spokes]]
label = "Auth Service"

[[spokes]]
label = "User Service"

[[spokes]]
label = "Order Service"

[[spokes]]
label = "Payment Service"

[[spokes]]
label = "Notification"
```
