use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;

pub(crate) trait DeleteEdge {
    fn delete_weight_at_unchecked_edge_coordinate(
        &mut self,
        coordinate: &(impl GetCoordinateIndices + Copy),
    ) -> Result<(), GraphComputingError>;

    fn delete_edge_weight_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}
