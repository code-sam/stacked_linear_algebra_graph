use graphblas_sparse_linear_algebra::collections::sparse_matrix::{Coordinate, SparseMatrixTrait};

use crate::error::GraphComputingError;
use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::{WeightedAdjacencyMatrix, WeightedAdjacencyMatrixTrait};
use crate::graph::value_type::ValueType;

pub(crate) trait DeleteEdge {
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;
}

// impl<T: ValueType> DeleteEdge for WeightedAdjacencyMatrix<T> {
//     fn delete_edge_unchecked(
//         &mut self,
//         coordinate: &AdjacencyMatrixCoordinate,
//     ) -> Result<(), GraphComputingError> {
//         self.sparse_matrix_mut_ref()
//             .drop_element(coordinate.clone())?;
//         Ok(())
//     }
// }
