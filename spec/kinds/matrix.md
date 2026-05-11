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

## Quadrant fields

| Field    | Required | Type   | Description                                                             |
|----------|----------|--------|-------------------------------------------------------------------------|
| label    | yes      | string | Text displayed in the quadrant                                          |
| emphasis | no       | string | `"primary"` or `"secondary"`. See schema.                               |
| position | no       | string | Explicit cell: `"top-left"`, `"top-right"`, `"bottom-left"`, `"bottom-right"` |

## Quadrant order

**Without `position`** (default): Quadrants are declared in reading order (left-to-right, top-to-bottom):
1. Top-left (high Y, low X)
2. Top-right (high Y, high X)
3. Bottom-left (low Y, low X)
4. Bottom-right (low Y, high X)

**With `position`**: All four quadrants must each declare a distinct `position`. Order in the file does not matter.
When any quadrant has `position`, all four must specify it.

## Rendering rules

- The diagram is a 2×2 grid divided by a horizontal and vertical axis line.
- Each quadrant occupies one cell of the grid and displays its label centered.
- `x_axis` is rendered below the horizontal center line.
- `y_axis` is rendered to the left of the vertical center line, rotated 90°.
- Exactly 4 quadrants are required; more or fewer is a parse error.

## Example

```toml
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
