pub mod error;
pub mod graph;
// pub mod operators;

pub use graphblas_sparse_linear_algebra;

#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// #[cfg(bench)]
// pub mod util;
