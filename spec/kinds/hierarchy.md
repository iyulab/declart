# Hierarchy

A hierarchy diagram represents a tree of labeled nodes connected by parent-child relationships. The `view` field determines how the tree is rendered.

## Fields

| Field    | Required | Type          | Description |
|----------|----------|---------------|-------------|
| `kind`   | yes      | `"hierarchy"` | Must be exactly `"hierarchy"` |
| `view`   | no       | string        | Rendering intent. Auto-selected if omitted. |
| `title`  | no       | string        | Chart title (rendered above the diagram) |
| `effect` | no       | string        | Fishbone view only: spine-end effect label. Falls back to `title` if omitted. |
| `nodes`  | yes      | array of Node | At least one node required |

## Node fields

| Field    | Required | Type   | Description |
|----------|----------|--------|-------------|
| `label`  | yes      | string | Display text |
| `id`     | no       | string | Stable identifier for `parent` references. If set, other nodes reference this node by `id`, not `label`. |
| `parent` | no       | string | `id` (preferred) or `label` of the parent node; omit for root nodes |

## View values

| value      | Meaning                                          | Auto-selected when      |
|------------|--------------------------------------------------|-------------------------|
| `org_chart`| Top-down tree; exactly one root required         | exactly 1 root node     |
| `fishbone` | Cause-and-effect diagram                         | 2+ root nodes           |

**Auto-selection:** When `view` is omitted, the engine selects `org_chart` for exactly one root node, `fishbone` for two or more root nodes. The CLI emits a warning when `view` is auto-selected; add an explicit `view` to suppress it.

## View-specific constraints

**org_chart:** exactly one root node required.  
**fishbone:** 2–20 root nodes (cause categories); `effect` (or `title` if `effect` is omitted) is rendered as the effect label at the right end of the spine. Recommend 8 or fewer root nodes — 9+ cause categories on the same side of the spine may overlap.

## Example — org_chart view (auto-selected)

```declart
kind = "hierarchy"
title = "Engineering Division"

[[nodes]]
label = "VP Engineering"

[[nodes]]
label = "Backend Team"
parent = "VP Engineering"

[[nodes]]
label = "Frontend Team"
parent = "VP Engineering"
```

## Example — fishbone view

```declart
kind = "hierarchy"
view = "fishbone"
title = "Slow Page Load"
effect = "High Latency"

[[nodes]]
label = "Server"

[[nodes]]
label = "CPU saturation"
parent = "Server"

[[nodes]]
label = "Network"

[[nodes]]
label = "Bandwidth"
parent = "Network"
```

## Example — org_chart with stable id references

```declart
kind = "hierarchy"
title = "Engineering Team"

[[nodes]]
id = "vp"
label = "VP Engineering"

[[nodes]]
label = "Backend Team"
parent = "vp"

[[nodes]]
label = "Frontend Team"
parent = "vp"
```
