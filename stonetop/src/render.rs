//! Rendering `PlaybookFixed` for reading aloud. The spoken forms in [`spoken`] and the
//! HTML-to-Markdown converter in [`markdown`] are shared by every view, so they live here
//! behind `ssr` rather than in `codegen`.

pub mod markdown;
pub mod spoken;
