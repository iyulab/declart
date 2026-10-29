//! Text equivalents of a diagram — the accessible name and description an SVG carries, and
//! the alternative text a page gives a diagram it shows as an image.

use crate::model::{
    Diagram, FlowView, HierarchyDiagram, HierarchyNode, HierarchyView, StateRole, TierView, TimelineWhen,
    TransitionKind,
};

/// A short accessible name for `diagram`: its declared `title`, or its kind and view
/// (`"Process diagram"`) when it has none.
pub fn accessible_name(diagram: &Diagram) -> String {
    if let Some(title) = title(diagram) {
        return title.to_string();
    }
    let kind = match diagram {
        Diagram::Flow(d) => match d.view {
            FlowView::Process => "Process",
            FlowView::Cycle => "Cycle",
            FlowView::Funnel => "Funnel",
            FlowView::Swimlane => "Swimlane",
        },
        Diagram::Tier(d) => match d.view {
            TierView::Pyramid => "Pyramid",
            TierView::Concentric => "Concentric",
        },
        Diagram::Hierarchy(_) => "Hierarchy",
        Diagram::Matrix(_) => "Matrix",
        Diagram::HubSpoke(_) => "Hub and spoke",
        Diagram::Venn(_) => "Venn",
        Diagram::Timeline(_) => "Timeline",
        Diagram::Comparison(_) => "Comparison",
        Diagram::State(_) => "State",
    };
    format!("{kind} diagram")
}

/// A one-paragraph text equivalent of `diagram`'s content, in declaration order — the order
/// and structure a reader of the drawing would take in. Use it as alternative text when the
/// SVG is shown as an image (`<img alt>`); the SVG itself carries it as `<desc>`.
///
/// ```
/// let d = declart_core::parse("kind = \"flow\"\n[[items]]\nlabel = \"Upload\"\n[[items]]\nlabel = \"Index\"").unwrap();
/// assert_eq!(declart_core::describe(&d), "Process: Upload → Index.");
/// ```
pub fn describe(diagram: &Diagram) -> String {
    let body = match diagram {
        Diagram::Flow(d) => {
            let labels: Vec<&str> = d.items.iter().map(|i| i.label.as_str()).collect();
            match d.view {
                FlowView::Process => format!("Process: {}", labels.join(" → ")),
                FlowView::Cycle => {
                    let first = labels.first().copied().unwrap_or("");
                    format!("Cycle: {} → {}", labels.join(" → "), first)
                }
                FlowView::Funnel => format!("Funnel, widest first: {}", labels.join(" → ")),
                FlowView::Swimlane => {
                    let steps: Vec<String> = d
                        .items
                        .iter()
                        .map(|i| match &i.actor {
                            Some(actor) => format!("{} ({})", i.label, actor),
                            None => i.label.clone(),
                        })
                        .collect();
                    format!("Swimlane: {}", steps.join(" → "))
                }
            }
        }
        Diagram::Tier(d) => {
            let labels: Vec<&str> = d.items.iter().map(|i| i.label.as_str()).collect();
            match d.view {
                TierView::Pyramid => format!("Pyramid, top to base: {}", labels.join(", ")),
                TierView::Concentric => {
                    format!("Concentric rings, core outward: {}", labels.join(", "))
                }
            }
        }
        Diagram::Hierarchy(d) => hierarchy(d),
        Diagram::Matrix(d) => {
            let names = ["top left", "top right", "bottom left", "bottom right"];
            let quadrants: Vec<String> = d
                .quadrants
                .iter()
                .enumerate()
                .map(|(qi, q)| {
                    let items: Vec<&str> = d
                        .items
                        .iter()
                        .filter(|it| it.quadrant == qi)
                        .map(|it| it.label.as_str())
                        .collect();
                    let place = names.get(qi).copied().unwrap_or("");
                    if items.is_empty() {
                        format!("{} ({})", q.label, place)
                    } else {
                        format!("{} ({}): {}", q.label, place, items.join(", "))
                    }
                })
                .collect();
            format!(
                "Matrix of {} by {}: {}",
                d.x_axis,
                d.y_axis,
                quadrants.join("; ")
            )
        }
        Diagram::HubSpoke(d) => {
            let spokes: Vec<&str> = d.spokes.iter().map(|s| s.label.as_str()).collect();
            format!("{} at the centre, connected to {}", d.center, spokes.join(", "))
        }
        Diagram::Venn(d) => {
            let sets: Vec<&str> = d.sets.iter().map(|s| s.label.as_str()).collect();
            let mut text = format!("Venn of {}", sets.join(", "));
            for i in &d.intersections {
                text.push_str(&format!("; {} overlap: {}", i.sets.join(" and "), i.label));
            }
            text
        }
        Diagram::Timeline(d) => {
            let events: Vec<String> = d
                .events
                .iter()
                .map(|e| match &e.when {
                    TimelineWhen::Point(date) => format!("{date} {}", e.label),
                    TimelineWhen::Span { start, end } => format!("{start} to {end} {}", e.label),
                })
                .collect();
            format!("Timeline: {}", events.join("; "))
        }
        Diagram::Comparison(d) => {
            let rows: Vec<String> = d
                .rows
                .iter()
                .map(|row| {
                    let cells: Vec<String> = d
                        .cells
                        .iter()
                        .filter(|c| &c.row == row)
                        .map(|c| format!("{} {}", c.column, c.value))
                        .collect();
                    if cells.is_empty() {
                        row.clone()
                    } else {
                        format!("{}: {}", row, cells.join(", "))
                    }
                })
                .collect();
            format!(
                "Comparison of {} by {}. {}",
                d.rows.join(", "),
                d.columns.join(", "),
                rows.join("; ")
            )
        }
        Diagram::State(d) => {
            let label_of = |key: &str| {
                d.states
                    .iter()
                    .find(|s| s.id.as_deref() == Some(key) || s.label == key)
                    .map(|s| s.label.clone())
                    .unwrap_or_else(|| key.to_string())
            };
            let mut text = format!(
                "States: {}",
                d.states
                    .iter()
                    .map(|s| match s.role {
                        Some(StateRole::Initial) => format!("{} (initial)", s.label),
                        Some(StateRole::Terminal) => format!("{} (final)", s.label),
                        None => s.label.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let transitions: Vec<String> = d
                .transitions
                .iter()
                .map(|t| {
                    let mut s = format!("{} → {}", label_of(&t.from), label_of(&t.to));
                    if let Some(trigger) = &t.trigger {
                        s.push_str(&format!(" on {trigger}"));
                    }
                    if t.kind == TransitionKind::Exception {
                        s.push_str(" (exception)");
                    }
                    s
                })
                .collect();
            if !transitions.is_empty() {
                text.push_str(&format!(". Transitions: {}", transitions.join("; ")));
            }
            text
        }
    };
    format!("{body}.")
}

fn title(diagram: &Diagram) -> Option<&str> {
    match diagram {
        Diagram::Flow(d) => d.title.as_deref(),
        Diagram::Tier(d) => d.title.as_deref(),
        Diagram::Hierarchy(d) => d.title.as_deref(),
        Diagram::Matrix(d) => d.title.as_deref(),
        Diagram::HubSpoke(d) => d.title.as_deref(),
        Diagram::Venn(d) => d.title.as_deref(),
        Diagram::Timeline(d) => d.title.as_deref(),
        Diagram::Comparison(d) => d.title.as_deref(),
        Diagram::State(d) => d.title.as_deref(),
    }
    .filter(|t| !t.trim().is_empty())
}

/// Nested outline: `Root (Child A (Grandchild), Child B)`. Several roots are joined with `;`.
fn hierarchy(d: &HierarchyDiagram) -> String {
    fn key(n: &HierarchyNode) -> &str {
        n.id.as_deref().unwrap_or(&n.label)
    }
    fn outline(d: &HierarchyDiagram, node: &HierarchyNode, depth: usize) -> String {
        let children: Vec<String> = d
            .nodes
            .iter()
            .filter(|c| c.parent.as_deref() == Some(key(node)))
            .map(|c| if depth < 32 { outline(d, c, depth + 1) } else { c.label.clone() })
            .collect();
        if children.is_empty() {
            node.label.clone()
        } else {
            format!("{} ({})", node.label, children.join(", "))
        }
    }
    let roots: Vec<String> = d
        .nodes
        .iter()
        .filter(|n| n.parent.is_none())
        .map(|n| outline(d, n, 0))
        .collect();
    // The fishbone view's spine end is `effect`, falling back to `title` as the renderer does.
    let effect = d.effect.as_deref().or(d.title.as_deref());
    let lead = match (&d.view, effect) {
        (HierarchyView::Fishbone, Some(effect)) => format!("Causes of {effect}: "),
        (HierarchyView::MindMap, _) => "Mind map: ".to_string(),
        _ => "Hierarchy: ".to_string(),
    };
    format!("{lead}{}", roots.join("; "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn process_reads_in_order() {
        let d = parse("kind = \"flow\"\ntitle = \"Pipeline\"\n[[items]]\nlabel = \"업로드\"\n[[items]]\nlabel = \"파싱\"\n[[items]]\nlabel = \"색인\"").unwrap();
        assert_eq!(accessible_name(&d), "Pipeline");
        assert_eq!(describe(&d), "Process: 업로드 → 파싱 → 색인.");
    }

    #[test]
    fn untitled_diagram_is_named_by_kind() {
        let d = parse("kind = \"flow\"\nview = \"cycle\"\n[[items]]\nlabel = \"Plan\"\n[[items]]\nlabel = \"Do\"").unwrap();
        assert_eq!(accessible_name(&d), "Cycle diagram");
        assert_eq!(describe(&d), "Cycle: Plan → Do → Plan.");
    }

    #[test]
    fn hierarchy_reads_as_nested_outline() {
        let d = parse("kind = \"hierarchy\"\nview = \"org_chart\"\n[[nodes]]\nlabel = \"CEO\"\n[[nodes]]\nlabel = \"CTO\"\nparent = \"CEO\"\n[[nodes]]\nlabel = \"Platform\"\nparent = \"CTO\"\n[[nodes]]\nlabel = \"CFO\"\nparent = \"CEO\"").unwrap();
        assert_eq!(describe(&d), "Hierarchy: CEO (CTO (Platform), CFO).");
    }

    #[test]
    fn every_valid_spec_example_has_a_description() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/kinds");
        let Ok(kinds) = std::fs::read_dir(root) else { return };
        let mut seen = 0;
        for kind in kinds.flatten() {
            let Ok(files) = std::fs::read_dir(kind.path().join("valid")) else { continue };
            for f in files.flatten() {
                let path = f.path();
                if path.extension().and_then(|e| e.to_str()) != Some("toml") {
                    continue;
                }
                let input = std::fs::read_to_string(&path).unwrap();
                let d = parse(&input).unwrap();
                let text = describe(&d);
                assert!(text.len() > 3 && text.ends_with('.'), "{}: {:?}", path.display(), text);
                assert!(!accessible_name(&d).is_empty());
                seen += 1;
            }
        }
        assert!(seen > 0 || !std::path::Path::new(root).exists());
    }
}
