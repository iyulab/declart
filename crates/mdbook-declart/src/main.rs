use declart_core::render::DEFAULT_THEME;
use serde_json::Value;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // mdBook calls `mdbook-declart supports <renderer>` to check compatibility.
    // Exit 0 = supported; exit 1 = unsupported.
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        // Support all renderers (html, linkcheck, etc.)
        std::process::exit(0);
    }

    // Normal invocation: read [PreprocessorContext, Book] JSON from stdin.
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut doc: Value =
        serde_json::from_str(&input).expect("invalid mdBook preprocessor input JSON");

    // doc is [ctx, book]; we modify book (index 1) in place.
    if let Some(sections) = doc[1]["sections"].as_array_mut() {
        process_items(sections);
    }

    // Write modified book JSON to stdout.
    let out = serde_json::to_string(&doc[1]).expect("failed to serialize book");
    io::stdout()
        .write_all(out.as_bytes())
        .expect("failed to write stdout");
}

fn process_items(items: &mut Vec<Value>) {
    for item in items {
        if let Some(chapter) = item.get_mut("Chapter") {
            if let Some(content) = chapter["content"].as_str() {
                let processed = process_content(content);
                chapter["content"] = Value::String(processed);
            }
            if let Some(sub_items) = chapter["sub_items"].as_array_mut() {
                process_items(sub_items);
            }
        }
    }
}

fn process_content(content: &str) -> String {
    let fence_open = "```declart\n";
    let fence_close = "\n```";

    let mut result = String::with_capacity(content.len());
    let mut remaining = content;

    while let Some(start) = remaining.find(fence_open) {
        result.push_str(&remaining[..start]);
        remaining = &remaining[start + fence_open.len()..];

        if let Some(end) = remaining.find(fence_close) {
            let diagram_src = &remaining[..end];
            remaining = &remaining[end + fence_close.len()..];
            // Consume optional trailing newline after closing fence
            if remaining.starts_with('\n') {
                remaining = &remaining[1..];
            }

            match render_diagram(diagram_src) {
                Ok(svg) => {
                    result.push_str("<figure class=\"declart\">\n");
                    result.push_str(&svg);
                    result.push_str("\n</figure>\n");
                }
                Err(e) => {
                    result.push_str("<div class=\"declart-error\"><pre>");
                    result.push_str(&html_escape(&e.to_string()));
                    result.push_str("</pre></div>\n");
                }
            }
        } else {
            // Unclosed code block — restore and stop processing
            result.push_str(fence_open);
            result.push_str(remaining);
            return result;
        }
    }

    result.push_str(remaining);
    result
}

fn render_diagram(src: &str) -> Result<String, declart_core::DeclartError> {
    let diagram = declart_core::parse_auto(src)?;
    declart_core::render_opts(&diagram, &DEFAULT_THEME, None)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_no_declart_block() {
        let input = "# Hello\n\n```rust\nfn main() {}\n```\n";
        assert_eq!(process_content(input), input);
    }

    #[test]
    fn renders_valid_declart_block() {
        let input = concat!(
            "Before\n\n",
            "```declart\n",
            "kind = \"pyramid\"\n",
            "[[items]]\nlabel = \"Top\"\n",
            "[[items]]\nlabel = \"Base\"\n",
            "```\n\n",
            "After\n"
        );
        let out = process_content(input);
        assert!(out.contains("<figure class=\"declart\">"), "should wrap in figure");
        assert!(out.contains("<svg"), "should contain SVG");
        assert!(out.contains("Before"), "prefix preserved");
        assert!(out.contains("After"), "suffix preserved");
    }

    #[test]
    fn renders_error_block_for_invalid_kind() {
        let input = concat!(
            "```declart\n",
            "kind = \"nonexistent\"\n",
            "```\n"
        );
        let out = process_content(input);
        assert!(out.contains("declart-error"), "should render error div");
    }

    #[test]
    fn handles_unclosed_fence_gracefully() {
        let input = "```declart\nkind = \"pyramid\"\n";
        let out = process_content(input);
        // Unclosed block → restore original content
        assert!(out.contains("```declart\n"));
    }

    #[test]
    fn html_escape_special_chars() {
        assert_eq!(html_escape("<div>&\"</div>"), "&lt;div&gt;&amp;&quot;&lt;/div&gt;");
    }
}
