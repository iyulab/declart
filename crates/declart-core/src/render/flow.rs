use crate::model::{
    FishboneCause, FishboneDiagram, HierarchyDiagram, HierarchyView,
    Item, ItemsDiagram, OrgChartDiagram, OrgChartNode,
    FlowDiagram, FlowView, TierDiagram, TierView,
};
use super::{concentric, cycle, fishbone, funnel, mind_map, org_chart, process, pyramid, swimlane, theme::Theme};

pub(crate) fn render_flow(d: &FlowDiagram, theme: &Theme) -> String {
    if d.view == FlowView::Swimlane {
        return swimlane::render(d, theme);
    }
    let items: Vec<Item> = d.items.iter().map(|fi| Item {
        label: fi.label.clone(),
        emphasis: fi.emphasis.clone(),
        status: fi.status.clone(),
    }).collect();
    let items_data = ItemsDiagram { title: d.title.clone(), items };
    match d.view {
        FlowView::Process  => process::render(&items_data, theme),
        FlowView::Cycle    => cycle::render(&items_data, theme),
        FlowView::Funnel   => funnel::render(&items_data, theme),
        FlowView::Swimlane => unreachable!(),
    }
}

pub(crate) fn render_tier(d: &TierDiagram, theme: &Theme) -> String {
    let items_data = ItemsDiagram { title: d.title.clone(), items: d.items.clone() };
    match d.view {
        TierView::Pyramid => pyramid::render(&items_data, theme),
        TierView::Concentric => concentric::render(&items_data, theme),
    }
}

fn resolve_nodes(d: &HierarchyDiagram) -> OrgChartDiagram {
    OrgChartDiagram {
        title: d.title.clone(),
        nodes: d.nodes.iter().map(|n| OrgChartNode {
            label: n.label.clone(),
            parent: n.parent.as_ref().map(|p| {
                d.nodes.iter()
                    .find(|other| other.id.as_deref() == Some(p.as_str()))
                    .map(|other| other.label.clone())
                    .unwrap_or_else(|| p.clone())
            }),
        }).collect(),
    }
}

pub(crate) fn render_hierarchy(d: &HierarchyDiagram, theme: &Theme) -> String {
    match d.view {
        HierarchyView::OrgChart => org_chart::render(&resolve_nodes(d), theme),
        HierarchyView::MindMap  => mind_map::render(&resolve_nodes(d), theme),
        HierarchyView::Fishbone => {
            let effect = d.effect.clone().or_else(|| d.title.clone()).unwrap_or_default();
            let root_labels: Vec<&str> = d.nodes.iter()
                .filter(|n| n.parent.is_none())
                .map(|n| n.label.as_str())
                .collect();
            let causes: Vec<FishboneCause> = root_labels.iter().map(|&root_label| {
                // Find children: node's parent matches root node's id or label
                let root_node = d.nodes.iter().find(|n| n.label == root_label);
                let root_id = root_node.and_then(|n| n.id.as_deref());
                let items: Vec<Item> = d.nodes.iter()
                    .filter(|n| {
                        n.parent.as_deref().map(|p| {
                            (root_id.is_some() && Some(p) == root_id) || p == root_label
                        }).unwrap_or(false)
                    })
                    .map(|n| Item { label: n.label.clone(), emphasis: None, status: None })
                    .collect();
                FishboneCause { label: root_label.to_string(), items }
            }).collect();
            let data = FishboneDiagram { title: None, effect, causes };
            fishbone::render(&data, theme)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Item, TierDiagram, TierView};
    use crate::render::DEFAULT_THEME;

    fn tier(view: TierView) -> TierDiagram {
        TierDiagram {
            title: None,
            view,
            items: vec![
                Item { label: "Core".to_string(), emphasis: None, status: None },
                Item { label: "Outer".to_string(), emphasis: None, status: None },
            ],
        }
    }

    #[test]
    fn render_tier_pyramid_uses_polygons() {
        let svg = render_tier(&tier(TierView::Pyramid), &DEFAULT_THEME);
        assert!(svg.contains("<polygon"), "pyramid view should render trapezoid polygons");
    }

    #[test]
    fn render_tier_concentric_uses_circles() {
        let svg = render_tier(&tier(TierView::Concentric), &DEFAULT_THEME);
        assert!(svg.contains("<circle"), "concentric view should render nested circles");
        assert!(!svg.contains("<polygon"), "concentric view should not render pyramid polygons");
    }
}
