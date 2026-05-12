# Using Claude or GPT to Generate Declart Diagrams

Declart's TOML format is designed for LLM generation. The structure is explicit and validated — an LLM can produce a well-formed diagram in one shot, and `declart validate` catches any mistakes before rendering.

## Why Declart works well with LLMs

- **No layout decisions**: the LLM only writes content (labels, structure). The engine handles all visual choices.
- **Strict schema**: `deny_unknown_fields` means invalid keys are caught immediately. The LLM cannot silently produce a broken diagram.
- **Explicit kind**: the `kind` field tells the parser exactly what to expect. No ambiguity.
- **Short format**: a typical diagram is 5–20 lines of TOML.

## Workflow

```
1. Prompt LLM → TOML output
2. declart validate diagram.toml   # catches errors with clear messages
3. declart render diagram.toml     # produces SVG
```

If `validate` fails, paste the error message back to the LLM and ask it to fix the specific field.

---

## Prompt Templates

### Pyramid — Hierarchies, priority layers

**Prompt**: *"Generate a Declart TOML pyramid diagram showing Maslow's hierarchy of needs. Use kind = 'pyramid', include a title, and list the 5 levels as items from top (most basic) to bottom."*

```toml
kind = "pyramid"
title = "Maslow's Hierarchy of Needs"

[[items]]
label = "Self-Actualization"

[[items]]
label = "Esteem"

[[items]]
label = "Love & Belonging"

[[items]]
label = "Safety"

[[items]]
label = "Physiological"
emphasis = "primary"
```

---

### Process — Sequential steps, workflows

**Prompt**: *"Create a Declart TOML process diagram for a 4-step CI/CD pipeline."*

```toml
kind = "process"
title = "CI/CD Pipeline"

[[items]]
label = "Build"

[[items]]
label = "Test"
emphasis = "primary"

[[items]]
label = "Stage"

[[items]]
label = "Deploy"
```

---

### Cycle — Closed loops, PDCA, lifecycles

**Prompt**: *"Generate a Declart TOML cycle diagram for the PDCA improvement cycle."*

```toml
kind = "cycle"
title = "PDCA Cycle"

[[items]]
label = "Plan"

[[items]]
label = "Do"

[[items]]
label = "Check"

[[items]]
label = "Act"
```

---

### Matrix 2×2 — Prioritization, Eisenhower

**Prompt**: *"Create an Eisenhower Matrix in Declart TOML with x_axis = 'Importance' and y_axis = 'Urgency'."*

```toml
kind = "matrix"
title = "Eisenhower Matrix"
x_axis = "Importance"
y_axis = "Urgency"

[[quadrants]]
label = "Do First"
position = "top-right"
emphasis = "primary"

[[quadrants]]
label = "Schedule"
position = "top-left"

[[quadrants]]
label = "Delegate"
position = "bottom-right"

[[quadrants]]
label = "Eliminate"
position = "bottom-left"
```

> **Note**: Use `position` to explicitly place quadrants. Valid values: `top-left`, `top-right`, `bottom-left`, `bottom-right`.

---

### Hub-and-Spoke — Central concept with related items

**Prompt**: *"Make a Declart hub-and-spoke diagram with 'Cloud Architecture' as the center and 5 services as spokes."*

```toml
kind = "hub_spoke"
title = "Cloud Architecture"
center = "API Gateway"

[[spokes]]
label = "Auth Service"

[[spokes]]
label = "User DB"

[[spokes]]
label = "Payment"

[[spokes]]
label = "Notifications"

[[spokes]]
label = "Analytics"
```

---

### Venn — Set intersections, overlapping groups

**Prompt**: *"Generate a 2-set Venn diagram showing the overlap between 'Frontend Skills' and 'Backend Skills'."*

```toml
kind = "venn"
title = "Full-Stack Skills"

[[sets]]
label = "Frontend"

[[sets]]
label = "Backend"

[[intersections]]
sets = ["Frontend", "Backend"]
label = "TypeScript"
```

---

### Timeline — Date-anchored events

**Prompt**: *"Create a Declart timeline of 5 product launch milestones in 2024, using ISO dates."*

```toml
kind = "timeline"
title = "Product Launch 2024"

[[events]]
date = "2024-01-15"
label = "Alpha"

[[events]]
date = "2024-03-01"
label = "Beta"

[[events]]
date = "2024-06-01"
label = "RC1"

[[events]]
date = "2024-09-15"
label = "GA"

[[events]]
date = "2024-12-01"
label = "v2 Plan"
```

> **Rule**: dates must be `YYYY-MM-DD`. Declart sorts events automatically.

---

### Fishbone / Ishikawa — Root cause analysis

**Prompt**: *"Generate a Declart fishbone diagram where the effect is 'Slow API Response' with 4 causes and sub-items."*

```toml
kind = "fishbone"
title = "API Performance Issues"
effect = "Slow API Response"

[[causes]]
label = "Database"

[[causes.items]]
label = "Missing indexes"

[[causes.items]]
label = "N+1 queries"

[[causes]]
label = "Network"

[[causes.items]]
label = "High latency"

[[causes]]
label = "Code"

[[causes.items]]
label = "Blocking I/O"

[[causes]]
label = "Infrastructure"
```

> **Limit**: 8 causes or fewer is recommended; maximum 20.

---

### Org Chart — Hierarchical trees

**Prompt**: *"Create a Declart org chart for a small engineering team with a CTO at the top."*

```toml
kind = "org_chart"
title = "Engineering Team"

[[nodes]]
id = "cto"
label = "CTO"

[[nodes]]
id = "fe_lead"
label = "Frontend Lead"
parent = "cto"

[[nodes]]
id = "be_lead"
label = "Backend Lead"
parent = "cto"

[[nodes]]
id = "fe_dev"
label = "FE Developer"
parent = "fe_lead"

[[nodes]]
id = "be_dev"
label = "BE Developer"
parent = "be_lead"
```

> **Rule**: exactly one root node (no `parent`). All `parent` values must reference an existing `id`.

---

### Funnel — Conversion funnels, sales pipelines

**Prompt**: *"Generate a Declart funnel for a 5-stage sales pipeline."*

```toml
kind = "funnel"
title = "Sales Pipeline"

[[items]]
label = "Leads"

[[items]]
label = "Qualified"

[[items]]
label = "Proposal"

[[items]]
label = "Negotiation"

[[items]]
label = "Closed Won"
emphasis = "primary"
```

> **Limit**: 10 stages maximum.

---

## Tips for LLMs

| Rule | Detail |
|------|--------|
| `kind` is required | Always include it as the first field |
| No unknown fields | Don't add `color`, `style`, or other keys not in the spec |
| `emphasis` values | Only `"primary"` or `"secondary"` |
| Fishbone sub-items | Use `[[causes.items]]` with `label` only |
| Matrix quadrants | Always exactly 4 `[[quadrants]]` entries |
| Org chart IDs | Each `id` must be unique; `parent` must match an existing `id` |
| Timeline dates | ISO 8601: `YYYY-MM-DD` only |
| Venn sets | Only 2 or 3 sets supported |

## Validating LLM Output

```bash
declart validate diagram.toml
```

Error messages include field names and hints:

```
invalid value `(missing)` for field `position`
  = hint: When any quadrant has position, all must specify it.
          Valid: top-left, top-right, bottom-left, bottom-right
```

Paste the error back to the LLM to get a corrected TOML.
