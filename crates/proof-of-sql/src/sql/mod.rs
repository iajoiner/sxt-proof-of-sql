//! This module contains the main logic for Proof of SQL.
pub mod ast;
pub mod parse;
pub mod postprocessing;
pub mod proof;
pub mod transform;
/// Contains testing machinery shared between different
/// modules.
#[cfg(feature = "test")]
pub mod utils;
