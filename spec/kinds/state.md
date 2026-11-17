# State

A state diagram represents a system's lifecycle as a set of named states and directed
transitions between them.

## Scope

`state` is a **named lifecycle for prose** — the diagram that explains how an order, a ticket, or a
deployment moves through its stages in a report or design document. It is not an executable state
machine notation:

- No guards, entry/exit actions, or internal transitions — `trigger` is a display label only.
- No nested (composite), parallel (orthogonal), or history states.
- Transitions connect only declared states; layout is chosen by the engine.

If a diagram needs any of the above, use an engineering tool such as Mermaid, D2, or PlantUML.

## Fields

| Field         | Required | Type              | Description                              |
|---------------|----------|-------------------|------------------------------------------|
| `kind`        | yes      | `"state"`         | Must be exactly `"state"`                |
| `title`       | no       | string            | Rendered above the diagram               |
| `states`      | yes      | array of State    | At least 2 states required               |
| `transitions` | no       | array of Transition | May be empty (unconnected states)      |

## State fields

| Field  | Required | Type   | Description                                                                    |
|--------|----------|--------|--------------------------------------------------------------------------------|
| `label`| yes      | string | Display text. Must be unique unless `id` is set.                               |
| `id`   | no       | string | Stable identifier for `from`/`to` references. Falls back to `label` if omitted.|
| `role` | no       | string | `"initial"` or `"terminal"`. At most one `"initial"`; multiple `"terminal"` allowed. |

## Transition fields

| Field     | Required | Type   | Description                                                  |
|-----------|----------|--------|--------------------------------------------------------------|
| `from`    | yes      | string | `id` (preferred) or `label` of source state                  |
| `to`      | yes      | string | `id` (preferred) or `label` of target state                  |
| `trigger` | no       | string | Event or condition causing this transition                    |
| `type`    | no       | string | `"normal"` (default) or `"exception"` — semantic type        |

Self-loop transitions (`from` = `to`) are valid. Transition without `trigger` is valid (unconditional).

## Cross-reference resolution

If a state has an `id`, use `id` in `from`/`to`. If it has no `id`, use `label`.
Unknown references → validation error. Duplicate labels without `id` → validation error.

## Example

```declart
kind = "state"
title = "Order Lifecycle"

[[states]]
id = "idle"
label = "Pending"
role = "initial"

[[states]]
id = "processing"
label = "Processing"

[[states]]
id = "done"
label = "Completed"
role = "terminal"

[[states]]
id = "cancelled"
label = "Cancelled"
role = "terminal"

[[transitions]]
from = "idle"
to = "processing"
trigger = "Order Received"

[[transitions]]
from = "processing"
to = "done"
trigger = "Payment OK"

[[transitions]]
from = "processing"
to = "cancelled"
trigger = "Payment Failed"
type = "exception"
```

<!-- limits:start — generated; do not edit by hand -->
## Label capacity

How long a label can be before the engine sets it smaller, wraps it, or cuts it short with "…". Each number is measured by rendering a small reference declaration with a growing label and reading the diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. **unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown without "…". A slot's width depends on its siblings, so read a number as a guide for the reference declaration named in the second column and use `declart validate --strict` for the declaration itself.

A dash means the engine reports nothing for the slot up to 120 characters. That is not a promise that any length fits: a slot the engine does not fit at all draws a long label past the picture without a warning.

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `states[].label` | 3 states | 5 / 5 | 12 / 12 |
| `transitions[].trigger` | 3 states | — / — | — / — |

<!-- limits:end -->
