use serde::Deserialize;

/// Minimal probe to extract the `kind` field without rejecting extra fields.
#[derive(Deserialize, Debug)]
pub struct KindProbe {
    pub kind: String,
}

/// Raw representation for kinds that use `[[items]]`: pyramid, process, cycle.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawItemsDiagram {
    pub kind: String,
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

/// Raw representation for the org_chart kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawOrgChartDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub nodes: Vec<RawOrgChartNode>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawOrgChartNode {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
}

/// Raw representation for the fishbone kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFishboneDiagram {
    pub kind: String,
    pub title: Option<String>,
    pub effect: String,
    #[serde(default)]
    pub causes: Vec<RawFishboneCause>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFishboneCause {
    pub label: String,
    #[serde(default)]
    pub items: Vec<RawFishboneItem>,
}

/// Sub-cause item in a fishbone diagram. Intentionally has only `label` — emphasis is not
/// supported on fishbone sub-items per spec.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawFishboneItem {
    pub label: String,
}

/// Raw representation for the comparison kind.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub rows: Vec<RawComparisonRow>,
    #[serde(default)]
    pub columns: Vec<RawComparisonColumn>,
    #[serde(default)]
    pub cells: Vec<RawComparisonCell>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonRow {
    pub label: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonColumn {
    pub label: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawComparisonCell {
    pub row: String,
    pub column: String,
    #[serde(default)]
    pub value: String,
}
