mod raw;

use crate::error::DeclartError;
use crate::model::{
    ComparisonCell, ComparisonDiagram, Diagram, Emphasis, Status,
    FlowDiagram, FlowItem, FlowView,
    TierDiagram, TierView,
    HierarchyDiagram, HierarchyNode, HierarchyView,
    HubSpokeDiagram, Item, MatrixDiagram,
    TimelineDiagram, TimelineEvent,
    VennDiagram, VennIntersection, VennSet,
    StateDiagram, StateNode, StateRole, StateTransition, TransitionKind,
};

pub fn parse(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = toml::from_str(input)?;
    match probe.kind.as_str() {
        "flow" => { let raw: raw::RawFlowDiagram = toml::from_str(input)?; validate_flow(raw) }
        "tier" => { let raw: raw::RawTierDiagram = toml::from_str(input)?; validate_tier(raw) }
        "sequence" => Err(DeclartError::InvalidValue {
            field: "kind".to_string(),
            value: "sequence".to_string(),
            hint: "`sequence` was renamed to `flow` in v0.17 — update your diagram to `kind = \"flow\"`".to_string(),
        }),
        "hierarchy" => { let raw: raw::RawHierarchyDiagram = toml::from_str(input)?; validate_hierarchy(raw) }
        "matrix" => { let raw: raw::RawMatrixDiagram = toml::from_str(input)?; validate_matrix(raw) }
        "hub_spoke" => { let raw: raw::RawHubSpokeDiagram = toml::from_str(input)?; validate_hub_spoke(raw) }
        "venn" => { let raw: raw::RawVennDiagram = toml::from_str(input)?; validate_venn(raw) }
        "timeline" => { let raw: raw::RawTimelineDiagram = toml::from_str(input)?; validate_timeline(raw) }
        "comparison" => { let raw: raw::RawComparisonDiagram = toml::from_str(input)?; validate_comparison(raw) }
        "state" => { let raw: raw::RawStateDiagram = toml::from_str(input)?; validate_state(raw) }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

pub fn parse_json(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = serde_json::from_str(input)?;
    match probe.kind.as_str() {
        "flow" => { let raw: raw::RawFlowDiagram = serde_json::from_str(input)?; validate_flow(raw) }
        "tier" => { let raw: raw::RawTierDiagram = serde_json::from_str(input)?; validate_tier(raw) }
        "sequence" => Err(DeclartError::InvalidValue {
            field: "kind".to_string(),
            value: "sequence".to_string(),
            hint: "`sequence` was renamed to `flow` in v0.17 — update your diagram to `\"kind\": \"flow\"`".to_string(),
        }),
        "hierarchy" => { let raw: raw::RawHierarchyDiagram = serde_json::from_str(input)?; validate_hierarchy(raw) }
        "matrix" => { let raw: raw::RawMatrixDiagram = serde_json::from_str(input)?; validate_matrix(raw) }
        "hub_spoke" => { let raw: raw::RawHubSpokeDiagram = serde_json::from_str(input)?; validate_hub_spoke(raw) }
        "venn" => { let raw: raw::RawVennDiagram = serde_json::from_str(input)?; validate_venn(raw) }
        "timeline" => { let raw: raw::RawTimelineDiagram = serde_json::from_str(input)?; validate_timeline(raw) }
        "comparison" => { let raw: raw::RawComparisonDiagram = serde_json::from_str(input)?; validate_comparison(raw) }
        "state" => { let raw: raw::RawStateDiagram = serde_json::from_str(input)?; validate_state(raw) }
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

fn parse_status(s: Option<&str>) -> Result<Option<Status>, DeclartError> {
    match s {
        None => Ok(None),
        Some("success") => Ok(Some(Status::Success)),
        Some("normal") => Ok(Some(Status::Normal)),
        Some("warning") => Ok(Some(Status::Warning)),
        Some("critical") => Ok(Some(Status::Critical)),
        Some(other) => Err(DeclartError::InvalidValue {
            field: "status".to_string(),
            value: other.to_string(),
            hint: "Valid values are: success, normal, warning, critical".to_string(),
        }),
    }
}

fn parse_items(raw_items: Vec<raw::RawItem>) -> Result<Vec<Item>, DeclartError> {
    raw_items.into_iter().map(|item| {
        let emphasis = parse_emphasis(item.emphasis.as_deref())?;
        let status = parse_status(item.status.as_deref())?;
        Ok(Item { label: item.label, emphasis, status })
    }).collect()
}

fn parse_flow_items(raw_items: Vec<raw::RawFlowItem>) -> Result<Vec<FlowItem>, DeclartError> {
    raw_items.into_iter().map(|item| {
        let emphasis = parse_emphasis(item.emphasis.as_deref())?;
        let status = parse_status(item.status.as_deref())?;
        Ok(FlowItem { label: item.label, emphasis, status, actor: item.actor })
    }).collect()
}

fn validate_flow(raw: raw::RawFlowDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "flow");
    if raw.items.is_empty() { return Err(DeclartError::EmptyItems); }
    let view = match raw.view.as_deref() {
        None | Some("process") => FlowView::Process,
        Some("cycle")     => FlowView::Cycle,
        Some("funnel")    => FlowView::Funnel,
        Some("swimlane")  => FlowView::Swimlane,
        Some("pyramid") => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: "pyramid".to_string(),
            hint: "pyramid is now a separate kind: use `kind = \"tier\"` with `view = \"pyramid\"` (or omit `view`)".to_string(),
        }),
        Some(other) => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: other.to_string(),
            hint: "Valid view values for flow: process, cycle, funnel, swimlane".to_string(),
        }),
    };
    let n = raw.items.len();
    match view {
        FlowView::Cycle if n < 2 =>
            return Err(DeclartError::TooFewItems { kind: "flow/cycle", min: 2, got: n }),
        FlowView::Funnel if n < 2 =>
            return Err(DeclartError::TooFewItems { kind: "flow/funnel", min: 2, got: n }),
        FlowView::Funnel if n > 10 =>
            return Err(DeclartError::InvalidValue {
                field: "items".to_string(),
                value: format!("{n} items"),
                hint: "Funnel view supports at most 10 stages.".to_string(),
            }),
        FlowView::Swimlane => {
            // Every item must have a non-empty actor
            for item in &raw.items {
                if item.actor.as_ref().map(|a| a.is_empty()).unwrap_or(true) {
                    return Err(DeclartError::InvalidValue {
                        field: "actor".to_string(),
                        value: "(missing)".to_string(),
                        hint: "Every item must have a non-empty `actor` field when view = \"swimlane\"".to_string(),
                    });
                }
            }
            // Must have at least 2 distinct actors
            let distinct: std::collections::HashSet<&str> =
                raw.items.iter().map(|i| i.actor.as_deref().unwrap_or("")).collect();
            if distinct.len() < 2 {
                return Err(DeclartError::InvalidValue {
                    field: "actor".to_string(),
                    value: format!("{} distinct actor(s)", distinct.len()),
                    hint: "Swimlane view requires at least 2 distinct actor values".to_string(),
                });
            }
        }
        _ => {}
    }
    let items = parse_flow_items(raw.items)?;
    Ok(Diagram::Flow(FlowDiagram { title: raw.title, view, items }))
}

fn validate_tier(raw: raw::RawTierDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "tier");
    if raw.items.is_empty() { return Err(DeclartError::EmptyItems); }
    let view = match raw.view.as_deref() {
        None | Some("pyramid") => TierView::Pyramid,
        Some("concentric") => TierView::Concentric,
        Some(other) => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: other.to_string(),
            hint: "Valid view values for tier: pyramid, concentric".to_string(),
        }),
    };
    let items = parse_items(raw.items)?;
    Ok(Diagram::Tier(TierDiagram { title: raw.title, view, items }))
}

fn validate_hierarchy(raw: raw::RawHierarchyDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "hierarchy");
    if raw.nodes.is_empty() { return Err(DeclartError::EmptyItems); }

    // Build identifier sets. Each node is identified by its id (if set) or label.
    let labels: std::collections::HashSet<&str> = raw.nodes.iter().map(|n| n.label.as_str()).collect();
    if labels.len() != raw.nodes.len() {
        return Err(DeclartError::InvalidValue {
            field: "label".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each node label must be unique within the diagram".to_string(),
        });
    }

    let ids: std::collections::HashSet<&str> = raw.nodes.iter()
        .filter_map(|n| n.id.as_deref())
        .collect();

    // Check for duplicate ids
    let id_count = raw.nodes.iter().filter(|n| n.id.is_some()).count();
    if ids.len() != id_count {
        return Err(DeclartError::InvalidValue {
            field: "id".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each node id must be unique within the diagram".to_string(),
        });
    }

    // Validate parent references: parent can match an id or a label
    for node in &raw.nodes {
        let node_key = node.id.as_deref().unwrap_or(&node.label);
        if let Some(parent) = &node.parent {
            if !ids.contains(parent.as_str()) && !labels.contains(parent.as_str()) {
                return Err(DeclartError::InvalidValue {
                    field: "parent".to_string(),
                    value: parent.clone(),
                    hint: "parent must reference an existing node id or label".to_string(),
                });
            }
            if node_key == parent.as_str() || node.label == *parent {
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
        Some("mind_map") => HierarchyView::MindMap,
        Some(other) => return Err(DeclartError::InvalidValue {
            field: "view".to_string(),
            value: other.to_string(),
            hint: "Valid view values for hierarchy: org_chart, fishbone, mind_map".to_string(),
        }),
    };
    match view {
        HierarchyView::OrgChart | HierarchyView::MindMap if root_count != 1 =>
            return Err(DeclartError::InvalidValue {
                field: "parent".to_string(),
                value: format!("{root_count} root nodes"),
                hint: "org_chart and mind_map views require exactly one root node".to_string(),
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
    let nodes = raw.nodes.into_iter().map(|n| HierarchyNode { label: n.label, id: n.id, parent: n.parent }).collect();
    Ok(Diagram::Hierarchy(HierarchyDiagram { title: raw.title, effect: raw.effect, view, nodes }))
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
            slots[slot] = Some(Item {
                label: q.label,
                emphasis: parse_emphasis(q.emphasis.as_deref())?,
                status: parse_status(q.status.as_deref())?,
            });
        }
        slots.into_iter().map(|s| s.unwrap()).collect::<Vec<Item>>()
    } else {
        // index order (backward compatible)
        raw.quadrants
            .into_iter()
            .map(|q| -> Result<Item, DeclartError> {
                Ok(Item {
                    label: q.label,
                    emphasis: parse_emphasis(q.emphasis.as_deref())?,
                    status: parse_status(q.status.as_deref())?,
                })
            })
            .collect::<Result<Vec<Item>, DeclartError>>()?
    };

    // Parse optional items classified into quadrants.
    const MAX_ITEMS_PER_QUADRANT: usize = 6;
    let mut per_quadrant = [0usize; 4];
    let mut items = Vec::with_capacity(raw.items.len());
    for raw_item in raw.items {
        let slot = position_slot(&raw_item.quadrant)?;
        per_quadrant[slot] += 1;
        if per_quadrant[slot] > MAX_ITEMS_PER_QUADRANT {
            return Err(DeclartError::InvalidValue {
                field: "items".to_string(),
                value: format!("{} items in one quadrant", per_quadrant[slot]),
                hint: "A matrix quadrant supports at most 6 items for legibility".to_string(),
            });
        }
        items.push(crate::model::MatrixItem {
            label: raw_item.label,
            emphasis: parse_emphasis(raw_item.emphasis.as_deref())?,
            status: parse_status(raw_item.status.as_deref())?,
            quadrant: slot,
        });
    }

    Ok(Diagram::Matrix(MatrixDiagram {
        title: raw.title,
        x_axis: raw.x_axis,
        y_axis: raw.y_axis,
        quadrants,
        items,
    }))
}

/// Maps a quadrant position name to its slot index 0–3 ([TL, TR, BL, BR]).
fn position_slot(pos: &str) -> Result<usize, DeclartError> {
    match pos {
        "top-left" => Ok(0),
        "top-right" => Ok(1),
        "bottom-left" => Ok(2),
        "bottom-right" => Ok(3),
        other => Err(DeclartError::InvalidValue {
            field: "quadrant".to_string(),
            value: other.to_string(),
            hint: "Valid values: top-left, top-right, bottom-left, bottom-right".to_string(),
        }),
    }
}

fn validate_venn(raw: raw::RawVennDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "venn");
    let n = raw.sets.len();
    if !(2..=3).contains(&n) {
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
    events.sort_by_key(|a| normalize_date(&a.date));
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

fn validate_state(raw: raw::RawStateDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "state");

    if raw.states.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "state", min: 2, got: raw.states.len() });
    }

    // Duplicate label check
    let labels: std::collections::HashSet<&str> =
        raw.states.iter().map(|s| s.label.as_str()).collect();
    if labels.len() != raw.states.len() {
        return Err(DeclartError::InvalidValue {
            field: "label".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each state label must be unique; use `id` to disambiguate if labels collide".to_string(),
        });
    }

    // Duplicate id check
    let ids: Vec<&str> = raw.states.iter().filter_map(|s| s.id.as_deref()).collect();
    let id_set: std::collections::HashSet<&str> = ids.iter().copied().collect();
    if id_set.len() != ids.len() {
        return Err(DeclartError::InvalidValue {
            field: "id".to_string(),
            value: "(duplicate)".to_string(),
            hint: "Each state id must be unique within the diagram".to_string(),
        });
    }

    // Parse roles and enforce at-most-one initial
    let mut initial_count = 0_usize;
    let mut states = Vec::with_capacity(raw.states.len());
    for s in raw.states {
        let role = match s.role.as_deref() {
            None => None,
            Some("initial") => { initial_count += 1; Some(StateRole::Initial) }
            Some("terminal") => Some(StateRole::Terminal),
            Some(other) => return Err(DeclartError::InvalidValue {
                field: "role".to_string(),
                value: other.to_string(),
                hint: "Valid values: initial, terminal".to_string(),
            }),
        };
        states.push(StateNode { label: s.label, id: s.id, role });
    }
    if initial_count > 1 {
        return Err(DeclartError::InvalidValue {
            field: "role".to_string(),
            value: format!("{initial_count} initial states"),
            hint: "At most one state may have role = \"initial\"".to_string(),
        });
    }

    // Build reachable key set (id OR label for each state)
    let reachable: std::collections::HashSet<&str> = states.iter().flat_map(|s| {
        let mut keys = vec![s.label.as_str()];
        if let Some(id) = s.id.as_deref() { keys.push(id); }
        keys
    }).collect();

    // Parse transitions
    let mut transitions = Vec::with_capacity(raw.transitions.len());
    for t in raw.transitions {
        if !reachable.contains(t.from.as_str()) {
            return Err(DeclartError::InvalidValue {
                field: "from".to_string(),
                value: t.from.clone(),
                hint: "from must reference an existing state id or label".to_string(),
            });
        }
        if !reachable.contains(t.to.as_str()) {
            return Err(DeclartError::InvalidValue {
                field: "to".to_string(),
                value: t.to.clone(),
                hint: "to must reference an existing state id or label".to_string(),
            });
        }
        let kind = match t.kind.as_deref() {
            None | Some("normal") => TransitionKind::Normal,
            Some("exception") => TransitionKind::Exception,
            Some(other) => return Err(DeclartError::InvalidValue {
                field: "type".to_string(),
                value: other.to_string(),
                hint: "Valid values: normal, exception".to_string(),
            }),
        };
        transitions.push(StateTransition {
            from: t.from, to: t.to, trigger: t.trigger, kind,
        });
    }

    Ok(Diagram::State(StateDiagram { title: raw.title, states, transitions }))
}

#[cfg(test)]
mod tests {
    use crate::model::{Diagram, FlowView, TierView, HierarchyView, StateRole};
    use super::{parse, parse_auto, parse_json};

    // --- flow tests ---

    #[test]
    fn parse_flow_defaults_to_process_view() {
        let input = "kind = \"flow\"\n\n[[items]]\nlabel = \"Plan\"\n\n[[items]]\nlabel = \"Do\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!("expected Flow") };
        assert_eq!(d.view, FlowView::Process);
        assert_eq!(d.items.len(), 2);
    }

    #[test]
    fn parse_flow_explicit_view_cycle() {
        let input = "kind = \"flow\"\nview = \"cycle\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!() };
        assert_eq!(d.view, FlowView::Cycle);
    }

    #[test]
    fn parse_flow_explicit_view_funnel() {
        let input = "kind = \"flow\"\nview = \"funnel\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!() };
        assert_eq!(d.view, FlowView::Funnel);
    }

    #[test]
    fn parse_flow_pyramid_view_gives_migration_hint() {
        let input = "kind = \"flow\"\nview = \"pyramid\"\n\n[[items]]\nlabel = \"Top\"\n";
        let err = parse(input).unwrap_err();
        assert!(err.to_string().contains("tier"), "error should mention tier kind");
    }

    #[test]
    fn parse_tier_defaults_to_pyramid_view() {
        let input = "kind = \"tier\"\n\n[[items]]\nlabel = \"Top\"\n\n[[items]]\nlabel = \"Bottom\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Tier(d) = diagram else { panic!("expected Tier") };
        assert_eq!(d.view, TierView::Pyramid);
        assert_eq!(d.items.len(), 2);
    }

    #[test]
    fn parse_tier_explicit_view_pyramid() {
        let input = "kind = \"tier\"\nview = \"pyramid\"\n\n[[items]]\nlabel = \"A\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Tier(d) = diagram else { panic!() };
        assert_eq!(d.view, TierView::Pyramid);
    }

    #[test]
    fn parse_tier_explicit_view_concentric() {
        let input = "kind = \"tier\"\nview = \"concentric\"\n\n[[items]]\nlabel = \"Core\"\n\n[[items]]\nlabel = \"Outer\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Tier(d) = diagram else { panic!("expected Tier") };
        assert_eq!(d.view, TierView::Concentric);
        assert_eq!(d.items.len(), 2);
    }

    #[test]
    fn parse_tier_rejects_invalid_view() {
        let input = "kind = \"tier\"\nview = \"cycle\"\n\n[[items]]\nlabel = \"A\"\n";
        let err = parse(input).unwrap_err();
        // Hint should mention both supported views so the migration path is clear.
        assert!(format!("{err}").contains("concentric"), "invalid-view hint should list concentric");
    }

    #[test]
    fn parse_flow_rejects_invalid_view() {
        let input = "kind = \"flow\"\nview = \"unknown\"\n\n[[items]]\nlabel = \"A\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_flow_cycle_requires_two_items() {
        let input = "kind = \"flow\"\nview = \"cycle\"\n\n[[items]]\nlabel = \"Only\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_flow_funnel_requires_two_items() {
        let input = "kind = \"flow\"\nview = \"funnel\"\n\n[[items]]\nlabel = \"Only\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_sequence_gives_deprecation_hint() {
        let input = "kind = \"sequence\"\n\n[[items]]\nlabel = \"A\"\n";
        let err = parse(input).unwrap_err();
        assert!(err.to_string().contains("renamed to `flow`"), "error should mention renamed to flow");
    }

    #[test]
    fn parse_rejects_old_flow_kind_names() {
        for kind in &["process", "cycle", "funnel", "pyramid"] {
            let input = format!("kind = \"{kind}\"\n\n[[items]]\nlabel = \"A\"\n");
            assert!(parse(&input).is_err(), "kind '{kind}' should be rejected");
        }
    }

    #[test]
    fn parse_flow_item_status_values() {
        use crate::model::Status;
        let input = r#"
kind = "flow"

[[items]]
label = "Build"
status = "success"

[[items]]
label = "Deploy"
status = "critical"

[[items]]
label = "Review"
status = "warning"

[[items]]
label = "Idle"
status = "normal"

[[items]]
label = "Untracked"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!("expected Flow") };
        assert_eq!(d.items[0].status, Some(Status::Success));
        assert_eq!(d.items[1].status, Some(Status::Critical));
        assert_eq!(d.items[2].status, Some(Status::Warning));
        assert_eq!(d.items[3].status, Some(Status::Normal));
        assert_eq!(d.items[4].status, None);
    }

    #[test]
    fn parse_flow_status_orthogonal_to_emphasis() {
        use crate::model::{Emphasis, Status};
        let input = r#"
kind = "flow"

[[items]]
label = "Critical Path"
emphasis = "primary"
status = "critical"

[[items]]
label = "Other"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!() };
        assert_eq!(d.items[0].emphasis, Some(Emphasis::Primary));
        assert_eq!(d.items[0].status, Some(Status::Critical));
    }

    #[test]
    fn parse_flow_rejects_invalid_status() {
        let input = "kind = \"flow\"\n\n[[items]]\nlabel = \"A\"\nstatus = \"green\"\n\n[[items]]\nlabel = \"B\"\n";
        let err = parse(input).unwrap_err();
        assert!(err.to_string().contains("status"), "error should mention status");
    }

    #[test]
    fn parse_tier_item_status() {
        use crate::model::Status;
        let input = "kind = \"tier\"\n\n[[items]]\nlabel = \"Top\"\nstatus = \"critical\"\n\n[[items]]\nlabel = \"Base\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Tier(d) = diagram else { panic!("expected Tier") };
        assert_eq!(d.items[0].status, Some(Status::Critical));
        assert_eq!(d.items[1].status, None);
    }

    #[test]
    fn parse_hub_spoke_spoke_status() {
        use crate::model::Status;
        let input = "kind = \"hub_spoke\"\ncenter = \"Core\"\n\n[[spokes]]\nlabel = \"Auth\"\nstatus = \"warning\"\n\n[[spokes]]\nlabel = \"DB\"\nstatus = \"success\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::HubSpoke(d) = diagram else { panic!("expected HubSpoke") };
        assert_eq!(d.spokes[0].status, Some(Status::Warning));
        assert_eq!(d.spokes[1].status, Some(Status::Success));
    }

    #[test]
    fn parse_matrix_quadrant_status() {
        use crate::model::Status;
        let input = r#"
kind = "matrix"
x_axis = "Likelihood"
y_axis = "Impact"

[[quadrants]]
label = "Critical Risk"
position = "top-right"
status = "critical"

[[quadrants]]
label = "Monitor"
position = "top-left"

[[quadrants]]
label = "Low"
position = "bottom-left"
status = "success"

[[quadrants]]
label = "Watch"
position = "bottom-right"
status = "warning"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Matrix(d) = diagram else { panic!("expected Matrix") };
        // position reorders to [top-left, top-right, bottom-left, bottom-right]
        assert_eq!(d.quadrants[0].status, None);                    // Monitor (top-left)
        assert_eq!(d.quadrants[1].status, Some(Status::Critical));  // Critical Risk (top-right)
        assert_eq!(d.quadrants[2].status, Some(Status::Success));   // Low (bottom-left)
        assert_eq!(d.quadrants[3].status, Some(Status::Warning));   // Watch (bottom-right)
    }

    #[test]
    fn parse_tier_rejects_invalid_status() {
        let input = "kind = \"tier\"\n\n[[items]]\nlabel = \"Top\"\nstatus = \"bogus\"\n";
        assert!(parse(input).is_err(), "invalid status value should be rejected");
    }

    #[test]
    fn parse_flow_swimlane_basic() {
        let input = r#"
kind = "flow"
view = "swimlane"

[[items]]
actor = "A"
label = "Step 1"

[[items]]
actor = "B"
label = "Step 2"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!("expected Flow") };
        assert_eq!(d.view, FlowView::Swimlane);
        assert_eq!(d.items[0].actor.as_deref(), Some("A"));
        assert_eq!(d.items[1].actor.as_deref(), Some("B"));
    }

    #[test]
    fn parse_flow_swimlane_rejects_missing_actor() {
        let input = r#"
kind = "flow"
view = "swimlane"
[[items]]
actor = "A"
label = "First"
[[items]]
label = "No actor"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_flow_swimlane_rejects_single_actor() {
        let input = r#"
kind = "flow"
view = "swimlane"
[[items]]
actor = "A"
label = "Step 1"
[[items]]
actor = "A"
label = "Step 2"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_flow_swimlane_hint_in_valid_view_error() {
        let input = "kind = \"flow\"\nview = \"unknown\"\n\n[[items]]\nlabel = \"A\"\n";
        let err = parse(input).unwrap_err();
        assert!(err.to_string().contains("swimlane"), "error hint should list swimlane");
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

    #[test]
    fn parse_hierarchy_node_id_allows_parent_by_id() {
        let input = r#"
kind = "hierarchy"

[[nodes]]
id = "ceo"
label = "Chief Executive Officer"

[[nodes]]
label = "CTO"
parent = "ceo"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.nodes.len(), 2);
        assert_eq!(d.nodes[0].id.as_deref(), Some("ceo"));
        assert_eq!(d.nodes[1].parent.as_deref(), Some("ceo"));
    }

    #[test]
    fn parse_hierarchy_rejects_duplicate_id() {
        let input = r#"
kind = "hierarchy"

[[nodes]]
id = "dup"
label = "Node A"

[[nodes]]
id = "dup"
label = "Node B"
"#;
        assert!(parse(input).is_err(), "duplicate id should be rejected");
    }

    #[test]
    fn parse_hierarchy_label_parent_still_works_without_id() {
        let input = "kind = \"hierarchy\"\n\n[[nodes]]\nlabel = \"CEO\"\n\n[[nodes]]\nlabel = \"CTO\"\nparent = \"CEO\"\n";
        assert!(parse(input).is_ok(), "existing label-based parent reference must still work");
    }

    #[test]
    fn parse_hierarchy_effect_field_separate_from_title() {
        let input = r#"
kind = "hierarchy"
view = "fishbone"
title = "API Performance"
effect = "Slow API Response"

[[nodes]]
label = "Materials"

[[nodes]]
label = "Methods"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert_eq!(d.title.as_deref(), Some("API Performance"));
        assert_eq!(d.effect.as_deref(), Some("Slow API Response"));
    }

    #[test]
    fn parse_hierarchy_effect_defaults_to_none() {
        let input = "kind = \"hierarchy\"\nview = \"fishbone\"\ntitle = \"Issues\"\n\n[[nodes]]\nlabel = \"Materials\"\n\n[[nodes]]\nlabel = \"Methods\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Hierarchy(d) = diagram else { panic!() };
        assert!(d.effect.is_none());
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
    fn parse_auto_dispatches_json_flow() {
        let input = r#"{"kind":"flow","view":"cycle","items":[{"label":"A"},{"label":"B"}]}"#;
        let diagram = parse_auto(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!() };
        assert_eq!(d.view, FlowView::Cycle);
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
    fn parse_matrix_items_placed_into_quadrants() {
        use crate::model::Status;
        let input = r#"
kind = "matrix"
x_axis = "Market Share"
y_axis = "Growth"

[[quadrants]]
label = "Stars"
position = "top-right"

[[quadrants]]
label = "Question Marks"
position = "top-left"

[[quadrants]]
label = "Cash Cows"
position = "bottom-right"

[[quadrants]]
label = "Dogs"
position = "bottom-left"

[[items]]
label = "Product A"
quadrant = "top-right"
status = "success"

[[items]]
label = "Product B"
quadrant = "top-right"
emphasis = "primary"

[[items]]
label = "Product C"
quadrant = "bottom-left"
status = "critical"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::Matrix(d) = diagram else { panic!("expected Matrix") };
        assert_eq!(d.items.len(), 3);
        // top-right = slot 1, bottom-left = slot 2
        assert_eq!(d.items[0].quadrant, 1);
        assert_eq!(d.items[0].status, Some(Status::Success));
        assert_eq!(d.items[2].quadrant, 2);
        assert_eq!(d.items[2].status, Some(Status::Critical));
    }

    #[test]
    fn parse_matrix_item_rejects_bad_quadrant() {
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[quadrants]]
label = "A"
[[quadrants]]
label = "B"
[[quadrants]]
label = "C"
[[quadrants]]
label = "D"

[[items]]
label = "X"
quadrant = "middle"
"#;
        assert!(parse(input).is_err(), "invalid quadrant position should be rejected");
    }

    #[test]
    fn parse_matrix_item_requires_quadrant_field() {
        // An item without `quadrant` is rejected (deny_unknown_fields + required field).
        let input = r#"
kind = "matrix"
x_axis = "X"
y_axis = "Y"

[[quadrants]]
label = "A"
[[quadrants]]
label = "B"
[[quadrants]]
label = "C"
[[quadrants]]
label = "D"

[[items]]
label = "orphan"
"#;
        assert!(parse(input).is_err(), "matrix item must declare a quadrant");
    }

    #[test]
    fn parse_matrix_rejects_too_many_items_per_quadrant() {
        let mut input = String::from("kind = \"matrix\"\nx_axis = \"X\"\ny_axis = \"Y\"\n");
        for label in ["A", "B", "C", "D"] {
            input.push_str(&format!("[[quadrants]]\nlabel = \"{label}\"\n"));
        }
        for i in 0..7 {
            input.push_str(&format!("[[items]]\nlabel = \"I{i}\"\nquadrant = \"top-left\"\n"));
        }
        assert!(parse(&input).is_err(), "more than 6 items in one quadrant should be rejected");
    }

    #[test]
    fn parse_matrix_without_items_still_works() {
        // Backward compatibility: label-only quadrants, no items.
        let input = "kind = \"matrix\"\nx_axis = \"X\"\ny_axis = \"Y\"\n\n[[quadrants]]\nlabel = \"Q1\"\n\n[[quadrants]]\nlabel = \"Q2\"\n\n[[quadrants]]\nlabel = \"Q3\"\n\n[[quadrants]]\nlabel = \"Q4\"\n";
        let diagram = parse(input).unwrap();
        let Diagram::Matrix(d) = diagram else { panic!() };
        assert!(d.items.is_empty());
    }

    #[test]
    fn parse_flow_funnel_rejects_more_than_10_items() {
        let mut input = "kind = \"flow\"\nview = \"funnel\"\n".to_string();
        for i in 0..11 {
            input.push_str(&format!("[[items]]\nlabel = \"Stage {}\"\n", i));
        }
        let err = parse(&input).unwrap_err();
        assert!(err.to_string().contains("items"), "error should mention 'items'");
    }

    #[test]
    fn parse_flow_funnel_accepts_exactly_10_items() {
        let mut input = "kind = \"flow\"\nview = \"funnel\"\n".to_string();
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
    fn parse_json_valid_flow_process() {
        let input = r#"{"kind": "flow", "items": [{"label": "Plan"}, {"label": "Do"}]}"#;
        let diagram = parse_json(input).unwrap();
        let Diagram::Flow(d) = diagram else { panic!() };
        assert_eq!(d.view, FlowView::Process);
    }

    #[test]
    fn parse_json_sequence_gives_deprecation_hint() {
        let input = r#"{"kind": "sequence", "items": [{"label": "A"}]}"#;
        let err = parse_json(input).unwrap_err();
        assert!(err.to_string().contains("renamed to `flow`"));
    }

    #[test]
    fn parse_auto_dispatches_toml_flow() {
        let input = "kind = \"flow\"\n\n[[items]]\nlabel = \"A\"\n\n[[items]]\nlabel = \"B\"\n";
        let diagram = parse_auto(input).unwrap();
        assert!(matches!(diagram, Diagram::Flow(_)));
    }

    #[test]
    fn parse_auto_json_with_leading_whitespace() {
        let input = "  \n{\"kind\":\"flow\",\"items\":[{\"label\":\"A\"},{\"label\":\"B\"}]}";
        let diagram = parse_auto(input).unwrap();
        assert!(matches!(diagram, Diagram::Flow(_)));
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

    // --- state tests ---

    #[test]
    fn parse_state_basic() {
        let input = r#"
kind = "state"

[[states]]
id = "idle"
label = "Idle"
role = "initial"

[[states]]
id = "done"
label = "Done"
role = "terminal"

[[transitions]]
from = "idle"
to = "done"
trigger = "finish"
"#;
        let diagram = parse(input).unwrap();
        let Diagram::State(d) = diagram else { panic!("expected State") };
        assert_eq!(d.states.len(), 2);
        assert_eq!(d.transitions[0].trigger.as_deref(), Some("finish"));
        assert!(matches!(d.states[0].role, Some(StateRole::Initial)));
    }

    #[test]
    fn parse_state_rejects_single_state() {
        let input = "kind = \"state\"\n\n[[states]]\nlabel = \"Only\"\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_state_rejects_multiple_initial() {
        let input = r#"
kind = "state"
[[states]]
label = "A"
role = "initial"
[[states]]
label = "B"
role = "initial"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_state_rejects_unknown_transition_ref() {
        let input = r#"
kind = "state"
[[states]]
label = "A"
[[states]]
label = "B"
[[transitions]]
from = "A"
to = "NONEXISTENT"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_state_allows_multiple_terminal() {
        let input = r#"
kind = "state"
[[states]]
label = "Done"
role = "terminal"
[[states]]
label = "Cancelled"
role = "terminal"
"#;
        assert!(parse(input).is_ok());
    }

    #[test]
    fn parse_state_self_loop_valid() {
        let input = r#"
kind = "state"
[[states]]
id = "s1"
label = "Retrying"
[[states]]
label = "Done"
[[transitions]]
from = "s1"
to = "s1"
trigger = "retry"
[[transitions]]
from = "s1"
to = "Done"
trigger = "success"
"#;
        assert!(parse(input).is_ok());
    }

    #[test]
    fn parse_state_no_transitions_valid() {
        let input = r#"
kind = "state"
[[states]]
label = "Pending"
[[states]]
label = "Active"
"#;
        assert!(parse(input).is_ok());
    }

    #[test]
    fn parse_state_rejects_duplicate_label() {
        let input = r#"
kind = "state"
[[states]]
label = "Ready"
[[states]]
label = "Ready"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn unknown_kind_hint_includes_state() {
        let err = parse("kind = \"unknown\"\n").unwrap_err();
        assert!(err.to_string().contains("state"), "hint should list 'state'");
    }
}
