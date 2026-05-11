use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagramKind {
    Pyramid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramModel {
    pub kind: DiagramKind,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub label: String,
    pub emphasis: Option<Emphasis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Emphasis {
    Highlight,
    Bold,
}
