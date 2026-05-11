pub(crate) mod font;
pub(crate) mod pyramid;
mod svg;
pub mod theme;

pub use theme::{Theme, DEFAULT_THEME};

use crate::{DiagramKind, DiagramModel, DeclartError};

pub fn render(model: &DiagramModel, theme: &Theme) -> Result<String, DeclartError> {
    match model.kind {
        DiagramKind::Pyramid => Ok(pyramid::render(model, theme)),
    }
}
