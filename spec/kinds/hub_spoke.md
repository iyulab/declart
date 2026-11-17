# Hub-and-Spoke

A hub-and-spoke diagram. A central node radiates connections to surrounding spoke nodes. Used for showing a central concept with related elements, or a hub with connected services.

## Fields

| Field    | Required | Type           | Description                          |
|----------|----------|----------------|--------------------------------------|
| kind     | yes      | `"hub_spoke"`  | Must be exactly `"hub_spoke"`        |
| title    | no       | string         | Title rendered above the diagram     |
| center   | yes      | string         | Label for the central hub node       |
| spokes   | yes      | array of Spoke | At least two spokes required         |

## Spoke fields

| Field    | Required | Type    | Description                               |
|----------|----------|---------|-------------------------------------------|
| label    | yes      | string  | Text displayed in the spoke node          |
| emphasis | no       | string  | `"primary"` or `"secondary"`. See schema. |
| status   | no       | string  | `"success"`, `"normal"`, `"warning"`, or `"critical"`. See schema. |

## Rendering rules

- The center node is rendered at the center of the diagram.
- Spoke nodes are arranged evenly around the center in a circle.
- Lines connect the center node to each spoke node.
- The center node is visually distinguished from spoke nodes (larger, different color).
- Spoke nodes use the base color; the center uses the apex color.

## Example

```declart
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

<!-- limits:start — generated; do not edit by hand -->
## Label capacity

How long a label can be before the engine sets it smaller, wraps it, or cuts it short with "…". Each number is measured by rendering a small reference declaration with a growing label and reading the diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. **unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown without "…". A slot's width depends on its siblings, so read a number as a guide for the reference declaration named in the second column and use `declart validate --strict` for the declaration itself.

A dash means the engine reports nothing for the slot up to 120 characters. That is not a promise that any length fits: a slot the engine does not fit at all draws a long label past the picture without a warning.

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `center` | 3 spokes | 9 / 9 | 22 / 22 |
| `spokes[].label` | 3 spokes | 9 / 9 | 15 / 15 |

<!-- limits:end -->

<!-- schema:start — generated; do not edit by hand -->
## Schema

What a declaration of this kind may contain, as `declart schema` and `schema()` report it. The value lists and counts are the ones the parser checks, and a test holds every field, count and rule to the parser in both directions, so a declaration written to this page parses. The prose above is the explanation; this is the reference.

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `hub_spoke` |
| `title` | string | no |  |
| `center` | string | yes |  |

**`spokes`** — 2 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `emphasis` | enum | no | `primary`, `secondary` |
| `status` | enum | no | `success`, `normal`, `warning`, `critical` |

<!-- schema:end -->
