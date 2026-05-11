pub(crate) mod cycle;
pub(crate) mod font;
pub(crate) mod matrix;
pub(crate) mod process;
pub(crate) mod pyramid;
mod svg;
pub mod theme;

pub use theme::{Theme, DEFAULT_THEME};

use crate::{Diagram, DeclartError};

pub fn render(diagram: &Diagram, theme: &Theme) -> Result<String, DeclartError> {
    match diagram {
        Diagram::Pyramid(d) => Ok(pyramid::render(d, theme)),
        Diagram::Process(d) => Ok(process::render(d, theme)),
        Diagram::Cycle(d) => Ok(cycle::render(d, theme)),
        Diagram::Matrix(d) => Ok(matrix::render(d, theme)),
    }
}
