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

## View values

| value       | Meaning                                          | Min items | Max items |
|-------------|--------------------------------------------------|-----------|-----------|
| `process`   | Linear left-to-right steps (default)             | 1         | —         |
| `cycle`     | Closed loop — last item connects to first        | 2         | —         |
| `funnel`    | Tapering stages (conversion/filtering)           | 2         | 10        |
| `swimlane`  | Steps grouped into horizontal actor lanes, top→down | 2         | —         |

When `view` is omitted, the engine uses `process`.

> For ranked/layered visuals (pyramid), use `kind = "tier"` instead.

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
