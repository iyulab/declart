# Org Chart

An org chart renders a hierarchical tree of labeled nodes connected by lines. Nodes are declared as a flat list with parent references, so the tree can have arbitrary depth.

## TOML Structure

```toml
kind = "org_chart"
title = "Company Structure"   # optional

[[nodes]]
label = "CEO"
# No parent = root

[[nodes]]
label = "CTO"
parent = "CEO"

[[nodes]]
label = "CFO"
parent = "CEO"
```

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | ✅ | Must be `"org_chart"` |
| `title` | string | — | Optional diagram title |
| `[[nodes]]` | array | ✅ | One or more node entries |
| `nodes[].label` | string | ✅ | Display text for the node; must be unique within the diagram |
| `nodes[].parent` | string | — | `label` of the parent node; omit for the root node |

## Constraints

- Exactly one root node (a node with no `parent`) is required.
- All `parent` values must reference an existing `label` in the same diagram.
- All `label` values must be unique within the diagram.
- A node cannot reference itself as its parent.
- At least one node is required.

## Rendering

- Nodes are rendered as rectangular boxes.
- The root node uses the theme's apex color; child nodes use the base color.
- Nodes at the same depth are aligned on the same horizontal level.
- Subtrees are centered beneath their parent.
- Connections use elbow connectors (vertical → horizontal → vertical line segments).

## Example: Company Structure

```declart
kind = "org_chart"
title = "Engineering Division"

[[nodes]]
label = "VP of Engineering"

[[nodes]]
label = "Backend Team"
parent = "VP of Engineering"

[[nodes]]
label = "Frontend Team"
parent = "VP of Engineering"

[[nodes]]
label = "Alice"
parent = "Backend Team"

[[nodes]]
label = "Bob"
parent = "Backend Team"

[[nodes]]
label = "Carol"
parent = "Frontend Team"
```
