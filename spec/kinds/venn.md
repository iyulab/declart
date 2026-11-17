# Venn

A Venn diagram. Two or three overlapping circles represent sets. Intersection regions may be labeled. Used for showing set relationships, similarities, and differences.

## Fields

| Field          | Required | Type                | Description                                    |
|----------------|----------|---------------------|------------------------------------------------|
| kind           | yes      | `"venn"`            | Must be exactly `"venn"`                       |
| title          | no       | string              | Title rendered above the diagram               |
| sets           | yes      | array of Set        | 2 or 3 sets required                           |
| intersections  | no       | array of Intersection | Labeled regions in the overlap areas         |

## Set fields

| Field | Required | Type   | Description               |
|-------|----------|--------|---------------------------|
| label | yes      | string | Name of the set           |

## Intersection fields

| Field | Required | Type         | Description                                                  |
|-------|----------|--------------|--------------------------------------------------------------|
| sets  | yes      | array of string | Set labels identifying which overlap region this labels   |
| label | yes      | string       | Text displayed in the overlap region                         |

## Rendering rules

- Circles are arranged to overlap: two sets side-by-side; three sets in a triangle.
- Each circle is rendered with partial opacity so overlaps are visible.
- Set labels appear near the outer edge of each circle (non-overlapping region).
- Intersection labels appear in the center of the overlap region.
- Exactly 2 or 3 sets are required; other counts are a parse error.

## Example

```declart
kind = "venn"
title = "Skills Overlap"

[[sets]]
label = "Design"

[[sets]]
label = "Engineering"

[[intersections]]
sets = ["Design", "Engineering"]
label = "UX Engineering"
```

<!-- limits:start — generated; do not edit by hand -->
## Label capacity

How long a label can be before the engine sets it smaller, wraps it, or cuts it short with "…". Each number is measured by rendering a small reference declaration with a growing label and reading the diagnostics `render_detailed` returns, so the table always agrees with them. Lengths are in characters written without spaces: Hangul syllables are one em wide, Latin lowercase letters about half that. **unchanged** is the longest label shown at the usual size on one line; **whole** is the longest shown without "…". A slot's width depends on its siblings, so read a number as a guide for the reference declaration named in the second column and use `declart validate --strict` for the declaration itself.

A dash means the engine reports nothing for the slot up to 120 characters. That is not a promise that any length fits: a slot the engine does not fit at all draws a long label past the picture without a warning.

| Slot | Measured in | Hangul (unchanged / whole) | Latin (unchanged / whole) |
|------|-------------|----------------------------|---------------------------|
| `sets[].label` | 2 sets | — / — | — / — |
| `intersections[].label` | 2 sets | — / — | — / — |

<!-- limits:end -->

<!-- schema:start — generated; do not edit by hand -->
## Schema

What a declaration of this kind may contain, as `declart schema` and `schema()` report it. The value lists and counts are the ones the parser checks, and a test holds every field, count and rule to the parser in both directions, so a declaration written to this page parses. The prose above is the explanation; this is the reference.

**Fields**

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `kind` | enum | yes | `venn` |
| `title` | string | no |  |

**`sets`** — 2 to 3 elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `label` | string | yes |  |

**`intersections`** — any number of elements

| Field | Type | Required | Values / meaning |
|-------|------|----------|------------------|
| `sets` | string[] | yes | the labels of the sets that overlap |
| `label` | string | yes |  |

<!-- schema:end -->
