pub mod error;
pub mod graph;
pub mod operations;
mod util;

#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// #[cfg(bench)]
// pub mod util;
