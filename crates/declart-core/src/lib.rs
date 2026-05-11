mod error;
mod model;
pub(crate) mod parse;
pub mod render;

pub use error::DeclartError;
pub use model::{Diagram, Emphasis, Item, ItemsDiagram, MatrixDiagram};
pub use parse::parse;
pub use render::render;
