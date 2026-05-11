# Common Schema Rules

Rules that apply to every Declart declaration file.

## Required fields

| Field | Type   | Description                          |
|-------|--------|--------------------------------------|
| kind  | string | The diagram kind. See `kinds/` for valid values. |

## Optional fields

| Field | Type   | Description                                        |
|-------|--------|----------------------------------------------------|
| title | string | Display title rendered above the diagram. Omit to suppress. |

## Item arrays

- Items are declared as `[[items]]` TOML array-of-tables.
- At least one item is required for all kinds.
- Item order in the file is rendering order.

## Forbidden fields

Any field not listed in a kind's spec document is forbidden. Forbidden fields cause a parse error. This includes but is not limited to: `color`, `fill`, `stroke`, `font`, `size`, `x`, `y`, `width`, `height`, `style`, `class`, `id`.

## Emphasis (shared optional item field)

When a kind supports item-level emphasis, it uses this field:

| Value       | Meaning                              |
|-------------|--------------------------------------|
| `primary`   | Most important item in the diagram   |
| `secondary` | Secondary importance                 |

The engine decides visual representation. Omitting `emphasis` means default weight.

> **v0.1 note:** `emphasis` is parsed and validated but not yet reflected in rendered output. Visual differentiation of emphasized items is planned for a future release.
