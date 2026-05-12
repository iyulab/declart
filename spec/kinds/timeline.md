# Timeline

A timeline diagram. Events are placed along a horizontal axis at positions proportional to their dates. Used for project histories, product roadmaps, and chronological narratives.

## Fields

| Field  | Required | Type           | Description                          |
|--------|----------|----------------|--------------------------------------|
| kind   | yes      | `"timeline"`   | Must be exactly `"timeline"`         |
| title  | no       | string         | Title rendered above the diagram     |
| events | yes      | array of Event | At least two events required         |

## Event fields

| Field | Required | Type   | Description                                                              |
|-------|----------|--------|--------------------------------------------------------------------------|
| date  | yes      | string | Date as `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. Partial forms are placed at the start of that year/month. |
| label | yes      | string | Text displayed above or below the event marker                           |

## Rendering rules

- Events are sorted by date and placed along a horizontal axis.
- The horizontal position of each event is proportional to its date within the overall date range.
- Event labels alternate above and below the axis to reduce overlap.
- The date string is displayed below each event marker.
- At least two events are required to establish a time range.

## Example

```toml
kind = "timeline"
title = "Product Launch History"

[[events]]
date = "2024-01-15"
label = "Alpha"

[[events]]
date = "2024-04-01"
label = "Beta"

[[events]]
date = "2024-07-20"
label = "Launch"

[[events]]
date = "2024-12-01"
label = "v2.0"
```
