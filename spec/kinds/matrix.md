# Matrix 2×2

A two-by-two matrix diagram. Used for prioritization, strategy frameworks, and categorization with two independent axes.

## Fields

| Field      | Required | Type           | Description                                      |
|------------|----------|----------------|--------------------------------------------------|
| kind       | yes      | `"matrix"`     | Must be exactly `"matrix"`                       |
| title      | no       | string         | Title rendered above the diagram                 |
| x_axis     | yes      | string         | Label for the horizontal axis                    |
| y_axis     | yes      | string         | Label for the vertical axis                      |
| quadrants  | yes      | array of 4     | Exactly 4 quadrants, in reading order            |
| items      | no       | array of Item  | Optional items classified into quadrants (BCG/Gartner style) |

## Quadrant fields

| Field    | Required | Type   | Description                                                             |
|----------|----------|--------|-------------------------------------------------------------------------|
| label    | yes      | string | Text displayed in the quadrant                                          |
| emphasis | no       | string | `"primary"` or `"secondary"`. See schema.                               |
| status   | no       | string | `"success"`, `"normal"`, `"warning"`, or `"critical"`. See schema.      |
| position | no       | string | Explicit cell: `"top-left"`, `"top-right"`, `"bottom-left"`, `"bottom-right"` |

## Quadrant order

**Without `position`** (default): Quadrants are declared in reading order (left-to-right, top-to-bottom):
1. Top-left (high Y, low X)
2. Top-right (high Y, high X)
3. Bottom-left (low Y, low X)
4. Bottom-right (low Y, high X)

**With `position`**: All four quadrants must each declare a distinct `position`. Order in the file does not matter.
When any quadrant has `position`, all four must specify it.

## Item placement (optional)

Beyond naming the four quadrants, you can classify items into them — the BCG growth-share matrix
(Stars / Cash Cows / Question Marks / Dogs) and Gartner Magic Quadrant pattern. Items are an
**optional** `[[items]]` array; omit it for a label-only matrix (backward compatible).

| Field      | Required | Type   | Description                                                  |
|------------|----------|--------|--------------------------------------------------------------|
| label      | yes      | string | Text displayed for the item                                  |
| quadrant   | yes      | string | Owning quadrant by position: `top-left`/`top-right`/`bottom-left`/`bottom-right` |
| emphasis   | no       | string | `"primary"` or `"secondary"`. See schema.                    |
| status     | no       | string | `"success"`/`"normal"`/`"warning"`/`"critical"`. See schema. |

- Assignment is by **category** (which quadrant), never by `[x, y]` coordinates — the engine lays
  items out within the cell. This preserves the no-coordinates principle.
- `quadrant` references the position slot, so it is independent of whether quadrants were declared
  with explicit `position` or in index order.
- At most **6 items per quadrant** (legibility). When a quadrant has items, its `label` is rendered
  as a header at the top of the cell and the items are listed below.

## Rendering rules

- The diagram is a 2×2 grid divided by a horizontal and vertical axis line.
- Each quadrant occupies one cell of the grid and displays its label (centered when empty, as a top
  header when it has items).
- `x_axis` is rendered below the horizontal center line.
- `y_axis` is rendered to the left of the vertical center line, rotated 90°.
- Exactly 4 quadrants are required; more or fewer is a parse error.

## Example

```declart
kind = "matrix"
title = "Eisenhower Matrix"
x_axis = "Importance"
y_axis = "Urgency"

[[quadrants]]
label = "Do First"

[[quadrants]]
label = "Schedule"

[[quadrants]]
label = "Delegate"

[[quadrants]]
label = "Eliminate"
```

## Item placement example (BCG)

```declart
kind = "matrix"
title = "BCG Growth-Share Matrix"
x_axis = "Market Share"
y_axis = "Market Growth"

[[quadrants]]
label = "Stars"
position = "top-right"

[[quadrants]]
label = "Question Marks"
position = "top-left"

[[quadrants]]
label = "Cash Cows"
position = "bottom-right"

[[quadrants]]
label = "Dogs"
position = "bottom-left"

[[items]]
label = "Product A"
quadrant = "top-right"
status = "success"

[[items]]
label = "Product B"
quadrant = "top-left"
status = "warning"

[[items]]
label = "Product C"
quadrant = "bottom-left"
status = "critical"
```
