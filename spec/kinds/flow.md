# Flow

A flow diagram represents an ordered list of labeled items. The `view` field determines how the flow is interpreted and rendered.

## Fields

| Field   | Required | Type       | Description                          |
|---------|----------|------------|--------------------------------------|
| `kind`  | yes      | `"flow"`   | Must be exactly `"flow"`             |
| `view`  | no       | string     | Rendering intent. Default: `process` |
| `title` | no       | string     | Title rendered above the diagram     |
| `items` | yes      | array of Item | At least one item required        |

## Item fields

| Field      | Required | Type   | Description                               |
|------------|----------|--------|-------------------------------------------|
| `label`    | yes      | string | Text displayed in the item                |
| `emphasis` | no       | string | `"primary"` or `"secondary"`. See schema. |
| `status`   | no       | string | `"success"`, `"normal"`, `"warning"`, or `"critical"` — health/severity signal, orthogonal to `emphasis`. See schema. |

## View values

| value       | Meaning                                          | Min items | Max items |
|-------------|--------------------------------------------------|-----------|-----------|
| `process`   | Linear left-to-right steps (default)             | 1         | —         |
| `cycle`     | Closed loop — last item connects to first        | 2         | —         |
| `funnel`    | Tapering stages (conversion/filtering)           | 2         | 10        |
| `swimlane`  | Steps grouped into horizontal actor lanes, top→down | 2         | —         |

When `view` is omitted, the engine uses `process`.

> For ranked/layered visuals (pyramid), use `kind = "tier"` instead.

## Long labels

In the `process` and `swimlane` views a label that does not fit on one line — even shrunk to 85% of the label size — is wrapped onto as many lines as the box holds (at most three) — at spaces, and between characters for Chinese and Japanese text without spaces. The label shrinks toward the theme's minimum label size only when wrapping alone is not enough, and only a label that still overflows at that size is truncated with `…` on its last line. Hangul, CJK and other fullwidth characters are measured one em wide.

## Swimlane item fields

When `view = "swimlane"`, each item gains one additional required field:

| Field   | Required | Type   | Description           |
|---------|----------|--------|-----------------------|
| `actor` | yes      | string | Lane owner of this step |

`actor` is ignored by all other views. At least 2 distinct actor values are required.

## Swimlane example

```declart
kind = "flow"
view = "swimlane"
title = "Order Processing"

[[items]]
actor = "Customer"
label = "Place Order"

[[items]]
actor = "System"
label = "Check Inventory"

[[items]]
actor = "Payment Gateway"
label = "Process Payment"

[[items]]
actor = "System"
label = "Confirm Order"

[[items]]
actor = "Customer"
label = "Receive Confirmation"
```

## Example

```declart
kind = "flow"
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

## Status example

`status` adds a traffic-light health signal per step. It is orthogonal to `emphasis`, so a step can
be both the most important (`primary`) and in trouble (`critical`).

```declart
kind = "flow"
title = "Release Pipeline Health"

[[items]]
label = "Build"
status = "success"

[[items]]
label = "Test"
status = "warning"

[[items]]
label = "Deploy"
emphasis = "primary"
status = "critical"

[[items]]
label = "Monitor"
status = "normal"
```

<!-- limits:start — generated; do not edit by hand -->
## Label capacity

How long a label can be before the engine sets it smaller, wraps it, or cuts it short with "…". Each number is measured by rendering a small reference declaration with a growing label and reading the diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. **unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown without "…". A slot's width depends on its siblings, so read a number as a guide for the reference declaration named in the second column and use `declart validate --strict` for the declaration itself.

A dash means the engine reports nothing for the slot up to 120 characters. That is not a promise that any length fits: a slot the engine does not fit at all draws a long label past the picture without a warning.

### `process`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `items[].label` | 3 items | 20 / 42 | 39 / 81 |

### `cycle`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `items[].label` | 3 items | 9 / 9 | 15 / 15 |

### `funnel`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `items[].label` | 3 items | 22 / 22 | 49 / 49 |

### `swimlane`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `items[].label` | 3 items | 18 / 36 | 34 / 72 |

<!-- limits:end -->

<!-- schema:start — generated; do not edit by hand -->
## Schema

What a declaration of this kind may contain, as `declart schema` and `schema()` report it. The value lists and counts are the ones the parser checks, and a test holds every field, count and rule to the parser in both directions, so a declaration written to this page parses. The prose above is the explanation; this is the reference.

### `process`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `flow` |
| `view` | enum | no | `process`, `cycle`, `funnel`, `swimlane` |
| `title` | string | no |  |

**`items`** — 1 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `emphasis` | enum | no | `primary`, `secondary` |
| `status` | enum | no | `success`, `normal`, `warning`, `critical` |

### `cycle`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `flow` |
| `view` | enum | no | `process`, `cycle`, `funnel`, `swimlane` |
| `title` | string | no |  |

**`items`** — 2 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `emphasis` | enum | no | `primary`, `secondary` |
| `status` | enum | no | `success`, `normal`, `warning`, `critical` |

### `funnel`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `flow` |
| `view` | enum | no | `process`, `cycle`, `funnel`, `swimlane` |
| `title` | string | no |  |

**`items`** — 2 to 10 elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `emphasis` | enum | no | `primary`, `secondary` |
| `status` | enum | no | `success`, `normal`, `warning`, `critical` |

### `swimlane`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `flow` |
| `view` | enum | no | `process`, `cycle`, `funnel`, `swimlane` |
| `title` | string | no |  |

**`items`** — 2 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `emphasis` | enum | no | `primary`, `secondary` |
| `status` | enum | no | `success`, `normal`, `warning`, `critical` |
| `actor` | string | yes | the lane the step belongs to |

**Rules**

- `actor` must not be empty on any item
- the items name at least 2 distinct actors

<!-- schema:end -->
