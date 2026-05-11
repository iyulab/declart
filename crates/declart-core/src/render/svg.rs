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
        let pts: String = points
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        self.elements.push(format!(
            r#"<polygon points="{}" fill="{}" stroke="{}" stroke-width="0"/>"#,
            pts, fill, stroke
        ));
    }

    pub fn text(&mut self, x: f32, y: f32, content: &str, fill: &str, font_size: f32) {
        self.elements.push(format!(
            r#"<text x="{:.1}" y="{:.1}" fill="{}" font-size="{:.1}" font-family="Noto Sans, sans-serif" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
            x, y, fill, font_size, escape_xml(content)
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
    fn text_content_is_escaped() {
        let mut builder = SvgBuilder::new(200.0, 100.0);
        builder.text(100.0, 50.0, "A & B <C>", "#000000", 14.0);
        let svg = builder.build("#ffffff");
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&lt;"));
        assert!(svg.contains("&gt;"));
    }
}
