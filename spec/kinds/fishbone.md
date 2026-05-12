# Fishbone (Ishikawa)

A fishbone (Ishikawa) cause-and-effect diagram. A central spine leads to an effect. Causes branch off the spine; each cause may have sub-causes. Used for root cause analysis and structured problem decomposition.

## Fields

| Field   | Required | Type            | Description                                        |
|---------|----------|-----------------|----------------------------------------------------|
| kind    | yes      | `"fishbone"`    | Must be exactly `"fishbone"`                       |
| title   | no       | string          | Title rendered above the diagram                   |
| effect  | yes      | string          | Label for the effect (right end of the spine)      |
| causes  | yes      | array of Cause  | At least two causes required                       |

## Cause fields

| Field | Required | Type            | Description                                  |
|-------|----------|-----------------|----------------------------------------------|
| label | yes      | string          | Label displayed at the branch head            |
| items | no       | array of Item   | Sub-causes branching off this cause           |

## Item fields (sub-cause)

| Field | Required | Type   | Description                          |
|-------|----------|--------|--------------------------------------|
| label | yes      | string | Text displayed on the sub-cause line |

## Rendering rules

- The spine is a horizontal line from left to right.
- The effect box is at the right end of the spine.
- Causes alternate above and below the spine, evenly spaced.
- Each cause is represented by a diagonal branch line ending in a labeled box.
- Sub-causes are shorter branches off the cause branch.
- At least two causes are required; maximum 20.
- **8 or fewer causes is recommended** for visual clarity. With more than 9 causes, cause boxes on the same side of the spine begin to overlap horizontally.

## Example

```toml
kind = "fishbone"
title = "Website Performance Issues"

effect = "Slow Page Load"

[[causes]]
label = "Server"

[[causes.items]]
label = "CPU saturation"

[[causes.items]]
label = "Memory limits"

[[causes]]
label = "Network"

[[causes.items]]
label = "Bandwidth"

[[causes]]
label = "Code"

[[causes]]
label = "Database"

[[causes.items]]
label = "Slow queries"
```
