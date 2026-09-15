//! Rendering `PlaybookFixed` and the Setting overview for reading aloud. The spoken forms in
//! [`spoken`] and the HTML-to-Markdown converter in [`markdown`] are shared by every view, so
//! they live here behind `ssr` rather than in `codegen`. [`render_markdown`] walks a whole
//! playbook, and [`render_setting_markdown`] the overview, into one Markdown document, the
//! form the golden files under `tests/golden/` hold.

mod document;
pub mod markdown;
mod playbook;
mod setting;
pub mod spoken;
pub mod starting_moves;

pub use playbook::render_markdown;
pub use setting::render_setting_markdown;
