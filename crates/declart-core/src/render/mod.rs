pub(crate) mod cycle;
pub(crate) mod fishbone;
pub(crate) mod font;
pub(crate) mod funnel;
pub(crate) mod hub_spoke;
pub(crate) mod matrix;
pub(crate) mod org_chart;
pub(crate) mod process;
pub(crate) mod pyramid;
mod svg;
pub mod theme;
pub(crate) mod timeline;
pub(crate) mod venn;

pub use theme::{Theme, ACCESSIBLE_THEME, DEFAULT_THEME, MONOCHROME_THEME, WARM_THEME};

use crate::{Diagram, DeclartError};

/// Renders `diagram` to an SVG string using the given `theme`.
pub fn render(diagram: &Diagram, theme: &Theme) -> Result<String, DeclartError> {
    render_opts(diagram, theme, None)
}

/// Renders `diagram` to an SVG string. When `width` is `Some(px)`, the outer SVG viewport is
/// scaled to that pixel width while the internal viewBox is preserved.
pub fn render_opts(diagram: &Diagram, theme: &Theme, width: Option<u32>) -> Result<String, DeclartError> {
    let svg = match diagram {
        Diagram::Pyramid(d) => pyramid::render(d, theme),
        Diagram::Process(d) => process::render(d, theme),
        Diagram::Cycle(d) => cycle::render(d, theme),
        Diagram::Matrix(d) => matrix::render(d, theme),
        Diagram::HubSpoke(d) => hub_spoke::render(d, theme),
        Diagram::Venn(d) => venn::render(d, theme),
        Diagram::Timeline(d) => timeline::render(d, theme),
        Diagram::Fishbone(d) => fishbone::render(d, theme),
        Diagram::OrgChart(d) => org_chart::render(d, theme),
        Diagram::Funnel(d) => funnel::render(d, theme),
    };
    if let Some(w) = width {
        Ok(apply_width(svg, w))
    } else {
        Ok(svg)
    }
}

/// Rescales the outer SVG viewport to `target_w` pixels wide, preserving aspect ratio.
/// The internal viewBox is kept unchanged so all coordinates remain valid.
fn apply_width(svg: String, target_w: u32) -> String {
    // SVG header format: <svg xmlns="..." width="{w}" height="{h}" viewBox="0 0 {w} {h}">
    let nat_w = extract_attr(&svg, "width=\"").unwrap_or(600.0);
    let nat_h = extract_attr(&svg, "height=\"").unwrap_or(400.0);
    let scale = target_w as f32 / nat_w;
    let new_h = (nat_h * scale).round() as u32;

    // Replace the first occurrence of width="..." and height="..." in the svg element
    let svg = replace_first_attr(svg, "width=\"", nat_w, target_w as f32);
    replace_first_attr(svg, "height=\"", nat_h, new_h as f32)
}

fn extract_attr(svg: &str, prefix: &str) -> Option<f32> {
    // prefix includes the opening quote, e.g. `width="`; value ends at the next `"`
    let start = svg.find(prefix)? + prefix.len();
    let end = svg[start..].find('"')? + start;
    svg[start..end].parse().ok()
}

fn replace_first_attr(svg: String, prefix: &str, _old_val: f32, new_val: f32) -> String {
    if let Some(start) = svg.find(prefix) {
        let after_prefix = start + prefix.len();
        if let Some(end_offset) = svg[after_prefix..].find('"') {
            let end = after_prefix + end_offset;
            let new_val_str = format!("{}", new_val as u32);
            return format!("{}{}{}", &svg[..after_prefix], new_val_str, &svg[end..]);
        }
    }
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_width_scales_height_proportionally() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="600" height="400" viewBox="0 0 600 400"></svg>"#.to_string();
        let result = apply_width(svg, 300);
        assert!(result.contains("width=\"300\""), "width should be 300");
        assert!(result.contains("height=\"200\""), "height should scale to 200");
        assert!(result.contains("viewBox=\"0 0 600 400\""), "viewBox should be unchanged");
    }

    #[test]
    fn apply_width_same_size_is_identity() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="600" height="380" viewBox="0 0 600 380"></svg>"#.to_string();
        let result = apply_width(svg.clone(), 600);
        assert!(result.contains("width=\"600\""));
        assert!(result.contains("height=\"380\""));
    }
}
