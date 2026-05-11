mod raw;

use crate::error::DeclartError;
use crate::model::{Diagram, Emphasis, Item, ItemsDiagram, MatrixDiagram};

pub fn parse(input: &str) -> Result<Diagram, DeclartError> {
    let probe: raw::KindProbe = toml::from_str(input)?;

    match probe.kind.as_str() {
        "pyramid" | "process" | "cycle" => {
            let raw: raw::RawItemsDiagram = toml::from_str(input)?;
            validate_items(raw)
        }
        "matrix" => {
            let raw: raw::RawMatrixDiagram = toml::from_str(input)?;
            validate_matrix(raw)
        }
        other => Err(DeclartError::UnknownKind(other.to_string())),
    }
}

fn parse_items(raw_items: Vec<raw::RawItem>) -> Result<Vec<Item>, DeclartError> {
    raw_items
        .into_iter()
        .map(|item| {
            let emphasis = match item.emphasis.as_deref() {
                None => None,
                Some("primary") => Some(Emphasis::Primary),
                Some("secondary") => Some(Emphasis::Secondary),
                Some(other) => {
                    return Err(DeclartError::InvalidValue {
                        field: "emphasis".to_string(),
                        value: other.to_string(),
                        hint: "Valid values are: primary, secondary".to_string(),
                    })
                }
            };
            Ok(Item { label: item.label, emphasis })
        })
        .collect()
}

fn validate_items(raw: raw::RawItemsDiagram) -> Result<Diagram, DeclartError> {
    if raw.items.is_empty() {
        return Err(DeclartError::EmptyItems);
    }
    let kind_str = raw.kind.as_str();
    if kind_str == "cycle" && raw.items.len() < 2 {
        return Err(DeclartError::TooFewItems { kind: "cycle", min: 2, got: raw.items.len() });
    }
    let items = parse_items(raw.items)?;
    let inner = ItemsDiagram { title: raw.title, items };
    let diagram = match kind_str {
        "pyramid" => Diagram::Pyramid(inner),
        "process" => Diagram::Process(inner),
        "cycle" => Diagram::Cycle(inner),
        _ => unreachable!(),
    };
    Ok(diagram)
}

fn validate_matrix(raw: raw::RawMatrixDiagram) -> Result<Diagram, DeclartError> {
    debug_assert_eq!(raw.kind, "matrix");
    if raw.quadrants.len() != 4 {
        return Err(DeclartError::InvalidQuadrantCount(raw.quadrants.len()));
    }
    let quadrants = raw
        .quadrants
        .into_iter()
        .map(|q| Item { label: q.label, emphasis: None })
        .collect();
    Ok(Diagram::Matrix(MatrixDiagram {
        title: raw.title,
        x_axis: raw.x_axis,
        y_axis: raw.y_axis,
        quadrants,
    }))
}

#[cfg(test)]
mod tests {
    use crate::model::{Diagram, Emphasis};
    use super::parse;

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
}
