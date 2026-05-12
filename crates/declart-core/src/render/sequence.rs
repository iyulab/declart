use crate::model::{
    FishboneCause, FishboneDiagram, HierarchyDiagram, HierarchyView,
    Item, ItemsDiagram, OrgChartDiagram, OrgChartNode, SequenceDiagram, SequenceView,
};
use super::{cycle, fishbone, funnel, org_chart, process, pyramid, theme::Theme};

pub(crate) fn render_sequence(d: &SequenceDiagram, theme: &Theme) -> String {
    let items_data = ItemsDiagram { title: d.title.clone(), items: d.items.clone() };
    match d.view {
        SequenceView::Process => process::render(&items_data, theme),
        SequenceView::Cycle   => cycle::render(&items_data, theme),
        SequenceView::Funnel  => funnel::render(&items_data, theme),
        SequenceView::Pyramid => pyramid::render(&items_data, theme),
    }
}

pub(crate) fn render_hierarchy(d: &HierarchyDiagram, theme: &Theme) -> String {
    match d.view {
        HierarchyView::OrgChart => {
            let data = OrgChartDiagram {
                title: d.title.clone(),
                nodes: d.nodes.iter().map(|n| OrgChartNode {
                    label: n.label.clone(),
                    parent: n.parent.clone(),
                }).collect(),
            };
            org_chart::render(&data, theme)
        }
        HierarchyView::Fishbone => {
            let effect = d.title.clone().unwrap_or_default();
            let root_labels: Vec<&str> = d.nodes.iter()
                .filter(|n| n.parent.is_none())
                .map(|n| n.label.as_str())
                .collect();
            let causes: Vec<FishboneCause> = root_labels.iter().map(|&root_label| {
                let items: Vec<Item> = d.nodes.iter()
                    .filter(|n| n.parent.as_deref() == Some(root_label))
                    .map(|n| Item { label: n.label.clone(), emphasis: None })
                    .collect();
                FishboneCause { label: root_label.to_string(), items }
            }).collect();
            let data = FishboneDiagram { title: None, effect, causes };
            fishbone::render(&data, theme)
        }
    }
}
