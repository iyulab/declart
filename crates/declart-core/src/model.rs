/// Optional visual weight hint applied to an item.
#[derive(Debug, Clone, PartialEq)]
pub enum Emphasis {
    /// Most important item — rendered with outline stroke and bold text.
    Primary,
    /// Secondary importance — rendered with a lighter color tint.
    Secondary,
}

/// A labeled item with an optional emphasis hint. Used by Sequence diagrams and as spokes/quadrants.
#[derive(Debug, Clone)]
pub struct Item {
    pub label: String,
    pub emphasis: Option<Emphasis>,
}

/// Model for diagram kinds that use a flat list of items (kept for internal compatibility).
#[derive(Debug, Clone)]
pub struct ItemsDiagram {
    pub title: Option<String>,
    pub items: Vec<Item>,
}

/// View variant for a Sequence diagram — determines how the flat item list is rendered.
#[derive(Debug, Clone, PartialEq)]
pub enum SequenceView {
    Process,
    Cycle,
    Funnel,
    Pyramid,
}

/// Model for Sequence diagrams (process / cycle / funnel / pyramid views).
#[derive(Debug, Clone)]
pub struct SequenceDiagram {
    pub title: Option<String>,
    pub view: SequenceView,
    pub items: Vec<Item>,
}

/// View variant for a Hierarchy diagram — determines how the node tree is rendered.
#[derive(Debug, Clone, PartialEq)]
pub enum HierarchyView {
    OrgChart,
    Fishbone,
}

/// A single node in a hierarchy diagram. `label` is the unique identifier;
/// `parent` references the parent node's label (`None` for the root).
#[derive(Debug, Clone)]
pub struct HierarchyNode {
    pub label: String,
    pub parent: Option<String>,
}

/// Model for Hierarchy diagrams (org_chart / fishbone views).
#[derive(Debug, Clone)]
pub struct HierarchyDiagram {
    pub title: Option<String>,
    pub view: HierarchyView,
    pub nodes: Vec<HierarchyNode>,
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

/// Model for the Hub-and-Spoke kind.
#[derive(Debug, Clone)]
pub struct HubSpokeDiagram {
    pub title: Option<String>,
    pub center: String,
    pub spokes: Vec<Item>,
}

/// A set in a Venn diagram.
#[derive(Debug, Clone)]
pub struct VennSet {
    pub label: String,
}

/// A labeled intersection region in a Venn diagram.
#[derive(Debug, Clone)]
pub struct VennIntersection {
    pub sets: Vec<String>,
    pub label: String,
}

/// Model for the Venn diagram kind (2–3 sets).
#[derive(Debug, Clone)]
pub struct VennDiagram {
    pub title: Option<String>,
    pub sets: Vec<VennSet>,
    pub intersections: Vec<VennIntersection>,
}

/// A single event on a timeline.
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub date: String,
    pub label: String,
}

/// Model for the Timeline kind.
#[derive(Debug, Clone)]
pub struct TimelineDiagram {
    pub title: Option<String>,
    pub events: Vec<TimelineEvent>,
}

/// A single cause branch in a fishbone diagram (may have sub-causes).
/// Internal bridge type used by the render layer.
#[derive(Debug, Clone)]
pub(crate) struct FishboneCause {
    pub label: String,
    pub items: Vec<Item>,
}

/// Internal bridge model for the Fishbone render path.
#[derive(Debug, Clone)]
pub(crate) struct FishboneDiagram {
    pub title: Option<String>,
    pub effect: String,
    pub causes: Vec<FishboneCause>,
}

/// A single node in an org chart. Internal bridge type used by the render layer.
#[derive(Debug, Clone)]
pub(crate) struct OrgChartNode {
    pub label: String,
    /// `None` for the root node; references `label` of the parent node for all others.
    pub parent: Option<String>,
}

/// Internal bridge model for the OrgChart render path.
#[derive(Debug, Clone)]
pub(crate) struct OrgChartDiagram {
    pub title: Option<String>,
    pub nodes: Vec<OrgChartNode>,
}

/// A single cell in a comparison table.
#[derive(Debug, Clone)]
pub struct ComparisonCell {
    /// Label of the row this cell belongs to.
    pub row: String,
    /// Label of the column this cell belongs to.
    pub column: String,
    /// Display value for this cell. Empty string if omitted.
    pub value: String,
}

/// Model for the Comparison table kind — rows × columns with optional cell values.
#[derive(Debug, Clone)]
pub struct ComparisonDiagram {
    pub title: Option<String>,
    /// Row labels (items being compared).
    pub rows: Vec<String>,
    /// Column labels (criteria / attributes).
    pub columns: Vec<String>,
    /// Cell values. Missing (row, column) combinations are treated as empty.
    pub cells: Vec<ComparisonCell>,
}

/// The parsed, validated representation of a diagram declaration.
#[derive(Debug, Clone)]
pub enum Diagram {
    Sequence(SequenceDiagram),
    Hierarchy(HierarchyDiagram),
    Matrix(MatrixDiagram),
    HubSpoke(HubSpokeDiagram),
    Venn(VennDiagram),
    Timeline(TimelineDiagram),
    Comparison(ComparisonDiagram),
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

    #[test]
    fn sequence_diagram_holds_view_and_items() {
        let d = SequenceDiagram {
            title: Some("PDCA".to_string()),
            view: SequenceView::Cycle,
            items: vec![Item { label: "Plan".to_string(), emphasis: None }],
        };
        assert_eq!(d.view, SequenceView::Cycle);
        assert_eq!(d.items[0].label, "Plan");
    }

    #[test]
    fn hierarchy_diagram_holds_view_and_nodes() {
        let d = HierarchyDiagram {
            title: Some("Org".to_string()),
            view: HierarchyView::OrgChart,
            nodes: vec![HierarchyNode { label: "CEO".to_string(), parent: None }],
        };
        assert_eq!(d.view, HierarchyView::OrgChart);
        assert!(d.nodes[0].parent.is_none());
    }
}
