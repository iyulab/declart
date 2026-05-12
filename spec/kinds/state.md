# State

A state diagram represents a system's lifecycle as a set of named states and directed
transitions between them.

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
title = "주문 처리 상태"

[[states]]
id = "idle"
label = "대기"
role = "initial"

[[states]]
id = "processing"
label = "처리 중"

[[states]]
id = "done"
label = "완료"
role = "terminal"

[[states]]
id = "cancelled"
label = "취소됨"
role = "terminal"

[[transitions]]
from = "idle"
to = "processing"
trigger = "주문 접수"

[[transitions]]
from = "processing"
to = "done"
trigger = "결제 완료"

[[transitions]]
from = "processing"
to = "cancelled"
trigger = "결제 실패"
type = "exception"
```
