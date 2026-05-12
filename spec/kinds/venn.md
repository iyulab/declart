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
