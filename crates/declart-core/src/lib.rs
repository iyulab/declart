//! Declart core library — parse and render declarative diagram definitions.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! declart-core = "0.10"
//! ```
//!
//! ```rust,no_run
//! use declart_core::{parse, render};
//! use declart_core::render::DEFAULT_THEME;
//!
//! let input = r#"
//! kind = "pyramid"
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
    Diagram, Emphasis, FishboneCause, FishboneDiagram, HubSpokeDiagram, Item, ItemsDiagram,
    MatrixDiagram, OrgChartDiagram, OrgChartNode, TimelineDiagram, TimelineEvent, VennDiagram,
    VennIntersection, VennSet,
};
pub use parse::parse;
pub use render::{render, render_opts, ACCESSIBLE_THEME, WARM_THEME};
