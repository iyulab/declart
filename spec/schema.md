# Common Schema Rules

Rules that apply to every Declart declaration file.

## Required fields

| Field | Type   | Description                          |
|-------|--------|--------------------------------------|
| kind  | string | One of: `flow`, `tier`, `hierarchy`, `timeline`, `matrix`, `hub_spoke`, `venn`, `comparison`, `state` |

## Optional fields

| Field | Type   | Description                                        |
|-------|--------|----------------------------------------------------|
| title | string | Display title rendered above the diagram. Omit to suppress. |
| view  | string | Rendering intent within the kind. Valid values depend on `kind`. Omit to let the engine select automatically. |

## Item arrays

- Items are declared as `[[items]]` TOML array-of-tables.
- At least one item is required for `flow` and `tier`. The `matrix` kind uses `[[quadrants]]`; `hierarchy` uses `[[nodes]]`; `timeline` uses `[[events]]`; `hub_spoke` uses `[[spokes]]`; `venn` uses `[[sets]]`; `comparison` uses `[[columns]]` and `[[rows]]`.
- Item order in the file is rendering order.

## Forbidden fields

Any field not listed in a kind's spec document is forbidden. Forbidden fields cause a parse error. This includes but is not limited to: `color`, `fill`, `stroke`, `font`, `size`, `x`, `y`, `width`, `height`, `style`, `class`.

## Emphasis (shared optional item field)

When a kind supports item-level emphasis, it uses this field:

| Value       | Meaning                              |
|-------------|--------------------------------------|
| `primary`   | Most important item in the diagram   |
| `secondary` | Secondary importance                 |

The engine decides visual representation. Omitting `emphasis` means default weight.

- `primary`: white outline stroke + bold text
- `secondary`: lighter color tint

## Status (shared optional item field)

A semantic condition signal — the "traffic-light" state common in reports and dashboards.
`status` is **orthogonal to `emphasis`**: `emphasis` expresses *importance*, `status` expresses
*health / severity*. An item may carry both at once.

| Value      | Meaning                                  |
|------------|------------------------------------------|
| `success`  | Explicitly good / done                   |
| `normal`   | Assessed as nominal (the unmarked baseline) |
| `warning`  | Needs attention                          |
| `critical` | Failing / blocked / urgent               |

The engine decides visual representation — you declare meaning, not color. The current engine
renders a small corner marker, dual-encoded by **both color and shape** so it remains readable in
monochrome and under color-vision deficiency: `success` = circle, `warning` = triangle,
`critical` = diamond. The marker stays visually independent of `emphasis`. `normal` and omitted
`status` render no marker. Marker colors come from the active theme's status palette (the
`accessible` theme uses a colorblind-safe Okabe-Ito palette).

Supported on items of: `flow` (process / cycle / funnel / swimlane), `tier`, `hub_spoke`, `matrix`
(quadrants). Like `emphasis`, declaring it on a kind that does not support it is a parse error.
