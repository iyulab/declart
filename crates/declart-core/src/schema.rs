//! What a declaration of a kind may contain — fields, values, counts and the rules between them.
//!
//! [`schema`] answers it for a kind and view so a caller can write a declaration that parses
//! without trying it first. The numbers and value lists are the ones the parser itself checks (they
//! are named constants in `parse`, read by both), and `tests/schema.rs` holds the rest to the parser
//! in both directions: every field and value listed is accepted, every required field is required,
//! every count and rule is enforced, and every field the parser reads is listed.

use crate::error::DeclartError;
use crate::parse::{
    checked_view, names, COMPARISON_MAX_COLUMNS, COMPARISON_MAX_ROWS, CYCLE_MIN_ITEMS, EMPHASES,
    FISHBONE_MAX_CATEGORIES, FISHBONE_MIN_CATEGORIES, FUNNEL_MAX_ITEMS, FUNNEL_MIN_ITEMS,
    HUB_SPOKE_MIN_SPOKES, KINDS, LINK_DIRECTIONS, LINK_RELATIONS, MATRIX_QUADRANT_COUNT,
    MAX_ITEMS_PER_QUADRANT, QUADRANT_POSITIONS, STATE_MIN_STATES, STATE_ROLES, STATUSES,
    SWIMLANE_MIN_ACTORS, SYSTEM_MAX_GROUPS, SYSTEM_MAX_LINKS, SYSTEM_MAX_NODES, SYSTEM_MIN_NODES,
    SYSTEM_ROLES, TIMELINE_MIN_EVENTS, TRANSITION_KINDS, VENN_MAX_SETS, VENN_MIN_SETS,
};
use serde::Serialize;

/// What kind of value a field holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FieldType {
    /// Free text.
    #[serde(rename = "string")]
    String,
    /// One of the listed `values`.
    #[serde(rename = "enum")]
    Enum,
    /// `YYYY`, `YYYY-MM` or `YYYY-MM-DD`.
    #[serde(rename = "date")]
    Date,
    /// The `id` or `label` of another element of the declaration.
    #[serde(rename = "reference")]
    Reference,
    /// A list of texts.
    #[serde(rename = "string[]")]
    StringList,
}

impl FieldType {
    /// The name the type has in JSON and in the CLI.
    pub fn as_str(self) -> &'static str {
        match self {
            FieldType::String => "string",
            FieldType::Enum => "enum",
            FieldType::Date => "date",
            FieldType::Reference => "reference",
            FieldType::StringList => "string[]",
        }
    }
}

/// One field of the declaration or of an element of a collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Field {
    pub name: &'static str,
    #[serde(rename = "type")]
    pub ty: FieldType,
    /// A declaration without a required field is an error.
    pub required: bool,
    /// The values an `enum` field takes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<&'static str>,
    /// What the field means where the type alone does not say.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'static str>,
}

/// A repeated element of the declaration, written as `[[name]]` in TOML or an array in JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Collection {
    pub name: &'static str,
    /// Fewest elements. A declaration with fewer is an error.
    pub min: usize,
    /// Most elements; `None` for no limit.
    pub max: Option<usize>,
    pub fields: Vec<Field>,
}

/// What a declaration of one kind (and view) may contain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Schema {
    pub kind: &'static str,
    /// The view this schema is for. `None` when the kind has no views, or when no view was asked for
    /// (then the declaration's own `view` decides — see `default_view` and the rules).
    pub view: Option<String>,
    /// Every view the kind accepts. Empty for a kind without a `view` field.
    pub views: Vec<&'static str>,
    /// The view used when a declaration has no `view`. `None` when the choice depends on the
    /// content: a hierarchy with one root is an `org_chart`, with more a `fishbone`.
    pub default_view: Option<&'static str>,
    /// Fields at the top of the declaration.
    pub fields: Vec<Field>,
    pub collections: Vec<Collection>,
    /// What else must hold, in words. Each one is enforced by the parser and checked by a test.
    pub rules: Vec<String>,
}

/// What a declaration of `kind` may contain, for `view`. With no `view` the schema is the one a
/// declaration without a `view` field is read by.
///
/// # Errors
/// [`DeclartError::UnknownKind`] for an unknown kind, [`DeclartError::InvalidValue`] for a view the
/// kind does not have.
pub fn schema(kind: &str, view: Option<&str>) -> Result<Schema, DeclartError> {
    let (known, view) = checked_view(kind, view)?;
    let kind: &'static str = KINDS
        .iter()
        .copied()
        .find(|k| *k == kind)
        .ok_or_else(|| DeclartError::UnknownKind(kind.to_string()))?;
    let default_view = match kind {
        "flow" => Some("process"),
        "tier" => Some("pyramid"),
        _ => None,
    };
    // What is read when no view is given: the default view where there is one.
    let effective = view.or(default_view);

    let mut fields = vec![Field {
        name: "kind",
        ty: FieldType::Enum,
        required: true,
        values: vec![kind],
        note: None,
    }];
    if !known.is_empty() {
        fields.push(Field {
            name: "view",
            ty: FieldType::Enum,
            required: false,
            values: known.to_vec(),
            note: None,
        });
    }
    fields.push(text("title", false, None));

    let mut s = Schema {
        kind,
        view: view.map(str::to_string),
        views: known.to_vec(),
        default_view,
        fields,
        collections: Vec::new(),
        rules: Vec::new(),
    };
    match kind {
        "flow" => flow(&mut s, effective),
        "tier" => tier(&mut s),
        "hierarchy" => hierarchy(&mut s, view),
        "timeline" => timeline(&mut s),
        "matrix" => matrix(&mut s),
        "hub_spoke" => hub_spoke(&mut s),
        "venn" => venn(&mut s),
        "comparison" => comparison(&mut s),
        "state" => state(&mut s),
        "system" => system(&mut s),
        _ => unreachable!("every kind in KINDS is handled"),
    }
    Ok(s)
}

// ---- building blocks --------------------------------------------------------------------------

fn text(name: &'static str, required: bool, note: Option<&'static str>) -> Field {
    Field {
        name,
        ty: FieldType::String,
        required,
        values: Vec::new(),
        note,
    }
}

fn typed(name: &'static str, ty: FieldType, required: bool, note: Option<&'static str>) -> Field {
    Field {
        ty,
        ..text(name, required, note)
    }
}

fn choice(name: &'static str, values: Vec<&'static str>, required: bool) -> Field {
    Field {
        name,
        ty: FieldType::Enum,
        required,
        values,
        note: None,
    }
}

/// `label`, `emphasis` and `status` — what most elements share.
fn item_fields() -> Vec<Field> {
    vec![
        text("label", true, None),
        choice("emphasis", names(EMPHASES), false),
        choice("status", names(STATUSES), false),
    ]
}

fn collection(
    name: &'static str,
    min: usize,
    max: Option<usize>,
    fields: Vec<Field>,
) -> Collection {
    Collection {
        name,
        min,
        max,
        fields,
    }
}

// ---- kinds ------------------------------------------------------------------------------------

fn flow(s: &mut Schema, view: Option<&str>) {
    let mut fields = item_fields();
    let (min, max) = match view {
        Some("cycle") => (CYCLE_MIN_ITEMS, None),
        Some("funnel") => (FUNNEL_MIN_ITEMS, Some(FUNNEL_MAX_ITEMS)),
        Some("swimlane") => (SWIMLANE_MIN_ACTORS, None),
        _ => (1, None),
    };
    if view == Some("swimlane") {
        fields.push(text("actor", true, Some("the lane the step belongs to")));
        s.rules
            .push("`actor` must not be empty on any item".to_string());
        s.rules.push(format!(
            "the items name at least {SWIMLANE_MIN_ACTORS} distinct actors"
        ));
    }
    s.collections.push(collection("items", min, max, fields));
}

fn tier(s: &mut Schema) {
    s.collections
        .push(collection("items", 1, None, item_fields()));
}

fn hierarchy(s: &mut Schema, view: Option<&str>) {
    s.fields.push(text(
        "effect",
        false,
        Some("fishbone: the label at the end of the spine; falls back to `title`"),
    ));
    s.collections.push(collection(
        "nodes",
        1,
        None,
        vec![
            text("label", true, None),
            text("id", false, Some("a short name to refer to the node by")),
            typed(
                "parent",
                FieldType::Reference,
                false,
                Some("the id or label of the node above; a node without one is a root"),
            ),
        ],
    ));
    s.rules.push("node labels are unique".to_string());
    s.rules.push("node ids are unique".to_string());
    s.rules
        .push("`parent` names another node's id or label, never the node itself".to_string());
    match view {
        Some("org_chart") | Some("mind_map") => s
            .rules
            .push("exactly one node has no `parent` (the root)".to_string()),
        Some("fishbone") => s.rules.push(format!(
            "{FISHBONE_MIN_CATEGORIES} to {FISHBONE_MAX_CATEGORIES} nodes have no `parent` (the cause categories)"
        )),
        _ => s.rules.push(format!(
            "without `view`: one node without a `parent` makes an org_chart, more make a fishbone ({FISHBONE_MIN_CATEGORIES} to {FISHBONE_MAX_CATEGORIES} of them)"
        )),
    }
}

fn timeline(s: &mut Schema) {
    s.collections.push(collection(
        "events",
        TIMELINE_MIN_EVENTS,
        None,
        vec![
            typed("date", FieldType::Date, false, Some("a milestone")),
            typed(
                "start",
                FieldType::Date,
                false,
                Some("a period; with `end`"),
            ),
            typed(
                "end",
                FieldType::Date,
                false,
                Some("a period; with `start`"),
            ),
            text("label", true, None),
            choice("emphasis", names(EMPHASES), false),
            choice("status", names(STATUSES), false),
        ],
    ));
    s.rules
        .push("each event has a `date`, or both `start` and `end`, not both kinds".to_string());
    s.rules.push("`end` is not before `start`".to_string());
}

fn matrix(s: &mut Schema) {
    s.fields.push(text("x_axis", true, None));
    s.fields.push(text("y_axis", true, None));
    let mut quadrant = item_fields();
    quadrant.push(choice("position", QUADRANT_POSITIONS.to_vec(), false));
    s.collections.push(collection(
        "quadrants",
        MATRIX_QUADRANT_COUNT,
        Some(MATRIX_QUADRANT_COUNT),
        quadrant,
    ));
    let mut item = item_fields();
    item.push(choice("quadrant", QUADRANT_POSITIONS.to_vec(), true));
    s.collections.push(collection("items", 0, None, item));
    s.rules.push(
        "when any quadrant has a `position`, every quadrant has one and each is used once"
            .to_string(),
    );
    s.rules.push(format!(
        "at most {MAX_ITEMS_PER_QUADRANT} items go in one quadrant"
    ));
}

fn hub_spoke(s: &mut Schema) {
    s.fields.push(text("center", true, None));
    s.collections.push(collection(
        "spokes",
        HUB_SPOKE_MIN_SPOKES,
        None,
        item_fields(),
    ));
}

fn venn(s: &mut Schema) {
    s.collections.push(collection(
        "sets",
        VENN_MIN_SETS,
        Some(VENN_MAX_SETS),
        vec![text("label", true, None)],
    ));
    s.collections.push(collection(
        "intersections",
        0,
        None,
        vec![
            typed(
                "sets",
                FieldType::StringList,
                true,
                Some("the labels of the sets that overlap"),
            ),
            text("label", true, None),
        ],
    ));
}

fn comparison(s: &mut Schema) {
    s.collections.push(collection(
        "columns",
        1,
        Some(COMPARISON_MAX_COLUMNS),
        vec![text("label", true, None)],
    ));
    s.collections.push(collection(
        "rows",
        1,
        Some(COMPARISON_MAX_ROWS),
        vec![
            text("label", true, None),
            text(
                "<column>",
                false,
                Some("one key per declared column, named by the column's label; its value is the cell"),
            ),
        ],
    ));
    s.rules
        .push("a column is not labelled `label` (the row field uses that name)".to_string());
    s.rules
        .push("a row has no key that is not a declared column".to_string());
}

fn state(s: &mut Schema) {
    s.collections.push(collection(
        "states",
        STATE_MIN_STATES,
        None,
        vec![
            text("label", true, None),
            text("id", false, Some("a short name to refer to the state by")),
            choice("role", names(STATE_ROLES), false),
        ],
    ));
    s.collections.push(collection(
        "transitions",
        0,
        None,
        vec![
            typed(
                "from",
                FieldType::Reference,
                true,
                Some("a state's id or label"),
            ),
            typed(
                "to",
                FieldType::Reference,
                true,
                Some("a state's id or label"),
            ),
            text("trigger", false, None),
            choice("type", names(TRANSITION_KINDS), false),
        ],
    ));
    s.rules.push("state labels are unique".to_string());
    s.rules.push("state ids are unique".to_string());
    s.rules
        .push("at most one state has the role `initial`".to_string());
    s.rules
        .push("`from` and `to` name an existing state's id or label".to_string());
}

fn system(s: &mut Schema) {
    s.collections.push(collection(
        "groups",
        0,
        Some(SYSTEM_MAX_GROUPS),
        vec![
            text("id", true, Some("a short name to refer to the group by")),
            text("label", true, None),
            typed(
                "parent",
                FieldType::Reference,
                false,
                Some("the id of the group this one sits inside"),
            ),
        ],
    ));
    s.collections.push(collection(
        "nodes",
        SYSTEM_MIN_NODES,
        Some(SYSTEM_MAX_NODES),
        vec![
            text("id", true, Some("a short name to refer to the node by")),
            text("label", true, None),
            typed(
                "group",
                FieldType::Reference,
                false,
                Some("the id of the group the node sits in"),
            ),
            choice("role", names(SYSTEM_ROLES), false),
            choice("emphasis", names(EMPHASES), false),
            choice("status", names(STATUSES), false),
        ],
    ));
    s.collections.push(collection(
        "links",
        0,
        Some(SYSTEM_MAX_LINKS),
        vec![
            typed(
                "from",
                FieldType::Reference,
                true,
                Some("the id of a node or a group"),
            ),
            typed(
                "to",
                FieldType::Reference,
                true,
                Some("the id of a node or a group"),
            ),
            text("label", false, None),
            choice("direction", names(LINK_DIRECTIONS), false),
            choice("relation", names(LINK_RELATIONS), false),
        ],
    ));
    s.rules
        .push("node ids and group ids are unique, all in one set".to_string());
    s.rules
        .push("`parent` names another group's id, never the group itself".to_string());
    s.rules
        .push("groups nest two deep at most (a group inside a group)".to_string());
    s.rules.push("`group` names a group's id".to_string());
    s.rules
        .push("`from` and `to` name the id of a node or a group".to_string());
    s.rules.push("a link joins two different ends".to_string());
    s.rules
        .push("a link does not join a group to something inside it".to_string());
}
