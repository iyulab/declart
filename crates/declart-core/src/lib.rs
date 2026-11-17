//! Declart core library — parse and render declarative diagram definitions.
//!
//! # Quick start
//!
//! ```toml
//! [dependencies]
//! declart-core = "0.23"
//! ```
//!
//! ```rust,no_run
//! use declart_core::{parse, render};
//! use declart_core::render::DEFAULT_THEME;
//!
//! let input = r#"
//! kind = "flow"
//! view = "cycle"
//! title = "Example"
//!
//! [[items]]
//! label = "Plan"
//!
//! [[items]]
//! label = "Do"
//! "#;
//!
//! let diagram = parse(input).unwrap();
//! let svg = render(&diagram, &DEFAULT_THEME).unwrap();
//! println!("{}", svg);
//! ```

mod date;
mod describe;
mod diagnostic;
mod error;
mod limits;
mod model;
pub(crate) mod parse;
pub mod render;

pub use describe::{accessible_name, describe};
pub use diagnostic::{Diagnostic, DiagnosticCode};
pub use error::DeclartError;
pub use limits::{limits, Capacity, SlotLimit, MAX_PROBED};
pub use model::{
    ComparisonCell, ComparisonDiagram, Diagram, Emphasis, FlowDiagram, FlowView, HierarchyDiagram,
    HierarchyNode, HierarchyView, HubSpokeDiagram, Item, ItemsDiagram, LinkDirection, LinkRelation,
    MatrixDiagram, SystemDiagram, SystemGroup, SystemLink, SystemNode, SystemRole, TierDiagram,
    TierView, TimelineDiagram, TimelineEvent, TimelineWhen, VennDiagram, VennIntersection, VennSet,
};
pub use parse::{parse, parse_auto, parse_json, views, KINDS};
pub use render::{render, render_diagnosed, render_opts, ACCESSIBLE_THEME, WARM_THEME};
