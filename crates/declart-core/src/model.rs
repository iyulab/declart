#[derive(Debug, Clone, PartialEq)]
pub enum DiagramKind {
    Pyramid,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Emphasis {
    Primary,
    Secondary,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub label: String,
    pub emphasis: Option<Emphasis>,
}

#[derive(Debug, Clone)]
pub struct DiagramModel {
    pub kind: DiagramKind,
    pub title: Option<String>,
    pub items: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagram_model_holds_items() {
        let model = DiagramModel {
            kind: DiagramKind::Pyramid,
            title: Some("Test".to_string()),
            items: vec![
                Item { label: "Top".to_string(), emphasis: None },
                Item { label: "Bottom".to_string(), emphasis: Some(Emphasis::Primary) },
            ],
        };
        assert_eq!(model.items.len(), 2);
        assert_eq!(model.items[0].label, "Top");
        assert_eq!(model.items[1].emphasis, Some(Emphasis::Primary));
        assert_eq!(model.kind, DiagramKind::Pyramid);
    }
}
