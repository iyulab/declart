# Process

A process diagram. Items are arranged as sequential steps from left to right. Used for workflows, procedures, and pipelines where order matters.

## Fields

| Field   | Required | Type          | Description                          |
|---------|----------|---------------|--------------------------------------|
| kind    | yes      | `"process"`   | Must be exactly `"process"`          |
| title   | no       | string        | Title rendered above the diagram     |
| items   | yes      | array of Item | At least one item required           |

## Item fields

| Field    | Required | Type    | Description                               |
|----------|----------|---------|-------------------------------------------|
| label    | yes      | string  | Text displayed in the step box            |
| emphasis | no       | string  | `"primary"` or `"secondary"`. See schema. |

## Rendering rules

- Items render left to right in declaration order.
- Each step is a uniform box; all boxes have the same height.
- Directional arrows connect adjacent boxes, pointing right.
- Box widths are equal and fill the available canvas width.
- Canvas width scales to guarantee a minimum box width of 100px.
- Color interpolates from left (apex) to right (base) across all steps.

## Example

```toml
kind = "process"
title = "Software Release Pipeline"

[[items]]
label = "Code Review"

[[items]]
label = "Build"

[[items]]
label = "Test"

[[items]]
label = "Deploy"

[[items]]
label = "Monitor"
```
