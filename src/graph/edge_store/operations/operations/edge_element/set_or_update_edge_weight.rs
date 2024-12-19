use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;
use crate::graph::indexing::GetVertexIndexIndex;

pub(crate) trait SetOrUpdateEdgeWeight<T> {
    fn set_or_update_edge_weight_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weigth: T,
    ) -> Result<(), GraphComputingError>;

    fn set_or_update_edge_weight_at_coordinate_unchecked(
        &mut self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
        weight: T,
    ) -> Result<(), GraphComputingError>;
}
