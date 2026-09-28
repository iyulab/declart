# Timeline

A timeline diagram. Events are placed along a horizontal axis at positions proportional to their dates. An event is either a **milestone** (a single date) or a **period** (a start and end date). Used for project histories, product roadmaps, and chronological narratives.

## Fields

| Field  | Required | Type           | Description                          |
|--------|----------|----------------|--------------------------------------|
| kind   | yes      | `"timeline"`   | Must be exactly `"timeline"`         |
| title  | no       | string         | Title rendered above the diagram     |
| events | yes      | array of Event | At least two events required         |

## Event fields

| Field | Required    | Type   | Description                                                   |
|-------|-------------|--------|---------------------------------------------------------------|
| date  | milestone   | string | The milestone's date                                          |
| start | period      | string | First date of the period                                      |
| end   | period      | string | Last date of the period (inclusive)                           |
| label | yes         | string | Text displayed with the milestone marker or period bar        |

Each event declares **either** `date` (a milestone) **or** both `start` and `end` (a period). Any other
combination is a validation error.

### Dates

All date fields accept `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`, and must name a real calendar date
(`2024-02-30` is rejected). A partial date names a whole year or month:

- As a milestone `date` or a period `start`, it is placed at the **first day** of that year/month.
- As a period `end`, it is **inclusive** — the period runs through the **last day** of that year/month
  (or through that day, for a full date). `start = "2024-02"`, `end = "2024-03"` covers February and March.

`end` must not fall before `start`. A period may be a single year, month, or day (`start = end`).

## Rendering rules

- Events are sorted by date (a period by its `start`) and placed along a horizontal axis.
- The horizontal position of each event is proportional to its date within the overall date range,
  which spans from the earliest date to the latest milestone or period end.
- Milestones are markers on the axis. Their labels alternate above and below the axis to reduce
  overlap, and the date string is displayed below each marker.
- Periods are bars below the axis, spanning their dates, with the label inside the bar (or beside
  it when the bar is too short) and the date range underneath. Overlapping periods stack into
  separate rows; periods that do not overlap share a row.
- At least two events are required to establish a time range.
- This is a qualitative roadmap, not a scheduler: there are no dependencies, progress, or
  resource fields. Use a project-management tool for precise Gantt charts.

## Example

```declart
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

### Milestones and periods

```declart
kind = "timeline"
title = "Product Roadmap 2024"

[[events]]
date = "2024-01-15"
label = "Kickoff"

[[events]]
start = "2024-02"
end = "2024-04"
label = "Private Beta"

[[events]]
start = "2024-03"
end = "2024-09"
label = "Enterprise Pilot"

[[events]]
start = "2024-05"
end = "2024-06"
label = "Public Beta"

[[events]]
date = "2024-07-01"
label = "GA Launch"
```
