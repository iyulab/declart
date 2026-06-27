# Using Claude or GPT to Generate Declart Diagrams

Declart's TOML format is designed for LLM generation. The structure is explicit and validated — an LLM can produce a well-formed diagram in one shot, and `declart validate` catches any mistakes before rendering.

<button onclick="(async()=>{const b=this;try{b.textContent='⏳ Loading...';const r=await fetch('https://raw.githubusercontent.com/iyulab/declart/main/docs/src/llm-guide.md');const t=await r.text();await navigator.clipboard.writeText(t);b.textContent='✅ Copied!';}catch(e){b.textContent='❌ Failed: '+e.message;}setTimeout(()=>b.textContent='📋 Copy to clipboard',3000);})()" style="margin:0.5em 0 1.5em;padding:6px 14px;cursor:pointer;border-radius:4px;border:1px solid currentColor;background:transparent;font-size:0.9em;opacity:0.85">📋 Copy to clipboard</button>

## Why Declart works well with LLMs

- **No layout decisions**: the LLM only writes content (labels, structure). The engine handles all visual choices.
- **Strict schema**: `deny_unknown_fields` means invalid keys are caught immediately. The LLM cannot silently produce a broken diagram.
- **Explicit kind**: the `kind` field tells the parser exactly what to expect. No ambiguity.
- **Short format**: a typical diagram is 5–20 lines of TOML.

## Kind and View

Declart v0.16+ uses a two-level structure:

- **`kind`** — the data contract (determines which fields are valid)
- **`view`** — the semantic intent (determines how the engine renders it)

| kind | views | Notes |
|------|-------|-------|
| `flow` | `process` (default), `cycle`, `funnel`, `swimlane` | `view` optional — defaults to `process`. `swimlane` requires an `actor` per item |
| `tier` | `pyramid` (default) | Ranked levels — `view` optional |
| `hierarchy` | `org_chart`, `fishbone`, `mind_map` | `view` optional — `org_chart`/`fishbone` auto-selected by root count; `mind_map` must be explicit |
| `timeline` | — | No view field |
| `matrix` | — | No view field |
| `hub_spoke` | — | No view field |
| `venn` | — | No view field |
| `comparison` | — | No view field |
| `state` | — | No view field. States + directed transitions |

## Workflow

```
1. Prompt LLM → TOML output
2. declart validate diagram.toml   # catches errors with clear messages
3. declart render diagram.toml     # produces SVG
```

If `validate` fails, paste the error message back to the LLM and ask it to fix the specific field.

---

## Prompt Templates

### Tier (Pyramid) — Hierarchies, priority layers

**Prompt**: *"Generate a Declart TOML diagram showing Maslow's hierarchy of needs as a pyramid. Use kind = 'tier', include a title, and list the 5 levels as items from top (apex) to bottom (base)."*

```toml
kind = "tier"
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

**Prompt**: *"Create a Declart TOML diagram for a 4-step CI/CD pipeline. Use kind = 'flow' (process view is the default)."*

```toml
kind = "flow"
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

**Prompt**: *"Generate a Declart TOML diagram for the PDCA improvement cycle. Use kind = 'flow' and view = 'cycle'."*

```toml
kind = "flow"
view = "cycle"
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

**Item placement (BCG / Gartner)**: add an optional `[[items]]` array to classify items into
quadrants. Each item needs a `quadrant` (one of the four positions) and may carry `emphasis`/`status`.
Assignment is by category, never coordinates. Max 6 items per quadrant.

```toml
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
quadrant = "bottom-left"
status = "critical"
```

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

> **Rule**: dates accept `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. Partial forms are placed at the start of that year/month. Declart sorts events automatically.

---

### Fishbone / Ishikawa — Root cause analysis

**Prompt**: *"Generate a Declart fishbone diagram where the effect is 'Slow API Response' with 4 cause categories and sub-causes. Use kind = 'hierarchy' and view = 'fishbone'. Each cause category is a root node; sub-causes have parent = the category label."*

```toml
kind = "hierarchy"
view = "fishbone"
title = "Slow API Response"

[[nodes]]
label = "Database"

[[nodes]]
label = "Missing indexes"
parent = "Database"

[[nodes]]
label = "N+1 queries"
parent = "Database"

[[nodes]]
label = "Network"

[[nodes]]
label = "High latency"
parent = "Network"

[[nodes]]
label = "Code"

[[nodes]]
label = "Blocking I/O"
parent = "Code"

[[nodes]]
label = "Infrastructure"
```

> **Structure**: Root nodes (no `parent`) become cause categories on the spine. Child nodes become sub-causes. The `effect` field is rendered as the spine-end effect label; if `effect` is omitted, `title` is used as fallback.
>
> **Limit**: 2–20 root nodes (cause categories). Recommend 8 or fewer for readability.

---

### Org Chart — Hierarchical trees

**Prompt**: *"Create a Declart org chart for a small engineering team with a CTO at the top. Use kind = 'hierarchy'. With a single root node, the engine automatically renders as an org chart."*

```toml
kind = "hierarchy"
title = "Engineering Team"

[[nodes]]
label = "CTO"

[[nodes]]
label = "Frontend Lead"
parent = "CTO"

[[nodes]]
label = "Backend Lead"
parent = "CTO"

[[nodes]]
label = "FE Developer"
parent = "Frontend Lead"

[[nodes]]
label = "BE Developer"
parent = "Backend Lead"
```

> **Rule**: exactly one root node (no `parent`). `parent` references another node's `id` (preferred) or `label`. For stable references that survive label renames, add `id = "stable-key"` to each node and use that in `parent`. To explicitly select the view: `view = "org_chart"`.

---

### Funnel — Conversion funnels, sales pipelines

**Prompt**: *"Generate a Declart funnel for a 5-stage sales pipeline. Use kind = 'flow' and view = 'funnel'."*

```toml
kind = "flow"
view = "funnel"
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

> **Limit**: 2–10 stages.

---

### Comparison — Feature matrices, trade-off tables

**Prompt**: *"Generate a Declart comparison table for three JavaScript frameworks across four criteria."*

```toml
kind = "comparison"
title = "JavaScript Framework Comparison"

[[columns]]
label = "Performance"

[[columns]]
label = "Ecosystem"

[[rows]]
label = "React"
Performance = "★★★★"
Ecosystem = "★★★★★"

[[rows]]
label = "Vue"
Performance = "★★★★"
Ecosystem = "★★★"

[[rows]]
label = "Svelte"
Performance = "★★★★★"
```

> **Limits**: 1–10 rows, 1–8 columns. Declare `[[columns]]` first for column order. Cell values are inline in each row, keyed by column label. Missing cells are rendered empty. Column label must not be `"label"` (reserved). Use TOML quoted keys (`"My Column" = "val"`) if a column name contains spaces.

---

### Swimlane — Cross-actor process flows

**Prompt**: *"Generate a Declart swimlane diagram for an order-processing flow across Customer, System, and Payment Gateway. Use kind = 'flow' and view = 'swimlane'. Each item needs an `actor`."*

```toml
kind = "flow"
view = "swimlane"
title = "Order Processing"

[[items]]
actor = "Customer"
label = "Place Order"

[[items]]
actor = "System"
label = "Check Inventory"

[[items]]
actor = "Payment Gateway"
label = "Process Payment"

[[items]]
actor = "System"
label = "Confirm Order"
```

> **Rule**: every item requires an `actor`; at least 2 distinct actors. Items are grouped into horizontal lanes by actor, ordered top→down. `actor` is ignored by all other flow views.

---

### Mind Map — Radial topic exploration

**Prompt**: *"Create a Declart mind map for Machine Learning concepts. Use kind = 'hierarchy' and view = 'mind_map' with a single root."*

```toml
kind = "hierarchy"
view = "mind_map"
title = "Machine Learning"

[[nodes]]
label = "ML Concepts"

[[nodes]]
label = "Supervised"
parent = "ML Concepts"

[[nodes]]
label = "Unsupervised"
parent = "ML Concepts"

[[nodes]]
label = "Classification"
parent = "Supervised"
```

> **Rule**: exactly one root node. `mind_map` must be set explicitly (never auto-selected). The root is centered; its subtree radiates outward. Use for learning maps, brainstorms, topic exploration.

---

### State — Lifecycle states and transitions

**Prompt**: *"Generate a Declart state diagram for an order lifecycle. Use kind = 'state' with states and directed transitions. Mark the start state with role = 'initial' and end states with role = 'terminal'."*

```toml
kind = "state"
title = "Order Lifecycle"

[[states]]
id = "pending"
label = "Pending"
role = "initial"

[[states]]
id = "processing"
label = "Processing"

[[states]]
id = "done"
label = "Completed"
role = "terminal"

[[states]]
id = "cancelled"
label = "Cancelled"
role = "terminal"

[[transitions]]
from = "pending"
to = "processing"
trigger = "Order Received"

[[transitions]]
from = "processing"
to = "done"
trigger = "Payment OK"

[[transitions]]
from = "processing"
to = "cancelled"
trigger = "Payment Failed"
type = "exception"
```

> **Rule**: at least 2 states. At most one `role = "initial"`; multiple `"terminal"` allowed. Reference states in `from`/`to` by `id` (preferred) or `label`. `trigger` is optional (unconditional transition); `type = "exception"` marks error paths. Self-loops (`from` = `to`) are valid.

---

### Status signals — health / severity per item

**Prompt**: *"Generate a Declart flow showing a release pipeline where each step has a health status. Use the `status` field with values success / warning / critical / normal. `status` is orthogonal to `emphasis`."*

```toml
kind = "flow"
title = "Release Pipeline Health"

[[items]]
label = "Build"
status = "success"

[[items]]
label = "Test"
status = "warning"

[[items]]
label = "Deploy"
emphasis = "primary"
status = "critical"

[[items]]
label = "Monitor"
status = "normal"
```

> **Rule**: `status` ∈ `success` / `normal` / `warning` / `critical`. It is the "traffic-light"
> signal common in reports, and is **orthogonal to `emphasis`** — an item can be both `primary` and
> `critical`. The engine renders a corner marker dual-encoded by color **and** shape
> (success=circle, warning=triangle, critical=diamond), so it reads in monochrome and for colorblind
> users. `normal` and omitted `status` render no marker. Supported on `flow`, `tier`,
> `hub_spoke`, and `matrix` items. Declaring `status` on other kinds is a parse error.

---

## Tips for LLMs

| Rule | Detail |
|------|--------|
| `kind` is required | Always include it as the first field |
| `view` is optional | Omit to use the default; include to declare intent explicitly |
| No unknown fields | Don't add `color`, `style`, or other keys not in the spec |
| `emphasis` values | Only `"primary"` or `"secondary"` |
| `status` values | Only `"success"`, `"normal"`, `"warning"`, `"critical"`. Orthogonal to `emphasis`. Supported on `flow`, `tier`, `hub_spoke`, `matrix` |
| Flow views | `kind = "flow"` + `view`: `process` (default), `cycle`, `funnel`, `swimlane` |
| Swimlane actors | `view = "swimlane"` requires an `actor` on every item; ≥2 distinct actors |
| Tier views | `kind = "tier"` + `view`: `pyramid` (default and only) |
| Hierarchy nodes | `label` must be unique; `parent` references `id` (preferred) or `label` of another node |
| Hierarchy `id` | Add `id = "key"` to nodes for stable `parent` references that survive label renames |
| Hierarchy views | `org_chart`, `fishbone`, `mind_map`. 1 root → `org_chart`; 2+ roots → `fishbone` (auto). `mind_map` must be explicit |
| Fishbone `effect` | Rendered as the spine-end effect label; falls back to `title` if omitted |
| Matrix quadrants | Always exactly 4 `[[quadrants]]` entries |
| Matrix items | Optional `[[items]]` with `quadrant` (position) to classify into quadrants (BCG/Gartner). Max 6 per quadrant |
| Timeline dates | ISO 8601: `YYYY`, `YYYY-MM`, or `YYYY-MM-DD` |
| Venn sets | Only 2 or 3 sets supported |
| Comparison cells | Column label in each row must match an existing `[[columns]]` label |
| State roles | At most one `role = "initial"`; multiple `"terminal"` allowed. `from`/`to` reference state `id` or `label` |

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
