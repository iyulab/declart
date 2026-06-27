# Tier

A tier diagram represents an ordered set of labeled levels. The `view` field determines how the levels are rendered and what relationship they express: `pyramid` shows them as ranked layers (importance, abstraction, or priority), while `concentric` shows them as nested rings (containment and dependency — the "onion" model).

## Fields

| Field   | Required | Type       | Description                          |
|---------|----------|------------|--------------------------------------|
| `kind`  | yes      | `"tier"`   | Must be exactly `"tier"`             |
| `view`  | no       | string     | Rendering intent. Default: `pyramid` |
| `title` | no       | string     | Title rendered above the diagram     |
| `items` | yes      | array of Item | At least one item required        |

## Item fields

| Field      | Required | Type   | Description                               |
|------------|----------|--------|-------------------------------------------|
| `label`    | yes      | string | Text displayed in the tier level          |
| `emphasis` | no       | string | `"primary"` or `"secondary"`. See schema. |
| `status`   | no       | string | `"success"`, `"normal"`, `"warning"`, or `"critical"`. See schema. |

## View values

| value        | Meaning                                                      | Min items | Max items |
|--------------|--------------------------------------------------------------|-----------|-----------|
| `pyramid`    | Stacked layers from apex (first) to base (last)              | 1         | —         |
| `concentric` | Nested rings from core (first, innermost) to periphery (last) | 1         | —         |

When `view` is omitted, the engine uses `pyramid`.

In `concentric`, item order reads inner → outer: the first item is the innermost core (the contained center), each subsequent item a ring enclosing it. This expresses containment and outer-depends-on-inner relationships (architecture layers, stakeholder rings, product layers). The data structure is identical to `pyramid` — only the rendering and the relationship it conveys differ.

## Example

```declart
kind = "tier"
title = "Maslow's Hierarchy of Needs"

[[items]]
label = "Self-Actualization"

[[items]]
label = "Esteem"

[[items]]
label = "Love & Belonging"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
```

## Example — concentric (onion)

```declart
kind = "tier"
view = "concentric"
title = "Clean Architecture Layers"

[[items]]
label = "Entities"

[[items]]
label = "Use Cases"

[[items]]
label = "Interface Adapters"

[[items]]
label = "Frameworks & Drivers"
```
