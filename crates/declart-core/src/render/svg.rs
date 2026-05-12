pub struct SvgBuilder {
    width: f32,
    height: f32,
    elements: Vec<String>,
}

impl SvgBuilder {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn polygon(&mut self, points: &[(f32, f32)], fill: &str, stroke: &str) {
        self.polygon_stroked(points, fill, stroke, 0.0);
    }

    pub fn polygon_stroked(
        &mut self,
        points: &[(f32, f32)],
        fill: &str,
        stroke: &str,
        stroke_width: f32,
    ) {
        let pts: String = points
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        self.elements.push(format!(
            r#"<polygon points="{}" fill="{}" stroke="{}" stroke-width="{:.1}"/>"#,
            pts, fill, stroke, stroke_width
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rect_rounded(
        &mut self,
        x: f32, y: f32, width: f32, height: f32, rx: u32,
        fill: &str, stroke: &str, stroke_width: f32,
    ) {
        self.elements.push(format!(
            r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="{}" fill="{}" stroke="{}" stroke-width="{:.1}"/>"#,
            x, y, width, height, rx, fill, stroke, stroke_width
        ));
    }

    pub fn circle(&mut self, cx: f32, cy: f32, r: f32, fill: &str, fill_opacity: f32) {
        self.elements.push(format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" fill-opacity="{:.2}"/>"#,
            cx, cy, r, fill, fill_opacity
        ));
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, stroke: &str, stroke_width: f32) {
        self.elements.push(format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{:.1}"/>"#,
            x1, y1, x2, y2, stroke, stroke_width
        ));
    }

    pub fn circle_stroked(
        &mut self, cx: f32, cy: f32, r: f32,
        fill: &str, stroke: &str, stroke_width: f32,
    ) {
        self.elements.push(format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" stroke="{}" stroke-width="{:.1}"/>"#,
            cx, cy, r, fill, stroke, stroke_width
        ));
    }

    pub fn line_dashed(
        &mut self, x1: f32, y1: f32, x2: f32, y2: f32,
        stroke: &str, stroke_width: f32,
    ) {
        self.elements.push(format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{:.1}" stroke-dasharray="6,3"/>"#,
            x1, y1, x2, y2, stroke, stroke_width
        ));
    }

    pub fn text(&mut self, x: f32, y: f32, content: &str, fill: &str, font_size: f32) {
        self.elements.push(format!(
            r#"<text x="{:.1}" y="{:.1}" fill="{}" font-size="{:.1}" font-family="Noto Sans, sans-serif" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
            x, y, fill, font_size, escape_xml(content)
        ));
    }

    pub fn text_weighted(
        &mut self,
        x: f32,
        y: f32,
        content: &str,
        fill: &str,
        font_size: f32,
        bold: bool,
    ) {
        let weight = if bold { "bold" } else { "normal" };
        self.elements.push(format!(
            r#"<text x="{:.1}" y="{:.1}" fill="{}" font-size="{:.1}" font-family="Noto Sans, sans-serif" font-weight="{}" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
            x, y, fill, font_size, weight, escape_xml(content)
        ));
    }

    pub fn text_rotated(
        &mut self,
        x: f32,
        y: f32,
        content: &str,
        fill: &str,
        font_size: f32,
        degrees: f32,
    ) {
        self.elements.push(format!(
            r#"<text x="{:.1}" y="{:.1}" fill="{}" font-size="{:.1}" font-family="Noto Sans, sans-serif" text-anchor="middle" dominant-baseline="middle" transform="rotate({:.1},{:.1},{:.1})">{}</text>"#,
            x, y, fill, font_size, degrees, x, y, escape_xml(content)
        ));
    }

    pub fn build(self, background: &str) -> String {
        let elements = self.elements.join("\n  ");
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0}" height="{h:.0}" viewBox="0 0 {w:.0} {h:.0}">
  <rect width="100%" height="100%" fill="{bg}"/>
  {el}
</svg>"#,
            w = self.width,
            h = self.height,
            bg = background,
            el = elements,
        )
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_document_has_xmlns() {
        let builder = SvgBuilder::new(100.0, 100.0);
        let svg = builder.build("#ffffff");
        assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    }

    #[test]
    fn polygon_appears_in_output() {
        let mut builder = SvgBuilder::new(200.0, 200.0);
        builder.polygon(
            &[(10.0, 10.0), (190.0, 10.0), (190.0, 100.0), (10.0, 100.0)],
            "#1a3a5c",
            "none",
        );
        let svg = builder.build("#ffffff");
        assert!(svg.contains("<polygon"));
        assert!(svg.contains("10.0,10.0"));
    }

    #[test]
    fn rect_rounded_produces_rect_element() {
        let mut builder = SvgBuilder::new(300.0, 200.0);
        builder.rect_rounded(10.0, 20.0, 100.0, 50.0, 8, "#336699", "none", 0.0);
        let svg = builder.build("#ffffff");
        assert!(svg.contains("<rect"), "should contain a <rect element");
        assert!(svg.contains("rx=\"8\""), "should have rx attribute");
        assert!(svg.contains("fill=\"#336699\""));
    }

    #[test]
    fn text_content_is_escaped() {
        let mut builder = SvgBuilder::new(200.0, 100.0);
        builder.text(100.0, 50.0, "A & B <C>", "#000000", 14.0);
        let svg = builder.build("#ffffff");
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&lt;"));
        assert!(svg.contains("&gt;"));
    }

    #[test]
    fn circle_stroked_has_stroke_attribute() {
        let mut b = SvgBuilder::new(200.0, 200.0);
        b.circle_stroked(100.0, 100.0, 30.0, "#ffffff", "#003087", 2.0);
        let svg = b.build("#ffffff");
        assert!(svg.contains("stroke=\"#003087\""));
        assert!(svg.contains("stroke-width=\"2.0\""));
    }

    #[test]
    fn line_dashed_contains_dasharray() {
        let mut b = SvgBuilder::new(200.0, 200.0);
        b.line_dashed(0.0, 0.0, 100.0, 100.0, "#000000", 1.5);
        let svg = b.build("#ffffff");
        assert!(svg.contains("stroke-dasharray"));
    }
}
