use std::fmt::Debug;

use graphblas_sparse_linear_algebra::index::ElementIndex as GraphblasElementIndex;

// TODO: is there a need to re-define ElementIndex?
pub(crate) type ElementIndex = GraphblasElementIndex;
pub type ElementCount = ElementIndex;
pub type Index = ElementIndex;
