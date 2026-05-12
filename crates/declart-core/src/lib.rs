//! Declart core library — parse and render declarative diagram definitions.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! declart-core = "0.14"
//! ```
//!
//! ```rust,no_run
//! use declart_core::{parse, render};
//! use declart_core::render::DEFAULT_THEME;
//!
//! let input = r#"
//! kind = "sequence"
//! view = "pyramid"
//! title = "Example"
//!
//! [[items]]
//! label = "Top"
//!
//! [[items]]
//! label = "Bottom"
//! "#;
//!
//! let diagram = parse(input).unwrap();
//! let svg = render(&diagram, &DEFAULT_THEME).unwrap();
//! println!("{}", svg);
//! ```

mod error;
mod model;
pub(crate) mod parse;
pub mod render;

pub use error::DeclartError;
pub use model::{
    ComparisonCell, ComparisonDiagram, Diagram, Emphasis,
    HierarchyDiagram, HierarchyNode, HierarchyView,
    HubSpokeDiagram, Item, ItemsDiagram, MatrixDiagram,
    SequenceDiagram, SequenceView,
    TimelineDiagram, TimelineEvent, VennDiagram, VennIntersection, VennSet,
};
pub use parse::{parse, parse_auto, parse_json};
pub use render::{render, render_opts, ACCESSIBLE_THEME, WARM_THEME};
