use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawDiagram {
    pub kind: String,
    pub title: Option<String>,
    #[serde(default)]
    pub items: Vec<RawItem>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawItem {
    pub label: String,
    pub emphasis: Option<String>,
}
