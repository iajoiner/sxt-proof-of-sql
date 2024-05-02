//! TODO: add docs
pub(crate) mod bit;
pub mod commitment;
pub mod database;
pub(crate) mod encode;
pub mod math;
pub(crate) mod polynomial;
pub(crate) mod proof;
pub(crate) mod ref_into;
pub mod scalar;
mod serialize;
pub(crate) use serialize::impl_serde_for_ark_serde;
pub(crate) mod slice_ops;
