mod raw;

use crate::error::DeclartError;
use crate::model::{ComparisonCell, ComparisonDiagram, Diagram, Emphasis, FishboneCause, FishboneDiagram, HubSpokeDiagram, Item, ItemsDiagram, MatrixDiagram, OrgChartDiagram, OrgChartNode, TimelineDiagram, TimelineEvent, VennDiagram, VennIntersection, VennSet};

/// Parses a TOML declaration string into a validated [`Diagram`].
///
/// Returns [`DeclartError`] for unknown kinds, forbidden fields, missing required fields,
/// invalid values, or structural violations (e.g. wrong quadrant count).
pub fn parse(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = toml::from_str(input)?;

    match probe.kind.as_str() {
        "pyramid" | "process" | "cycle" | "funnel" => {
            let raw: raw::RawItemsDiagram = toml::from_str(input)?;
            validate_items(raw)
        }
        "matrix" => {
            let raw: raw::RawMatrixDiagram = toml::from_str(input)?;
            validate_matrix(raw)
        }
        "hub_spoke" => {
            let raw: raw::RawHubSpokeDiagram = toml::from_str(input)?;
            validate_hub_spoke(raw)
        }
        "venn" => {
            let raw: raw::RawVennDiagram = toml::from_str(input)?;
            validate_venn(raw)
        }
        "timeline" => {
            let raw: raw::RawTimelineDiagram = toml::from_str(input)?;
            validate_timeline(raw)
        }
        "fishbone" => {
            let raw: raw::RawFishboneDiagram = toml::from_str(input)?;
            validate_fishbone(raw)
        }
        "org_chart" => {
            let raw: raw::RawOrgChartDiagram = toml::from_str(input)?;
            validate_org_chart(raw)
        }
        "comparison" => {
            let raw: raw::RawComparisonDiagram = toml::from_str(input)?;
            validate_comparison(raw)
        }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

/// Parses a JSON declaration string into a validated [`Diagram`].
///
/// The JSON structure mirrors the TOML format: `"kind"` is required, and array fields
/// use JSON arrays (e.g., `"items": [...]` instead of `[[items]]` in TOML).
pub fn parse_json(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = serde_json::from_str(input)?;

    match probe.kind.as_str() {
        "pyramid" | "process" | "cycle" | "funnel" => {
            let raw: raw::RawItemsDiagram = serde_json::from_str(input)?;
            validate_items(raw)
        }
        "matrix" => {
            let raw: raw::RawMatrixDiagram = serde_json::from_str(input)?;
            validate_matrix(raw)
        }
        "hub_spoke" => {
            let raw: raw::RawHubSpokeDiagram = serde_json::from_str(input)?;
            validate_hub_spoke(raw)
        }
        "venn" => {
            let raw: raw::RawVennDiagram = serde_json::from_str(input)?;
            validate_venn(raw)
        }
        "timeline" => {
            let raw: raw::RawTimelineDiagram = serde_json::from_str(input)?;
            validate_timeline(raw)
        }
        "fishbone" => {
            let raw: raw::RawFishboneDiagram = serde_json::from_str(input)?;
            validate_fishbone(raw)
        }
        "org_chart" => {
            let raw: raw::RawOrgChartDiagram = serde_json::from_str(input)?;
            validate_org_chart(raw)
        }
        "comparison" => {
            let raw: raw::RawComparisonDiagram = serde_json::from_str(input)?;
            validate_comparison(raw)
        }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

/// Auto-detects input format and parses into a validated [`Diagram`].
///
/// If the trimmed input starts with `{`, JSON parsing is used; otherwise TOML.
pub fn parse_auto(input: &str) -> Result<Diagram, DeclartError> {
    if input.trim_start().starts_with('{') {
        parse_json(input)
    } else {
        parse(input)
    }
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
    raw_items
        .into_iter()
        .map(|item| {
            let emphasis = parse_emphasis(item.emphasis.as_deref())?;
            Ok(Item { label: item.label, emphasis })
        })
        .collect()
}

fn validate_items(raw: raw::RawItemsDiagram) -> Result<Diagram, DeclartError> {
    if raw.items.is_empty() {
        return Err(DeclartError::EmptyItems);
    }
    let kind_str = raw.kind.as_str();
    let n_items = raw.items.len();
    if kind_str == "cycle" && n_items < 2 {
        return Err(DeclartError::TooFewItems { kind: "cycle", min: 2, got: n_items });
    }
    if kind_str == "funnel" && n_items < 2 {
        return Err(DeclartError::TooFewItems { kind: "funnel", min: 2, got: n_items });
    }
    if kind_str == "funnel" && n_items > 10 {
        return Err(DeclartError::InvalidValue {
            field: "items".to_string(),
            value: format!("{} items", n_items),
            hint: "Funnel diagrams support at most 10 stages. Beyond that, lower stages collapse to the minimum width and lose the funnel shape.".to_string(),
        });
    }
    let items = parse_items(raw.items)?;
    let inner = ItemsDiagram { title: raw.title, items };
    let diagram = match kind_str {
        "pyramid" => Diagram::Pyramid(inner),
        "process" => Diagram::Process(inner),
        "cycle" => Diagram::Cycle(inner),
        "funnel" => Diagram::Funnel(inner),
        _ => unreachable!(),
    };
    Ok(diagram)
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

fn validate_fishbone(raw: raw::RawFishboneDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "fishbone");
    if raw.causes.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "fishbone", min: 2, got: raw.causes.len() });
    }
    if raw.causes.len() > 20 {
        return Err(DeclartError::InvalidValue {
            field: "causes".to_string(),
            value: format!("{} causes", raw.causes.len()),
            hint: "Fishbone diagrams support at most 20 causes. For visual clarity, 8 or fewer is recommended.".to_string(),
        });
    }
    let causes = raw
        .causes
        .into_iter()
        .map(|c| {
            let items = c.items.into_iter().map(|i| Item { label: i.label, emphasis: None }).collect();
            FishboneCause { label: c.label, items }
        })
        .collect();
    Ok(Diagram::Fishbone(FishboneDiagram {
        title: raw.title,
        effect: raw.effect,
        causes,
    }))
}

fn validate_timeline(raw: raw::RawTimelineDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "timeline");
    if raw.events.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "timeline", min: 2, got: raw.events.len() });
    }
    for event in &raw.events {
        if !is_valid_iso_date(&event.date) {
            return Err(DeclartError::InvalidValue {
                field: "date".to_string(),
                value: event.date.clone(),
                hint: "Date must be in ISO 8601 format: YYYY-MM-DD".to_string(),
            });
        }
    }
    let mut events: Vec<TimelineEvent> = raw
        .events
        .into_iter()
        .map(|e| TimelineEvent { date: e.date, label: e.label })
        .collect();
    events.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(Diagram::Timeline(TimelineDiagram { title: raw.title, events }))
}

fn is_valid_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 {
        return false;
    }
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    bytes[..4].iter().all(|b| b.is_ascii_digit())
        && bytes[5..7].iter().all(|b| b.is_ascii_digit())
        && bytes[8..10].iter().all(|b| b.is_ascii_digit())
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

fn validate_org_chart(raw: raw::RawOrgChartDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "org_chart");
    if raw.nodes.is_empty() {
        return Err(DeclartError::EmptyItems);
    }
    // Validate: exactly one root (no parent), all parent references exist, no cycles.
    let ids: std::collections::HashSet<&str> = raw.nodes.iter().map(|n| n.id.as_str()).collect();
    if ids.len() != raw.nodes.len() {
        return Err(DeclartError::InvalidValue {
            field: "id".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each node id must be unique within the diagram".to_string(),
        });
    }
    for node in &raw.nodes {
        if let Some(parent) = &node.parent {
            if !ids.contains(parent.as_str()) {
                return Err(DeclartError::InvalidValue {
                    field: "parent".to_string(),
                    value: parent.clone(),
                    hint: "parent must reference an existing node id".to_string(),
                });
            }
            if node.id == *parent {
                return Err(DeclartError::InvalidValue {
                    field: "parent".to_string(),
                    value: parent.clone(),
                    hint: "A node cannot be its own parent".to_string(),
                });
            }
        }
    }
    let roots: Vec<&raw::RawOrgChartNode> = raw.nodes.iter().filter(|n| n.parent.is_none()).collect();
    if roots.is_empty() {
        return Err(DeclartError::InvalidValue {
            field: "parent".to_string(),
            value: "(all nodes have parents)".to_string(),
            hint: "Exactly one root node (with no parent) is required".to_string(),
        });
    }
    if roots.len() > 1 {
        return Err(DeclartError::InvalidValue {
            field: "parent".to_string(),
            value: format!("{} root nodes", roots.len()),
            hint: "Exactly one root node (with no parent) is required".to_string(),
        });
    }
    let nodes = raw.nodes.into_iter().map(|n| OrgChartNode {
        id: n.id,
        label: n.label,
        parent: n.parent,
    }).collect();
    Ok(Diagram::OrgChart(OrgChartDiagram { title: raw.title, nodes }))
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

    let row_labels: std::collections::HashSet<&str> =
        raw.rows.iter().map(|r| r.label.as_str()).collect();
    let col_labels: std::collections::HashSet<&str> =
        raw.columns.iter().map(|c| c.label.as_str()).collect();

    for cell in &raw.cells {
        if !row_labels.contains(cell.row.as_str()) {
            return Err(DeclartError::InvalidValue {
                field: "cell.row".to_string(),
                value: cell.row.clone(),
                hint: "cell.row must reference an existing row label".to_string(),
            });
        }
        if !col_labels.contains(cell.column.as_str()) {
            return Err(DeclartError::InvalidValue {
                field: "cell.column".to_string(),
                value: cell.column.clone(),
                hint: "cell.column must reference an existing column label".to_string(),
            });
        }
    }

    let rows = raw.rows.into_iter().map(|r| r.label).collect();
    let columns = raw.columns.into_iter().map(|c| c.label).collect();
    let cells = raw.cells.into_iter().map(|c| ComparisonCell {
        row: c.row,
        column: c.column,
        value: c.value,
    }).collect();

    Ok(Diagram::Comparison(ComparisonDiagram { title: raw.title, rows, columns, cells }))
}

#[cfg(test)]
mod tests {
    use crate::model::{Diagram, Emphasis};
    use super::{parse, parse_auto, parse_json};

    const VALID_PYRAMID: &str = r#"
kind = "pyramid"
title = "Test Pyramid"

[[items]]
label = "Top"

[[items]]
label = "Bottom"
emphasis = "primary"
"#;

    #[test]
    fn parse_valid_pyramid() {
        let diagram = parse(VALID_PYRAMID).unwrap();
        let Diagram::Pyramid(d) = diagram else { panic!("expected Pyramid") };
        assert_eq!(d.title, Some("Test Pyramid".to_string()));
        assert_eq!(d.items.len(), 2);
        assert_eq!(d.items[0].label, "Top");
        assert_eq!(d.items[1].emphasis, Some(Emphasis::Primary));
    }

    #[test]
    fn parse_rejects_forbidden_field() {
        let input = "kind = \"pyramid\"\n\n[[items]]\nlabel = \"Top\"\ncolor = \"blue\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_rejects_unknown_kind() {
        let input = "kind = \"diagram\"\n\n[[items]]\nlabel = \"Item\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_rejects_empty_items() {
        let input = "kind = \"pyramid\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_title_is_optional() {
        let input = "kind = \"pyramid\"\n\n[[items]]\nlabel = \"Only\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Pyramid(d) = diagram else { panic!() };
        assert!(d.title.is_none());
    }

    #[test]
    fn parse_valid_process() {
        let input = "kind = \"process\"\n\n[[items]]\nlabel = \"Step 1\"\n\n[[items]]\nlabel = \"Step 2\"\n";
        let diagram = parse(input).unwrap();
        assert!(matches!(diagram, Diagram::Process(_)));
    }

    #[test]
    fn parse_valid_cycle() {
        let input = "kind = \"cycle\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse(input).unwrap();
        assert!(matches!(diagram, Diagram::Cycle(_)));
    }

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
        // Declared in reverse order: bottom-right first, then top-left, top-right, bottom-left
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
        // Model stores in canonical order: top-left, top-right, bottom-left, bottom-right
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
    fn parse_fishbone_rejects_too_many_causes() {
        let mut input = "kind = \"fishbone\"\neffect = \"E\"\n".to_string();
        for i in 0..21 {
            input.push_str(&format!("[[causes]]\nlabel = \"C{}\"\n", i));
        }
        let err = parse(&input).unwrap_err();
        assert!(err.to_string().contains("causes"), "error should mention 'causes'");
    }

    #[test]
    fn parse_fishbone_accepts_exactly_20_causes() {
        let mut input = "kind = \"fishbone\"\neffect = \"E\"\n".to_string();
        for i in 0..20 {
            input.push_str(&format!("[[causes]]\nlabel = \"C{}\"\n", i));
        }
        assert!(parse(&input).is_ok(), "20 causes should be accepted");
    }

    #[test]
    fn parse_funnel_rejects_more_than_10_items() {
        let mut input = "kind = \"funnel\"\n".to_string();
        for i in 0..11 {
            input.push_str(&format!("[[items]]\nlabel = \"Stage {}\"\n", i));
        }
        let err = parse(&input).unwrap_err();
        assert!(err.to_string().contains("items"), "error should mention 'items'");
    }

    #[test]
    fn parse_funnel_accepts_exactly_10_items() {
        let mut input = "kind = \"funnel\"\n".to_string();
        for i in 0..10 {
            input.push_str(&format!("[[items]]\nlabel = \"Stage {}\"\n", i));
        }
        assert!(parse(&input).is_ok(), "10 funnel stages should be accepted");
    }

    // --- JSON parsing ---

    const VALID_PYRAMID_JSON: &str = r#"{
  "kind": "pyramid",
  "title": "Maslow",
  "items": [
    { "label": "Self-Actualization" },
    { "label": "Esteem", "emphasis": "primary" }
  ]
}"#;

    #[test]
    fn parse_json_valid_pyramid() {
        let diagram = parse_json(VALID_PYRAMID_JSON).unwrap();
        let Diagram::Pyramid(d) = diagram else { panic!("expected Pyramid") };
        assert_eq!(d.title, Some("Maslow".to_string()));
        assert_eq!(d.items.len(), 2);
        assert_eq!(d.items[0].label, "Self-Actualization");
        assert_eq!(d.items[1].emphasis, Some(Emphasis::Primary));
    }

    #[test]
    fn parse_json_rejects_unknown_kind() {
        let input = r#"{"kind": "foobar", "items": []}"#;
        assert!(parse_json(input).is_err());
    }

    #[test]
    fn parse_json_rejects_forbidden_field() {
        let input = r#"{"kind": "pyramid", "items": [{"label": "A", "color": "red"}]}"#;
        assert!(parse_json(input).is_err());
    }

    #[test]
    fn parse_json_valid_process() {
        let input = r#"{"kind": "process", "items": [{"label": "Plan"}, {"label": "Do"}]}"#;
        let diagram = parse_json(input).unwrap();
        assert!(matches!(diagram, Diagram::Process(_)));
    }

    #[test]
    fn parse_json_valid_org_chart() {
        let input = r#"{
  "kind": "org_chart",
  "nodes": [
    {"id": "ceo", "label": "CEO"},
    {"id": "cto", "label": "CTO", "parent": "ceo"}
  ]
}"#;
        let diagram = parse_json(input).unwrap();
        assert!(matches!(diagram, Diagram::OrgChart(_)));
    }

    #[test]
    fn parse_auto_dispatches_json() {
        let diagram = parse_auto(VALID_PYRAMID_JSON).unwrap();
        assert!(matches!(diagram, Diagram::Pyramid(_)));
    }

    #[test]
    fn parse_auto_dispatches_toml() {
        let diagram = parse_auto(VALID_PYRAMID).unwrap();
        assert!(matches!(diagram, Diagram::Pyramid(_)));
    }

    #[test]
    fn parse_auto_json_with_leading_whitespace() {
        let input = format!("  \n{}", VALID_PYRAMID_JSON);
        let diagram = parse_auto(&input).unwrap();
        assert!(matches!(diagram, Diagram::Pyramid(_)));
    }

    // --- Comparison table ---

    const VALID_COMPARISON: &str = r#"
kind = "comparison"
title = "Framework Comparison"

[[rows]]
label = "React"

[[rows]]
label = "Vue"

[[columns]]
label = "Performance"

[[columns]]
label = "Ecosystem"

[[cells]]
row = "React"
column = "Performance"
value = "★★★★"

[[cells]]
row = "Vue"
column = "Ecosystem"
value = "★★★"
"#;

    #[test]
    fn parse_valid_comparison() {
        let diagram = parse(VALID_COMPARISON).unwrap();
        let Diagram::Comparison(d) = diagram else { panic!("expected Comparison") };
        assert_eq!(d.rows.len(), 2);
        assert_eq!(d.columns.len(), 2);
        assert_eq!(d.cells.len(), 2);
        assert_eq!(d.cells[0].value, "★★★★");
    }

    #[test]
    fn parse_comparison_rejects_invalid_cell_row() {
        let input = r#"
kind = "comparison"
[[rows]]
label = "A"
[[columns]]
label = "X"
[[cells]]
row = "NONEXISTENT"
column = "X"
value = "ok"
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
    fn parse_json_valid_comparison() {
        let input = r#"{
  "kind": "comparison",
  "rows": [{"label": "React"}, {"label": "Vue"}],
  "columns": [{"label": "Speed"}],
  "cells": [{"row": "React", "column": "Speed", "value": "Fast"}]
}"#;
        let diagram = parse_json(input).unwrap();
        assert!(matches!(diagram, Diagram::Comparison(_)));
    }
}
