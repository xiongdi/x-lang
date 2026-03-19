//! XIR compatibility module
//!
//! The canonical low-level IR now lives in the `x-lir` crate.
//! This module is kept only as a compatibility re-export so existing
//! imports like `x_codegen::xir::*` continue to work during the backend
//! migration to the new architecture.
//!
//! Preferred path for new code:
//! - `x_lir::*`
//! - or `x_codegen::xir::*` if code still expects the old location

pub use x_lir::*;
