use serde::Deserialize;

/// Minimal probe to extract the `kind` field without rejecting extra fields.
#[derive(Deserialize, Debug)]
pub struct KindProbe {
    pub kind: String,
}

/// Raw representation of a flow diagram item. Includes `actor` for the swimlane view.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFlowItem {
    pub label: String,
    pub emphasis: Option<String>,
    pub status: Option<String>,
    pub actor: Option<String>,
}

/// Raw representation for `flow` kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFlowDiagram {
    pub kind: String,
    pub view: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub items: Vec<RawFlowItem>,
}

/// Raw representation for `tier` kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawTierDiagram {
    pub kind: String,
    pub view: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub items: Vec<RawItem>,
}

/// Raw representation for the matrix kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawMatrixDiagram {
    pub kind: String, // consumed by deny_unknown_fields; value already known from KindProbe
    pub title: Option<String>,
    pub x_axis: String,
    pub y_axis: String,
    #[serde(default)]
    pub quadrants: Vec<RawQuadrant>,
    #[serde(default)]
    pub items: Vec<RawMatrixItem>,
}

/// An item classified into a matrix quadrant. `quadrant` references a quadrant by position name.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawMatrixItem {
    pub label: String,
    pub emphasis: Option<String>,
    pub status: Option<String>,
    pub quadrant: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawItem {
    pub label: String,
    pub emphasis: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawQuadrant {
    pub label: String,
    pub emphasis: Option<String>,
    pub status: Option<String>,
    pub position: Option<String>,
}

/// Raw representation for the hub_spoke kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawHubSpokeDiagram {
    pub kind: String,
    pub title: Option<String>,
    pub center: String,
    #[serde(default)]
    pub spokes: Vec<RawItem>,
}

/// Raw representation for the venn kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawVennDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub sets: Vec<RawVennSet>,
    #[serde(default)]
    pub intersections: Vec<RawVennIntersection>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawVennSet {
    pub label: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawVennIntersection {
    pub sets: Vec<String>,
    pub label: String,
}

/// Raw representation for the timeline kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawTimelineDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub events: Vec<RawTimelineEvent>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawTimelineEvent {
    pub date: String,
    pub label: String,
}

/// Raw representation for `hierarchy` kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawHierarchyDiagram {
    pub kind: String,
    pub view: Option<String>,
    pub title: Option<String>,
    pub effect: Option<String>,
    #[serde(default)]
    pub nodes: Vec<RawHierarchyNode>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawHierarchyNode {
    pub label: String,
    pub id: Option<String>,
    pub parent: Option<String>,
}

/// Raw representation for the comparison kind.
/// Cells are declared inline within each row entry (no separate [[cells]] array).
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub rows: Vec<RawComparisonRow>,
    #[serde(default)]
    pub columns: Vec<RawComparisonColumn>,
}

/// A row entry in the comparison table. `label` identifies the row; all other string
/// keys are treated as cell values keyed by column label.
#[derive(Deserialize, Debug)]
pub struct RawComparisonRow {
    pub label: String,
    #[serde(flatten)]
    pub cells: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonColumn {
    pub label: String,
}

/// Raw representation for `state` kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawStateDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub states: Vec<RawStateNode>,
    #[serde(default)]
    pub transitions: Vec<RawStateTransition>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawStateNode {
    pub label: String,
    pub id: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawStateTransition {
    pub from: String,
    pub to: String,
    pub trigger: Option<String>,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
}
