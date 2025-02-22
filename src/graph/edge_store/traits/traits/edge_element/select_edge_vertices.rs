use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;

use crate::error::GraphComputingError;
use crate::graph::indexing::GetEdgeTypeIndex;

pub(crate) trait GetEdgeVerticesMask {
    fn select_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_vertices_with_outgoing_edges_using_transpose(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
    fn select_connected_vertices_using_transpose(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseVector<bool>, GraphComputingError>;
}

// pub(crate) trait SelectEdgeVertices<T: ValueType> {
//     fn select_vertices_with_outgoing_edges(
//         &self,
//     ) -> Result<SparseVector<bool>, GraphComputingError>;
//     fn select_vertices_with_incoming_edges(
//         &self,
//     ) -> Result<SparseVector<bool>, GraphComputingError>;
//     fn select_connected_vertices(&self) -> Result<SparseVector<bool>, GraphComputingError>;
// }
