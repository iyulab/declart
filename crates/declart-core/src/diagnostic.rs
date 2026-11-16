//! Diagnostics — what the engine changed to make a label fit.
//!
//! A declaration carries no visual information, so only the engine knows when a label did not fit
//! its slot and was cut short or set smaller. [`render_diagnosed`](crate::render::render_diagnosed)
//! returns those facts next to the SVG, so a pipeline that never looks at the picture can still
//! tell a faithful rendering from one that dropped text.

use serde::Serialize;
use std::cell::RefCell;

/// What the engine did to a label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    /// The label did not fit and was cut short with "…" — text is missing from the picture.
    LabelTruncated,
    /// The label was set smaller than the kind's usual size to fit — nothing is missing.
    LabelShrunk,
}

impl DiagnosticCode {
    /// The stable name of the code, as used in JSON output (`label_truncated`, `label_shrunk`).
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticCode::LabelTruncated => "label_truncated",
            DiagnosticCode::LabelShrunk => "label_shrunk",
        }
    }
}

/// One thing the engine did to make the diagram fit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    /// What happened.
    pub code: DiagnosticCode,
    /// The label as declared.
    pub label: String,
    /// A sentence saying what happened to the label.
    pub message: String,
    /// What to change in the declaration.
    pub hint: String,
}

thread_local! {
    static SINK: RefCell<Option<Vec<Diagnostic>>> = const { RefCell::new(None) };
}

/// Records `code` for `label` when a collection is running (a no-op otherwise). The same code for
/// the same label is recorded once.
pub(crate) fn record(code: DiagnosticCode, label: &str) {
    SINK.with(|sink| {
        let mut sink = sink.borrow_mut();
        let Some(list) = sink.as_mut() else { return };
        if list.iter().any(|d| d.code == code && d.label == label) {
            return;
        }
        let (message, hint) = match code {
            DiagnosticCode::LabelTruncated => (
                format!("label \"{label}\" does not fit its slot and was cut short with \"…\""),
                "shorten the label, or move its detail to another item".to_string(),
            ),
            DiagnosticCode::LabelShrunk => (
                format!("label \"{label}\" was set smaller than usual to fit its slot"),
                "shorten the label to keep the usual size".to_string(),
            ),
        };
        list.push(Diagnostic {
            code,
            label: label.to_string(),
            message,
            hint,
        });
    });
}

/// Runs `f` and returns its result with everything recorded while it ran. Nested collections keep
/// their own lists.
pub(crate) fn collect<T>(f: impl FnOnce() -> T) -> (T, Vec<Diagnostic>) {
    let outer = SINK.with(|sink| sink.borrow_mut().replace(Vec::new()));
    let value = f();
    let recorded = SINK.with(|sink| std::mem::replace(&mut *sink.borrow_mut(), outer));
    (value, recorded.unwrap_or_default())
}
