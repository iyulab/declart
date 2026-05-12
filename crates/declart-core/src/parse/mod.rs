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
    // label is the unique identifier; validate uniqueness and parent references.
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
    {"label": "CEO"},
    {"label": "CTO", "parent": "CEO"}
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
}
