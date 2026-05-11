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

/// Model for diagram kinds that use a flat list of items: Pyramid, Process, Cycle.
#[derive(Debug, Clone)]
pub struct ItemsDiagram {
    pub title: Option<String>,
    pub items: Vec<Item>,
}

/// Model for the Matrix 2×2 kind.
#[derive(Debug, Clone)]
pub struct MatrixDiagram {
    pub title: Option<String>,
    pub x_axis: String,
    pub y_axis: String,
    /// Exactly 4 quadrants: [top-left, top-right, bottom-left, bottom-right].
    pub quadrants: Vec<Item>,
}

/// The parsed, validated representation of a diagram declaration.
#[derive(Debug, Clone)]
pub enum Diagram {
    Pyramid(ItemsDiagram),
    Process(ItemsDiagram),
    Cycle(ItemsDiagram),
    Matrix(MatrixDiagram),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_diagram_holds_items() {
        let d = ItemsDiagram {
            title: Some("Test".to_string()),
            items: vec![
                Item { label: "Top".to_string(), emphasis: None },
                Item { label: "Bottom".to_string(), emphasis: Some(Emphasis::Primary) },
            ],
        };
        assert_eq!(d.items.len(), 2);
        assert_eq!(d.items[0].label, "Top");
        assert_eq!(d.items[1].emphasis, Some(Emphasis::Primary));
    }

    #[test]
    fn matrix_diagram_holds_quadrants() {
        let d = MatrixDiagram {
            title: None,
            x_axis: "Effort".to_string(),
            y_axis: "Impact".to_string(),
            quadrants: (0..4)
                .map(|i| Item { label: format!("Q{}", i + 1), emphasis: None })
                .collect(),
        };
        assert_eq!(d.quadrants.len(), 4);
        assert_eq!(d.x_axis, "Effort");
    }
}
