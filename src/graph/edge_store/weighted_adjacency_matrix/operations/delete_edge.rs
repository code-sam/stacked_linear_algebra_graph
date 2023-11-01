use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::drop_sparse_matrix_element;

use crate::error::GraphComputingError;
use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;

use crate::graph::value_type::ValueType;

pub(crate) trait DeleteEdge<T: ValueType> {
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType> DeleteEdge<T> for WeightedAdjacencyMatrix {
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError> {
        drop_sparse_matrix_element(self, *coordinate)?;
        Ok(())
    }
}
