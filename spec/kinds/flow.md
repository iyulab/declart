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
title = "주문 처리 프로세스"

[[items]]
actor = "고객"
label = "주문 요청"

[[items]]
actor = "시스템"
label = "재고 확인"

[[items]]
actor = "결제 게이트웨이"
label = "결제 처리"

[[items]]
actor = "시스템"
label = "주문 확정"

[[items]]
actor = "고객"
label = "확인 수신"
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
