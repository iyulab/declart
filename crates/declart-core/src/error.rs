use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeclartError {
    #[error("unknown diagram kind: `{0}`\n  = hint: Valid kinds are: pyramid, process, cycle, matrix")]
    UnknownKind(String),

    #[error("invalid quadrant count: expected exactly 4, got {0}")]
    InvalidQuadrantCount(usize),

    #[error("not enough items: `{kind}` requires at least {min} items, got {got}")]
    TooFewItems { kind: &'static str, min: usize, got: usize },

    #[error("empty items: at least one [[items]] entry is required")]
    EmptyItems,

    #[error("invalid value `{value}` for field `{field}`\n  = hint: {hint}")]
    InvalidValue {
        field: String,
        value: String,
        hint: String,
    },

    #[error("parse error\n  = hint: Check for forbidden fields (color, font, size, etc.) or missing required fields.")]
    Parse(#[from] toml::de::Error),
}
