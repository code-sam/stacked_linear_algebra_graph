use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};

use crate::graph::indexing::{
    GetAssignedIndexData, GetVertexIndexIndex, GetVertexTypeIndex, VertexIndex,
};
use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex::{GetVertexIndex, GetVertexValue};
use crate::graph::vertex_store::AddVertex as AddVertexToStore;

pub trait AddVertex<T: ValueType> {
    fn add_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

pub(crate) trait AddPrivateVertex<T: ValueType> {
    fn add_private_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError>;

    fn add_or_update_private_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<VertexIndex>, GraphComputingError>;

    fn add_or_update_private_vertex_from_vertex(
        &mut self,
        vertex: &(impl GetVertexIndex + GetVertexValue<T>),
    ) -> Result<Option<VertexIndex>, GraphComputingError>;
}

#[cfg(test)]
mod tests {}
