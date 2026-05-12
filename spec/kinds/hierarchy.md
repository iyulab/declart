# Hierarchy

A hierarchy diagram represents a tree of labeled nodes connected by parent-child relationships. The `view` field determines how the tree is rendered.

## Fields

| Field    | Required | Type          | Description |
|----------|----------|---------------|-------------|
| `kind`   | yes      | `"hierarchy"` | Must be exactly `"hierarchy"` |
| `view`   | no       | string        | Rendering intent. Auto-selected if omitted. |
| `title`  | no       | string        | Chart title. In `fishbone` view: rendered as the effect label. |
| `nodes`  | yes      | array of Node | At least one node required |

## Node fields

| Field    | Required | Type   | Description |
|----------|----------|--------|-------------|
| `label`  | yes      | string | Display text; must be unique within the diagram |
| `parent` | no       | string | `label` of the parent node; omit for root nodes |

## View values

| value      | Meaning                                          | Auto-selected when      |
|------------|--------------------------------------------------|-------------------------|
| `org_chart`| Top-down tree; exactly one root required         | exactly 1 root node     |
| `fishbone` | Cause-and-effect; `title` becomes effect label   | 2+ root nodes           |

**Auto-selection:** When `view` is omitted, the engine selects `org_chart` for exactly one root node, `fishbone` for two or more root nodes.

## View-specific constraints

**org_chart:** exactly one root node required.  
**fishbone:** 2–20 root nodes (cause categories); `title` is rendered as the effect label at the right end of the spine.

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
