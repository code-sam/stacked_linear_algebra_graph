use graphblas_sparse_linear_algebra::index::ElementIndex as GraphblasElementIndex;
use std::fmt::Debug;

// TODO: is there a need to re-define ElementIndex?
pub(crate) type ElementIndex = GraphblasElementIndex;
pub type ElementCount = ElementIndex;
pub(crate) type Index = ElementIndex;

// pub type VertexIndex = Index;
// pub type VertexTypeIndex = Index;
// pub type EdgeTypeIndex = Index;

pub trait GetIndex: Debug {
    fn index_ref(&self) -> &Index;
    fn index(&self) -> Index;
}
