# Common Schema Rules

Rules that apply to every Declart declaration file.

## Required fields

| Field | Type   | Description                          |
|-------|--------|--------------------------------------|
| kind  | string | One of: `flow`, `tier`, `hierarchy`, `timeline`, `matrix`, `hub_spoke`, `venn`, `comparison` |

## Optional fields

| Field | Type   | Description                                        |
|-------|--------|----------------------------------------------------|
| title | string | Display title rendered above the diagram. Omit to suppress. |
| view  | string | Rendering intent within the kind. Valid values depend on `kind`. Omit to let the engine select automatically. |

## Item arrays

- Items are declared as `[[items]]` TOML array-of-tables.
- At least one item is required for `flow` and `tier`. The `matrix` kind uses `[[quadrants]]`; `hierarchy` uses `[[nodes]]`; `timeline` uses `[[events]]`; `hub_spoke` uses `[[spokes]]`; `venn` uses `[[sets]]`; `comparison` uses `[[columns]]` and `[[rows]]`.
- Item order in the file is rendering order.

## Forbidden fields

Any field not listed in a kind's spec document is forbidden. Forbidden fields cause a parse error. This includes but is not limited to: `color`, `fill`, `stroke`, `font`, `size`, `x`, `y`, `width`, `height`, `style`, `class`.

## Emphasis (shared optional item field)

When a kind supports item-level emphasis, it uses this field:

| Value       | Meaning                              |
|-------------|--------------------------------------|
| `primary`   | Most important item in the diagram   |
| `secondary` | Secondary importance                 |

The engine decides visual representation. Omitting `emphasis` means default weight.

- `primary`: white outline stroke + bold text
- `secondary`: lighter color tint
