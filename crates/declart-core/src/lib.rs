mod error;
mod model;
pub mod parse;
pub mod render;

pub use error::DeclartError;
pub use model::{DiagramKind, DiagramModel, Emphasis, Item};
pub use parse::parse;
pub use render::render;
