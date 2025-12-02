// src/lib.rs

// Core curve math (no wasm, pure Rust).
mod curve;
pub use crate::curve::{Curve, CurveConfig, CurveError, CurveSnapshot};
