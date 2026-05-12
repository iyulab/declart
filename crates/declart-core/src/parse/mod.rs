mod raw;

use crate::error::DeclartError;
use crate::model::{
    ComparisonCell, ComparisonDiagram, Diagram, Emphasis,
    HierarchyDiagram, HierarchyNode, HierarchyView,
    HubSpokeDiagram, Item, MatrixDiagram,
    SequenceDiagram, SequenceView, TimelineDiagram, TimelineEvent,
    VennDiagram, VennIntersection, VennSet,
};

pub fn parse(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = toml::from_str(input)?;
    match probe.kind.as_str() {
        "sequence" => { let raw: raw::RawSequenceDiagram = toml::from_str(input)?; validate_sequence(raw) }
        "hierarchy" => { let raw: raw::RawHierarchyDiagram = toml::from_str(input)?; validate_hierarchy(raw) }
        "matrix" => { let raw: raw::RawMatrixDiagram = toml::from_str(input)?; validate_matrix(raw) }
        "hub_spoke" => { let raw: raw::RawHubSpokeDiagram = toml::from_str(input)?; validate_hub_spoke(raw) }
        "venn" => { let raw: raw::RawVennDiagram = toml::from_str(input)?; validate_venn(raw) }
        "timeline" => { let raw: raw::RawTimelineDiagram = toml::from_str(input)?; validate_timeline(raw) }
        "comparison" => { let raw: raw::RawComparisonDiagram = toml::from_str(input)?; validate_comparison(raw) }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

pub fn parse_json(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = serde_json::from_str(input)?;
    match probe.kind.as_str() {
        "sequence" => { let raw: raw::RawSequenceDiagram = serde_json::from_str(input)?; validate_sequence(raw) }
        "hierarchy" => { let raw: raw::RawHierarchyDiagram = serde_json::from_str(input)?; validate_hierarchy(raw) }
        "matrix" => { let raw: raw::RawMatrixDiagram = serde_json::from_str(input)?; validate_matrix(raw) }
        "hub_spoke" => { let raw: raw::RawHubSpokeDiagram = serde_json::from_str(input)?; validate_hub_spoke(raw) }
        "venn" => { let raw: raw::RawVennDiagram = serde_json::from_str(input)?; validate_venn(raw) }
        "timeline" => { let raw: raw::RawTimelineDiagram = serde_json::from_str(input)?; validate_timeline(raw) }
        "comparison" => { let raw: raw::RawComparisonDiagram = serde_json::from_str(input)?; validate_comparison(raw) }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

/// Auto-detects input format and parses into a validated [`Diagram`].
///
/// If the trimmed input starts with `{`, JSON parsing is used; otherwise TOML.
pub fn parse_auto(input: &str) -> Result<Diagram, DeclartError> {
    if input.trim_start().starts_with('{') { parse_json(input) } else { parse(input) }
}

fn parse_emphasis(s: Option<&str>) -> Result<Option<Emphasis>, DeclartError> {
    match s {
        None => Ok(None),
        Some("primary") => Ok(Some(Emphasis::Primary)),
        Some("secondary") => Ok(Some(Emphasis::Secondary)),
        Some(other) => Err(DeclartError::InvalidValue {
            field: "emphasis".to_string(),
            value: other.to_string(),
            hint: "Valid values are: primary, secondary".to_string(),
        }),
    }
}

fn parse_items(raw_items: Vec<raw::RawItem>) -> Result<Vec<Item>, DeclartError> {
    raw_items.into_iter().map(|item| {
        let emphasis = parse_emphasis(item.emphasis.as_deref())?;
        Ok(Item { label: item.label, emphasis })
    }).collect()
}

fn validate_sequence(raw: raw::RawSequenceDiagram) -> Result<Diagram, DeclartError> {
    if raw.items.is_empty() { return Err(DeclartError::EmptyItems); }
    let view = match raw.view.as_deref() {
        None | Some("process") => SequenceView::Process,
        Some("cycle") => SequenceView::Cycle,
        Some("funnel") => SequenceView::Funnel,
        Some("pyramid") => SequenceView::Pyramid,
        Some(other) => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: other.to_string(),
            hint: "Valid view values for sequence: process, cycle, funnel, pyramid".to_string(),
        }),
    };
    let n = raw.items.len();
    match view {
        SequenceView::Cycle if n < 2 =>
            return Err(DeclartError::TooFewItems { kind: "sequence/cycle", min: 2, got: n }),
        SequenceView::Funnel if n < 2 =>
            return Err(DeclartError::TooFewItems { kind: "sequence/funnel", min: 2, got: n }),
        SequenceView::Funnel if n > 10 =>
            return Err(DeclartError::InvalidValue {
                field: "items".to_string(),
                value: format!("{n} items"),
                hint: "Funnel view supports at most 10 stages.".to_string(),
            }),
        _ => {}
    }
    let items = parse_items(raw.items)?;
    Ok(Diagram::Sequence(SequenceDiagram { title: raw.title, view, items }))
}

fn validate_hierarchy(raw: raw::RawHierarchyDiagram) -> Result<Diagram, DeclartError> {
    if raw.nodes.is_empty() { return Err(DeclartError::EmptyItems); }
    let labels: std::collections::HashSet<&str> = raw.nodes.iter().map(|n| n.label.as_str()).collect();
    if labels.len() != raw.nodes.len() {
        return Err(DeclartError::InvalidValue {
            field: "label".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each node label must be unique within the diagram".to_string(),
        });
    }
    for node in &raw.nodes {
        if let Some(parent) = &node.parent {
            if !labels.contains(parent.as_str()) {
                return Err(DeclartError::InvalidValue {
                    field: "parent".to_string(),
                    value: parent.clone(),
                    hint: "parent must reference an existing node label".to_string(),
                });
            }
            if node.label == *parent {
                return Err(DeclartError::InvalidValue {
                    field: "parent".to_string(),
                    value: parent.clone(),
                    hint: "A node cannot be its own parent".to_string(),
                });
            }
        }
    }
    let root_count = raw.nodes.iter().filter(|n| n.parent.is_none()).count();
    if root_count == 0 {
        return Err(DeclartError::InvalidValue {
            field: "parent".to_string(),
            value: "(all nodes have parents)".to_string(),
            hint: "At least one root node (with no parent) is required".to_string(),
        });
    }
    let view = match raw.view.as_deref() {
        None => if root_count == 1 { HierarchyView::OrgChart } else { HierarchyView::Fishbone },
        Some("org_chart") => HierarchyView::OrgChart,
        Some("fishbone") => HierarchyView::Fishbone,
        Some(other) => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: other.to_string(),
            hint: "Valid view values for hierarchy: org_chart, fishbone".to_string(),
        }),
    };
    match view {
        HierarchyView::OrgChart if root_count != 1 =>
            return Err(DeclartError::InvalidValue {
                field: "parent".to_string(),
                value: format!("{root_count} root nodes"),
                hint: "org_chart view requires exactly one root node".to_string(),
            }),
        HierarchyView::Fishbone if root_count < 2 =>
            return Err(DeclartError::TooFewItems { kind: "hierarchy/fishbone", min: 2, got: root_count }),
        HierarchyView::Fishbone if root_count > 20 =>
            return Err(DeclartError::InvalidValue {
                field: "nodes".to_string(),
                value: format!("{root_count} root nodes"),
                hint: "fishbone view supports at most 20 cause categories".to_string(),
            }),
        _ => {}
    }
    let nodes = raw.nodes.into_iter().map(|n| HierarchyNode { label: n.label, parent: n.parent }).collect();
    Ok(Diagram::Hierarchy(HierarchyDiagram { title: raw.title, view, nodes }))
}

fn validate_matrix(raw: raw::RawMatrixDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "matrix");
    if raw.quadrants.len() != 4 {
        return Err(DeclartError::InvalidQuadrantCount(raw.quadrants.len()));
    }

    let has_positions = raw.quadrants.iter().any(|q| q.position.is_some());

    let quadrants = if has_positions {
        // position-based: all four must declare a distinct position
        let mut slots: [Option<Item>; 4] = [None, None, None, None];
        for q in raw.quadrants {
            let pos = q.position.ok_or_else(|| DeclartError::InvalidValue {
                field: "position".to_string(),
                value: "(missing)".to_string(),
                hint: "When any quadrant has position, all must specify it. Valid: top-left, top-right, bottom-left, bottom-right".to_string(),
            })?;
            let slot = match pos.as_str() {
                "top-left" => 0_usize,
                "top-right" => 1,
                "bottom-left" => 2,
                "bottom-right" => 3,
                other => return Err(DeclartError::InvalidValue {
                    field: "position".to_string(),
                    value: other.to_string(),
                    hint: "Valid values: top-left, top-right, bottom-left, bottom-right".to_string(),
                }),
            };
            if slots[slot].is_some() {
                return Err(DeclartError::InvalidValue {
                    field: "position".to_string(),
                    value: pos,
                    hint: "Duplicate quadrant position".to_string(),
                });
            }
            slots[slot] = Some(Item { label: q.label, emphasis: parse_emphasis(q.emphasis.as_deref())? });
        }
        slots.into_iter().map(|s| s.unwrap()).collect::<Vec<Item>>()
    } else {
        // index order (backward compatible)
        raw.quadrants
            .into_iter()
            .map(|q| -> Result<Item, DeclartError> {
                Ok(Item { label: q.label, emphasis: parse_emphasis(q.emphasis.as_deref())? })
            })
            .collect::<Result<Vec<Item>, DeclartError>>()?
    };

    Ok(Diagram::Matrix(MatrixDiagram {
        title: raw.title,
        x_axis: raw.x_axis,
        y_axis: raw.y_axis,
        quadrants,
    }))
}

fn validate_venn(raw: raw::RawVennDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "venn");
    let n = raw.sets.len();
    if n < 2 || n > 3 {
        return Err(DeclartError::InvalidValue {
            field: "sets".to_string(),
            value: format!("{} sets", n),
            hint: "Venn diagrams require exactly 2 or 3 sets".to_string(),
        });
    }
    let sets = raw.sets.into_iter().map(|s| VennSet { label: s.label }).collect();
    let intersections = raw
        .intersections
        .into_iter()
        .map(|i| VennIntersection { sets: i.sets, label: i.label })
        .collect();
    Ok(Diagram::Venn(VennDiagram { title: raw.title, sets, intersections }))
}

fn validate_timeline(raw: raw::RawTimelineDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "timeline");
    if raw.events.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "timeline", min: 2, got: raw.events.len() });
    }
    for event in &raw.events {
        if normalize_date(&event.date).is_none() {
            return Err(DeclartError::InvalidValue {
                field: "date".to_string(),
                value: event.date.clone(),
                hint: "Date must be YYYY, YYYY-MM, or YYYY-MM-DD".to_string(),
            });
        }
    }
    let mut events: Vec<TimelineEvent> = raw
        .events
        .into_iter()
        .map(|e| TimelineEvent { date: e.date, label: e.label })
        .collect();
    events.sort_by(|a, b| normalize_date(&a.date).cmp(&normalize_date(&b.date)));
    Ok(Diagram::Timeline(TimelineDiagram { title: raw.title, events }))
}

/// Normalizes a partial date string to YYYY-MM-DD for sorting/computation.
/// Accepts YYYY, YYYY-MM, or YYYY-MM-DD. Returns None for invalid input.
pub(crate) fn normalize_date(s: &str) -> Option<String> {
    let b = s.as_bytes();
    match b.len() {
        4 if b.iter().all(|c| c.is_ascii_digit()) => Some(format!("{}-01-01", s)),
        7 if b[4] == b'-'
            && b[..4].iter().all(|c| c.is_ascii_digit())
            && b[5..].iter().all(|c| c.is_ascii_digit()) => Some(format!("{}-01", s)),
        10 if b[4] == b'-'
            && b[7] == b'-'
            && b[..4].iter().all(|c| c.is_ascii_digit())
            && b[5..7].iter().all(|c| c.is_ascii_digit())
            && b[8..].iter().all(|c| c.is_ascii_digit()) => Some(s.to_string()),
        _ => None,
    }
}

fn validate_hub_spoke(raw: raw::RawHubSpokeDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "hub_spoke");
    if raw.spokes.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "hub_spoke", min: 2, got: raw.spokes.len() });
    }
    let spokes = parse_items(raw.spokes)?;
    Ok(Diagram::HubSpoke(HubSpokeDiagram {
        title: raw.title,
        center: raw.center,
        spokes,
    }))
}

fn validate_comparison(raw: raw::RawComparisonDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "comparison");
    if raw.rows.is_empty() {
        return Err(DeclartError::InvalidValue {
            field: "rows".to_string(),
            value: "0 rows".to_string(),
            hint: "Comparison tables require at least 1 row".to_string(),
        });
    }
    if raw.columns.is_empty() {
        return Err(DeclartError::InvalidValue {
            field: "columns".to_string(),
            value: "0 columns".to_string(),
            hint: "Comparison tables require at least 1 column".to_string(),
        });
    }
    if raw.rows.len() > 10 {
        return Err(DeclartError::InvalidValue {
            field: "rows".to_string(),
            value: format!("{} rows", raw.rows.len()),
            hint: "Comparison tables support at most 10 rows for legibility".to_string(),
        });
    }
    if raw.columns.len() > 8 {
        return Err(DeclartError::InvalidValue {
            field: "columns".to_string(),
            value: format!("{} columns", raw.columns.len()),
            hint: "Comparison tables support at most 8 columns for legibility".to_string(),
        });
    }

    for col in &raw.columns {
        if col.label == "label" {
            return Err(DeclartError::InvalidValue {
                field: "column label".to_string(),
                value: "label".to_string(),
                hint: "'label' is reserved for the row identifier and cannot be used as a column name".to_string(),
            });
        }
    }

    let col_labels: Vec<&str> = raw.columns.iter().map(|c| c.label.as_str()).collect();
    let col_set: std::collections::HashSet<&str> = col_labels.iter().copied().collect();

    // Validate that no row uses a cell key that isn't a declared column.
    for row in &raw.rows {
        for key in row.cells.keys() {
            if !col_set.contains(key.as_str()) {
                return Err(DeclartError::InvalidValue {
                    field: "row cell key".to_string(),
                    value: key.clone(),
                    hint: format!("'{}' is not a declared column label", key),
                });
            }
        }
    }

    let rows: Vec<String> = raw.rows.iter().map(|r| r.label.clone()).collect();
    let columns: Vec<String> = raw.columns.into_iter().map(|c| c.label).collect();

    // Build cells from inline row data.
    let mut cells = Vec::new();
    for row in &raw.rows {
        for (col_key, value) in &row.cells {
            cells.push(ComparisonCell {
                row: row.label.clone(),
                column: col_key.clone(),
                value: value.clone(),
            });
        }
    }

    Ok(Diagram::Comparison(ComparisonDiagram { title: raw.title, rows, columns, cells }))
}

#[cfg(test)]
mod tests {
    use crate::model::{Diagram, Emphasis, HierarchyView, SequenceView};
    use super::{parse, parse_auto, parse_json};

    // --- sequence tests ---

    #[test]
    fn parse_sequence_defaults_to_process_view() {
        let input = "kind = \"sequence\"\n\n[[items]]\nlabel = \"Plan\"\n\n[[items]]\nlabel = \"Do\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!("expected Sequence") };
        assert_eq!(d.view, SequenceView::Process);
        assert_eq!(d.items.len(), 2);
    }

    #[test]
    fn parse_sequence_explicit_view_cycle() {
        let input = "kind = \"sequence\"\nview = \"cycle\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!() };
        assert_eq!(d.view, SequenceView::Cycle);
    }

    #[test]
    fn parse_sequence_explicit_view_funnel() {
        let input = "kind = \"sequence\"\nview = \"funnel\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!() };
        assert_eq!(d.view, SequenceView::Funnel);
    }

    #[test]
    fn parse_sequence_explicit_view_pyramid() {
        let input = "kind = \"sequence\"\nview = \"pyramid\"\n\n[[items]]\nlabel = \"Top\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!() };
        assert_eq!(d.view, SequenceView::Pyramid);
    }

    #[test]
    fn parse_sequence_rejects_invalid_view() {
        let input = "kind = \"sequence\"\nview = \"unknown\"\n\n[[items]]\nlabel = \"A\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_sequence_cycle_requires_two_items() {
        let input = "kind = \"sequence\"\nview = \"cycle\"\n\n[[items]]\nlabel = \"Only\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_sequence_funnel_requires_two_items() {
        let input = "kind = \"sequence\"\nview = \"funnel\"\n\n[[items]]\nlabel = \"Only\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_rejects_old_sequence_kind_names() {
        for kind in &["process", "cycle", "funnel", "pyramid"] {
            let input = format!("kind = \"{kind}\"\n\n[[items]]\nlabel = \"A\"\n");
            assert!(parse(&input).is_err(), "kind '{kind}' should be rejected");
        }
    }

    // --- hierarchy tests ---

    #[test]
    fn parse_hierarchy_auto_selects_org_chart_for_single_root() {
        let input = "kind = \"hierarchy\"\n\n[[nodes]]\nlabel = \"CEO\"\n\n[[nodes]]\nlabel = \"CTO\"\nparent = \"CEO\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.view, HierarchyView::OrgChart);
        assert_eq!(d.nodes.len(), 2);
    }

    #[test]
    fn parse_hierarchy_auto_selects_fishbone_for_multiple_roots() {
        let input = "kind = \"hierarchy\"\ntitle = \"Issues\"\n\n[[nodes]]\nlabel = \"Materials\"\n\n[[nodes]]\nlabel = \"Methods\"\n\n[[nodes]]\nlabel = \"Bad input\"\nparent = \"Materials\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.view, HierarchyView::Fishbone);
    }

    #[test]
    fn parse_hierarchy_explicit_view_org_chart() {
        let input = "kind = \"hierarchy\"\nview = \"org_chart\"\n\n[[nodes]]\nlabel = \"Root\"\n\n[[nodes]]\nlabel = \"Child\"\nparent = \"Root\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.view, HierarchyView::OrgChart);
    }

    #[test]
    fn parse_hierarchy_explicit_view_fishbone() {
        let input = "kind = \"hierarchy\"\nview = \"fishbone\"\ntitle = \"Effect\"\n\n[[nodes]]\nlabel = \"Cause A\"\n\n[[nodes]]\nlabel = \"Cause B\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.view, HierarchyView::Fishbone);
    }

    #[test]
    fn parse_hierarchy_org_chart_rejects_multiple_roots() {
        let input = "kind = \"hierarchy\"\nview = \"org_chart\"\n\n[[nodes]]\nlabel = \"Root A\"\n\n[[nodes]]\nlabel = \"Root B\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_hierarchy_fishbone_requires_two_root_nodes() {
        let input = "kind = \"hierarchy\"\nview = \"fishbone\"\ntitle = \"Effect\"\n\n[[nodes]]\nlabel = \"Only Cause\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_rejects_old_hierarchy_kind_names() {
        let org = "kind = \"org_chart\"\n\n[[nodes]]\nlabel = \"Root\"\n";
        assert!(parse(org).is_err());
        let fish = "kind = \"fishbone\"\neffect = \"E\"\n\n[[causes]]\nlabel = \"C1\"\n\n[[causes]]\nlabel = \"C2\"\n";
        assert!(parse(fish).is_err());
    }

    // --- smoke tests for unchanged kinds ---

    #[test]
    fn parse_matrix_smoke() {
        let input = "kind = \"matrix\"\nx_axis = \"X\"\ny_axis = \"Y\"\n\n[[quadrants]]\nlabel = \"Q1\"\n\n[[quadrants]]\nlabel = \"Q2\"\n\n[[quadrants]]\nlabel = \"Q3\"\n\n[[quadrants]]\nlabel = \"Q4\"\n";
        assert!(parse(input).is_ok());
    }

    #[test]
    fn parse_timeline_smoke() {
        let input = "kind = \"timeline\"\n\n[[events]]\ndate = \"2024\"\nlabel = \"A\"\n\n[[events]]\ndate = \"2025\"\nlabel = \"B\"\n";
        assert!(parse(input).is_ok());
    }

    #[test]
    fn parse_auto_dispatches_json_sequence() {
        let input = r#"{"kind":"sequence","view":"cycle","items":[{"label":"A"},{"label":"B"}]}"#;
        let diagram = parse_auto(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!() };
        assert_eq!(d.view, SequenceView::Cycle);
    }

    #[test]
    fn parse_json_hierarchy() {
        let input = r#"{"kind":"hierarchy","nodes":[{"label":"CEO"},{"label":"CTO","parent":"CEO"}]}"#;
        let diagram = parse_json(input).unwrap();
        assert!(matches!(diagram, Diagram::Hierarchy(_)));
    }

    // --- existing tests preserved (matrix) ---

    #[test]
    fn parse_matrix_requires_four_quadrants() {
        let input = r#"
kind = "matrix"
x_axis = "Effort"
y_axis = "Impact"

[[quadrants]]
label = "Q1"

[[quadrants]]
label = "Q2"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_matrix_with_position_reorders_quadrants() {
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[quadrants]]
label = "Bottom-Right"
position = "bottom-right"

[[quadrants]]
label = "Top-Left"
position = "top-left"

[[quadrants]]
label = "Top-Right"
position = "top-right"

[[quadrants]]
label = "Bottom-Left"
position = "bottom-left"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Matrix(d) = diagram else { panic!("expected Matrix") };
        assert_eq!(d.quadrants[0].label, "Top-Left");
        assert_eq!(d.quadrants[1].label, "Top-Right");
        assert_eq!(d.quadrants[2].label, "Bottom-Left");
        assert_eq!(d.quadrants[3].label, "Bottom-Right");
    }

    #[test]
    fn parse_matrix_position_rejects_duplicate() {
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[quadrants]]
label = "A"
position = "top-left"

[[quadrants]]
label = "B"
position = "top-left"

[[quadrants]]
label = "C"
position = "top-right"

[[quadrants]]
label = "D"
position = "bottom-left"
"#;
        assert!(parse(input).is_err(), "duplicate position should be rejected");
    }

    #[test]
    fn parse_matrix_position_rejects_missing_on_some() {
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[quadrants]]
label = "A"
position = "top-left"

[[quadrants]]
label = "B"

[[quadrants]]
label = "C"
position = "top-right"

[[quadrants]]
label = "D"
position = "bottom-left"
"#;
        assert!(parse(input).is_err(), "mixed position/no-position should be rejected");
    }

    #[test]
    fn parse_matrix_rejects_items_field() {
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[items]]
label = "Q1"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_sequence_funnel_rejects_more_than_10_items() {
        let mut input = "kind = \"sequence\"\nview = \"funnel\"\n".to_string();
        for i in 0..11 {
            input.push_str(&format!("[[items]]\nlabel = \"Stage {}\"\n", i));
        }
        let err = parse(&input).unwrap_err();
        assert!(err.to_string().contains("items"), "error should mention 'items'");
    }

    #[test]
    fn parse_sequence_funnel_accepts_exactly_10_items() {
        let mut input = "kind = \"sequence\"\nview = \"funnel\"\n".to_string();
        for i in 0..10 {
            input.push_str(&format!("[[items]]\nlabel = \"Stage {}\"\n", i));
        }
        assert!(parse(&input).is_ok(), "10 funnel stages should be accepted");
    }

    // --- JSON parsing ---

    #[test]
    fn parse_json_rejects_unknown_kind() {
        let input = r#"{"kind": "foobar", "items": []}"#;
        assert!(parse_json(input).is_err());
    }

    #[test]
    fn parse_json_valid_sequence_process() {
        let input = r#"{"kind": "sequence", "items": [{"label": "Plan"}, {"label": "Do"}]}"#;
        let diagram = parse_json(input).unwrap();
        let Diagram::Sequence(d) = diagram else { panic!() };
        assert_eq!(d.view, SequenceView::Process);
    }

    #[test]
    fn parse_auto_dispatches_toml_sequence() {
        let input = "kind = \"sequence\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse_auto(input).unwrap();
        assert!(matches!(diagram, Diagram::Sequence(_)));
    }

    #[test]
    fn parse_auto_json_with_leading_whitespace() {
        let input = "  \n{\"kind\":\"sequence\",\"items\":[{\"label\":\"A\"},{\"label\":\"B\"}]}";
        let diagram = parse_auto(input).unwrap();
        assert!(matches!(diagram, Diagram::Sequence(_)));
    }

    // --- Timeline partial dates ---

    #[test]
    fn parse_timeline_accepts_year_only() {
        let input = r#"
kind = "timeline"

[[events]]
date = "2023"
label = "Start"

[[events]]
date = "2024"
label = "End"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Timeline(d) = diagram else { panic!("expected Timeline") };
        assert_eq!(d.events.len(), 2);
        assert_eq!(d.events[0].date, "2023");
    }

    #[test]
    fn parse_timeline_accepts_year_month() {
        let input = r#"
kind = "timeline"

[[events]]
date = "2024-01"
label = "Q1"

[[events]]
date = "2024-06"
label = "Q2"
"#;
        let diagram = parse(input).unwrap();
        assert!(matches!(diagram, Diagram::Timeline(_)));
    }

    #[test]
    fn parse_timeline_mixed_formats_sort_correctly() {
        let input = r#"
kind = "timeline"

[[events]]
date = "2025"
label = "Later"

[[events]]
date = "2023-06"
label = "Middle"

[[events]]
date = "2022-01-01"
label = "First"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Timeline(d) = diagram else { panic!() };
        assert_eq!(d.events[0].date, "2022-01-01");
        assert_eq!(d.events[1].date, "2023-06");
        assert_eq!(d.events[2].date, "2025");
    }

    #[test]
    fn parse_timeline_rejects_invalid_format() {
        let input = r#"
kind = "timeline"

[[events]]
date = "January 2024"
label = "Bad"

[[events]]
date = "2024-01-01"
label = "Good"
"#;
        assert!(parse(input).is_err());
    }

    // --- Comparison table ---

    const VALID_COMPARISON: &str = r#"
kind = "comparison"
title = "Framework Comparison"

[[columns]]
label = "Performance"

[[columns]]
label = "Ecosystem"

[[rows]]
label = "React"
Performance = "★★★★"
Ecosystem = "★★★★★"

[[rows]]
label = "Vue"
Ecosystem = "★★★"
"#;

    #[test]
    fn parse_valid_comparison() {
        let diagram = parse(VALID_COMPARISON).unwrap();
        let Diagram::Comparison(d) = diagram else { panic!("expected Comparison") };
        assert_eq!(d.rows.len(), 2);
        assert_eq!(d.columns.len(), 2);
        assert_eq!(d.cells.len(), 3); // React has 2 cells, Vue has 1
    }

    #[test]
    fn parse_comparison_rejects_unknown_column_key() {
        let input = r#"
kind = "comparison"
[[columns]]
label = "X"
[[rows]]
label = "A"
Y = "val"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_comparison_rejects_empty_rows() {
        let input = r#"
kind = "comparison"
[[columns]]
label = "X"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_comparison_rejects_too_many_columns() {
        let mut input = "kind = \"comparison\"\n[[rows]]\nlabel = \"A\"\n".to_string();
        for i in 0..9 {
            input.push_str(&format!("[[columns]]\nlabel = \"Col{}\"\n", i));
        }
        assert!(parse(&input).is_err());
    }

    #[test]
    fn parse_comparison_empty_cells_allowed() {
        let input = r#"
kind = "comparison"
[[columns]]
label = "Speed"
[[rows]]
label = "A"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Comparison(d) = diagram else { panic!() };
        assert_eq!(d.cells.len(), 0);
    }

    #[test]
    fn parse_json_valid_comparison() {
        let input = r#"{
  "kind": "comparison",
  "columns": [{"label": "Speed"}],
  "rows": [{"label": "React", "Speed": "Fast"}, {"label": "Vue"}]
}"#;
        let diagram = parse_json(input).unwrap();
        let Diagram::Comparison(d) = diagram else { panic!() };
        assert_eq!(d.cells.len(), 1);
        assert_eq!(d.cells[0].value, "Fast");
    }

    #[test]
    fn parse_rejects_unknown_kind() {
        let input = "kind = \"diagram\"\n\n[[items]]\nlabel = \"Item\"\n";
        assert!(parse(input).is_err());
    }
}
