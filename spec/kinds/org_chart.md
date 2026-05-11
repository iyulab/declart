# Org Chart

An org chart renders a hierarchical tree of labeled nodes connected by lines. Nodes are declared as a flat list with parent references, so the tree can have arbitrary depth.

## TOML Structure

```toml
kind = "org_chart"
title = "Company Structure"   # optional

[[nodes]]
id = "ceo"
label = "CEO"
# No parent = root

[[nodes]]
id = "cto"
label = "CTO"
parent = "ceo"

[[nodes]]
id = "cfo"
label = "CFO"
parent = "ceo"
```

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | ✅ | Must be `"org_chart"` |
| `title` | string | — | Optional diagram title |
| `[[nodes]]` | array | ✅ | One or more node entries |
| `nodes[].id` | string | ✅ | Unique node identifier (used in `parent` references) |
| `nodes[].label` | string | ✅ | Display text for the node |
| `nodes[].parent` | string | — | `id` of the parent node; omit for the root node |

## Constraints

- Exactly one root node (a node with no `parent`) is required.
- All `parent` values must reference an existing `id` in the same diagram.
- All `id` values must be unique within the diagram.
- A node cannot reference itself as its parent.
- At least one node is required.

## Rendering

- Nodes are rendered as rectangular boxes.
- The root node uses the theme's apex color; child nodes use the base color.
- Nodes at the same depth are aligned on the same horizontal level.
- Subtrees are centered beneath their parent.
- Connections use elbow connectors (vertical → horizontal → vertical line segments).

## Example: Company Structure

```toml
kind = "org_chart"
title = "Engineering Division"

[[nodes]]
id = "vp"
label = "VP of Engineering"

[[nodes]]
id = "backend"
label = "Backend Team"
parent = "vp"

[[nodes]]
id = "frontend"
label = "Frontend Team"
parent = "vp"

[[nodes]]
id = "be1"
label = "Alice"
parent = "backend"

[[nodes]]
id = "be2"
label = "Bob"
parent = "backend"

[[nodes]]
id = "fe1"
label = "Carol"
parent = "frontend"
```
