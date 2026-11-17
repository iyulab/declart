# System

A system diagram explains how a set of components fit together: the components, the boundaries
they sit in, and the links between them.

## Scope

`system` is a **conceptual architecture diagram for prose** — the picture in a report, design
document or study note that answers "what are the parts, which ones belong together, and what
passes between them". It is not a deployment, network or runtime specification:

- Components and links are **only the ones you declare**. There are no ports, protocols, sizes,
  coordinates or routes in the source; the engine places everything.
- A group is a boundary (a zone, a plane, a team, a pod). Groups nest at most two deep — a group
  inside a group.
- A link says what connects two components, and may carry a short label. Whether it is solid or
  dashed comes from its `relation`, not from a style field.
- One diagram holds at most **16 nodes, 6 groups and 24 links**. Past that, a diagram stops being
  readable at a glance; split it into two diagrams, one per concern. Exceeding a limit is a
  validation error.

If the diagram needs sequence ordering, entity relationships, class structure or a network
topology with addresses, use an engineering tool such as Mermaid, D2 or PlantUML.

## Choosing between `system` and another kind

A diagram has one kind. Pick the first line that fits:

| The relationship is… | Use |
|---|---|
| a single line of steps in order | `flow` |
| one centre with items radiating from it | `hub_spoke` |
| the stages of one thing's life, moving between states | `state` |
| a tree — every item has one parent | `hierarchy` |
| anything else: components in boundaries, or linked in more than a tree | `system` |

## Fields

| Field    | Required | Type             | Description                                  |
|----------|----------|------------------|----------------------------------------------|
| `kind`   | yes      | `"system"`       | Must be exactly `"system"`                   |
| `title`  | no       | string           | Rendered above the diagram                   |
| `groups` | no       | array of Group   | Boundaries. At most 6                        |
| `nodes`  | yes      | array of Node    | Components. At least 2, at most 16           |
| `links`  | no       | array of Link    | Connections. At most 24                      |

## Group fields

| Field    | Required | Type   | Description                                                      |
|----------|----------|--------|------------------------------------------------------------------|
| `id`     | yes      | string | Unique across all groups and nodes                               |
| `label`  | yes      | string | Display text                                                     |
| `parent` | no       | string | `id` of the group this one sits inside. Nesting is at most two deep |

## Node fields

| Field      | Required | Type   | Description                                                      |
|------------|----------|--------|------------------------------------------------------------------|
| `id`       | yes      | string | Unique across all groups and nodes                               |
| `label`    | yes      | string | Display text                                                     |
| `group`    | no       | string | `id` of the group the node sits in                               |
| `role`     | no       | string | `"store"` (data is kept here — a database, queue or file store), `"external"` (outside your control), `"actor"` (a person or client acting on the system). Omit for an ordinary component |
| `emphasis` | no       | string | `"primary"` or `"secondary"` — see [Schema](../schema.md)        |
| `status`   | no       | string | `"success"`, `"normal"`, `"warning"` or `"critical"` — see [Schema](../schema.md) |

## Link fields

| Field       | Required | Type   | Description                                                      |
|-------------|----------|--------|------------------------------------------------------------------|
| `from`      | yes      | string | `id` of a node or a group                                        |
| `to`        | yes      | string | `id` of a node or a group                                        |
| `label`     | no       | string | What passes along the link                                       |
| `direction` | no       | string | `"forward"` (default, `from` → `to`), `"both"`, or `"none"` (an association) |
| `relation`  | no       | string | `"direct"` (default — a call, a data flow) or `"indirect"` (asynchronous, optional or conditional) |

A link to a group ends at the group's boundary — use it for "the whole zone talks to X". A link
cannot join a group to something inside it, and cannot join an end to itself (a state that loops
back is a `state` diagram).

## Rules

- Node and group `id`s share one namespace; each must be unique.
- `group` and `parent` must name an existing group; `from` and `to` must name an existing node
  or group.
- Every link needs two different ends, neither inside the other.
- The engine chooses the layout. It lays each group out first and then treats it as one box, so
  boxes never overlap; links run along straight horizontal and vertical lines and go around
  components, not through them. Every label is set whole — a label too long for a box makes the
  box wider — and `render_diagnosed` reports the rare label that still had to be shrunk or cut.

## Example

```declart
kind = "system"
title = "Zero Trust Architecture"

[[groups]]
id = "cp"
label = "Control plane"

[[groups]]
id = "pdp"
label = "Policy decision point"
parent = "cp"

[[groups]]
id = "dp"
label = "Data plane"

[[nodes]]
id = "pe"
label = "Policy engine"
group = "pdp"

[[nodes]]
id = "pa"
label = "Policy administrator"
group = "pdp"

[[nodes]]
id = "signals"
label = "Identity, device state, threat intelligence, logs"
group = "cp"

[[nodes]]
id = "subject"
label = "Subject: user or device"
group = "dp"
role = "actor"

[[nodes]]
id = "pep"
label = "Policy enforcement point"
group = "dp"

[[nodes]]
id = "resource"
label = "Enterprise resource"
group = "dp"

[[links]]
from = "pe"
to = "pa"
direction = "both"

[[links]]
from = "signals"
to = "pe"

[[links]]
from = "subject"
to = "pep"

[[links]]
from = "pep"
to = "resource"

[[links]]
from = "pa"
to = "pep"
label = "allow / deny"

[[links]]
from = "pep"
to = "pa"
label = "access request"
relation = "indirect"
```

A link to a whole group:

```declart
kind = "system"
title = "Function point measurement boundary"

[[groups]]
id = "app"
label = "Application being measured"

[[nodes]]
id = "user"
label = "User"
role = "actor"

[[nodes]]
id = "ilf"
label = "ILF: internal logical file"
group = "app"
role = "store"

[[nodes]]
id = "eif"
label = "EIF: external interface file, kept by another system"
role = "store"

[[links]]
from = "user"
to = "app"
label = "EI (input)"

[[links]]
from = "app"
to = "user"
label = "EO (derived output)"

[[links]]
from = "eif"
to = "app"
label = "reference"
relation = "indirect"
```
