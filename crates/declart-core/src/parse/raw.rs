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
}
