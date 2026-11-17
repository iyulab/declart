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
| `mind_map` | Radial tree centered on the root node            | never (must be explicit)|

**Auto-selection:** When `view` is omitted, the engine selects `org_chart` for exactly one root node, `fishbone` for two or more root nodes. The CLI emits a warning when `view` is auto-selected; add an explicit `view` to suppress it.

## View-specific constraints

**org_chart:** exactly one root node required.  
**fishbone:** 2–20 root nodes (cause categories); `effect` (or `title` if `effect` is omitted) is rendered as the effect label at the right end of the spine. Recommend 8 or fewer root nodes — 9+ cause categories on the same side of the spine may overlap.  
**mind_map:** exactly one root node required. The root is placed at the center; its subtree radiates outward in all directions. Use for learning maps, brainstorm notes, and topic exploration.

## Long labels

In the `org_chart` view a node label that does not fit on one line — even shrunk to 85% of the label size — is wrapped onto the lines the node holds, shrinking toward the theme's minimum label size as needed; only a label that still overflows at that size is truncated with `…`. Hangul, CJK and other fullwidth characters are measured one em wide.

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

## Example — mind_map view

```declart
kind = "hierarchy"
view = "mind_map"
title = "Machine Learning"

[[nodes]]
label = "ML Concepts"

[[nodes]]
label = "Supervised"
parent = "ML Concepts"

[[nodes]]
label = "Unsupervised"
parent = "ML Concepts"

[[nodes]]
label = "Reinforce"
parent = "ML Concepts"

[[nodes]]
label = "Classification"
parent = "Supervised"

[[nodes]]
label = "Regression"
parent = "Supervised"
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

<!-- limits:start — generated; do not edit by hand -->
## Label capacity

How long a label can be before the engine sets it smaller, wraps it, or cuts it short with "…". Each number is measured by rendering a small reference declaration with a growing label and reading the diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. **unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown without "…". A slot's width depends on its siblings, so read a number as a guide for the reference declaration named in the second column and use `declart validate --strict` for the declaration itself.

A dash means the engine reports nothing for the slot up to 120 characters. That is not a promise that any length fits: a slot the engine does not fit at all draws a long label past the picture without a warning.

### `org_chart`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `nodes[].label (no parent)` | a root, 2 children | 7 / 20 | 14 / 38 |
| `nodes[].label (parent set)` | a root, 2 children | 7 / 20 | 14 / 38 |

### `fishbone`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `nodes[].label (no parent)` | 2 categories, 2 causes | 9 / 9 | 15 / 15 |
| `nodes[].label (parent set)` | 2 categories, 2 causes | 7 / 7 | 14 / 14 |
| `effect` | 2 categories, 2 causes | 10 / 10 | 19 / 19 |

### `mind_map`

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `nodes[].label (no parent)` | a root, 2 children | 6 / 6 | 13 / 13 |
| `nodes[].label (parent set)` | a root, 2 children | 8 / 8 | 16 / 16 |

<!-- limits:end -->

<!-- schema:start — generated; do not edit by hand -->
## Schema

What a declaration of this kind may contain, as `declart schema` and `schema()` report it. The value lists and counts are the ones the parser checks, and a test holds every field, count and rule to the parser in both directions, so a declaration written to this page parses. The prose above is the explanation; this is the reference.

### Without `view`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `hierarchy` |
| `view` | enum | no | `org_chart`, `fishbone`, `mind_map` |
| `title` | string | no |  |
| `effect` | string | no | fishbone: the label at the end of the spine; falls back to `title` |

**`nodes`** — 1 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `id` | string | no | a short name to refer to the node by |
| `parent` | reference | no | the id or label of the node above; a node without one is a root |

**Rules**

- node labels are unique
- node ids are unique
- `parent` names another node's id or label, never the node itself
- without `view`: one node without a `parent` makes an org_chart, more make a fishbone (2 to 20 of them)

### `org_chart`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `hierarchy` |
| `view` | enum | no | `org_chart`, `fishbone`, `mind_map` |
| `title` | string | no |  |
| `effect` | string | no | fishbone: the label at the end of the spine; falls back to `title` |

**`nodes`** — 1 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `id` | string | no | a short name to refer to the node by |
| `parent` | reference | no | the id or label of the node above; a node without one is a root |

**Rules**

- node labels are unique
- node ids are unique
- `parent` names another node's id or label, never the node itself
- exactly one node has no `parent` (the root)

### `fishbone`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `hierarchy` |
| `view` | enum | no | `org_chart`, `fishbone`, `mind_map` |
| `title` | string | no |  |
| `effect` | string | no | fishbone: the label at the end of the spine; falls back to `title` |

**`nodes`** — 1 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `id` | string | no | a short name to refer to the node by |
| `parent` | reference | no | the id or label of the node above; a node without one is a root |

**Rules**

- node labels are unique
- node ids are unique
- `parent` names another node's id or label, never the node itself
- 2 to 20 nodes have no `parent` (the cause categories)

### `mind_map`

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `hierarchy` |
| `view` | enum | no | `org_chart`, `fishbone`, `mind_map` |
| `title` | string | no |  |
| `effect` | string | no | fishbone: the label at the end of the spine; falls back to `title` |

**`nodes`** — 1 or more elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |
| `id` | string | no | a short name to refer to the node by |
| `parent` | reference | no | the id or label of the node above; a node without one is a root |

**Rules**

- node labels are unique
- node ids are unique
- `parent` names another node's id or label, never the node itself
- exactly one node has no `parent` (the root)

<!-- schema:end -->
