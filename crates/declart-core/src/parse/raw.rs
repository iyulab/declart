use serde::Deserialize;

/// Minimal probe to extract the `kind` field without rejecting extra fields.
#[derive(Deserialize, Debug)]
pub struct KindProbe {
    pub kind: String,
}

/// Raw representation for `flow` kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFlowDiagram {
    pub kind: String,
    pub view: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub items: Vec<RawItem>,
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
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawItem {
    pub label: String,
    pub emphasis: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawQuadrant {
    pub label: String,
    pub emphasis: Option<String>,
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
