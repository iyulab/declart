#[derive(Debug, Clone)]
pub enum DiagramKind {
    Pyramid,
}

#[derive(Debug, Clone)]
pub struct DiagramModel {
    pub kind: DiagramKind,
    pub title: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub label: String,
    pub emphasis: Option<Emphasis>,
}

#[derive(Debug, Clone)]
pub enum Emphasis {
    Primary,
    Secondary,
}
