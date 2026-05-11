use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeclartError {
    #[error("unknown diagram kind: `{0}`\n  = hint: Valid kinds are: pyramid")]
    UnknownKind(String),

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
