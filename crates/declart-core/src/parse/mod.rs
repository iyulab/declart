mod raw;

use crate::error::DeclartError;
use crate::model::{DiagramKind, DiagramModel, Emphasis, Item};

pub fn parse(input: &str) -> Result<DiagramModel, DeclartError> {
    let raw: raw::RawDiagram = toml::from_str(input)?;
    validate(raw)
}

fn validate(raw: raw::RawDiagram) -> Result<DiagramModel, DeclartError> {
    let kind = match raw.kind.as_str() {
        "pyramid" => DiagramKind::Pyramid,
        other => return Err(DeclartError::UnknownKind(other.to_string())),
    };

    if raw.items.is_empty() {
        return Err(DeclartError::EmptyItems);
    }

    let items = raw
        .items
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
            Ok(Item {
                label: item.label,
                emphasis,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DiagramModel {
        kind,
        title: raw.title,
        items,
    })
}

#[cfg(test)]
mod tests {
    use crate::{DiagramKind, Emphasis};
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
        let model = parse(VALID_PYRAMID).unwrap();
        assert_eq!(model.kind, DiagramKind::Pyramid);
        assert_eq!(model.title, Some("Test Pyramid".to_string()));
        assert_eq!(model.items.len(), 2);
        assert_eq!(model.items[0].label, "Top");
        assert_eq!(model.items[1].emphasis, Some(Emphasis::Primary));
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
        let model = parse(input).unwrap();
        assert!(model.title.is_none());
    }
}
