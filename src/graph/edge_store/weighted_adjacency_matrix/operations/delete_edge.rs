use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    drop_sparse_matrix_element, drop_sparse_matrix_element_with_coordinate,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;
use graphblas_sparse_linear_algebra::index::ElementIndex;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;

pub(crate) trait DeleteEdge {
    fn delete_weight_at_unchecked_edge_coordinate(
        &mut self,
        coordinate: &impl GetCoordinateIndices,
    ) -> Result<(), GraphComputingError>;

    fn delete_edge_weight_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteEdge for WeightedAdjacencyMatrix {
    fn delete_weight_at_unchecked_edge_coordinate(
        &mut self,
        coordinate: &impl GetCoordinateIndices,
    ) -> Result<(), GraphComputingError> {
        drop_sparse_matrix_element_with_coordinate(self, coordinate)?;
        Ok(())
    }

    fn delete_edge_weight_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
    ) -> Result<(), GraphComputingError> {
        drop_sparse_matrix_element(self, tail, head)?;
        Ok(())
    }
}
