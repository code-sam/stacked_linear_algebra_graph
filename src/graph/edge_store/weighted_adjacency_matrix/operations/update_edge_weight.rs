use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    SetMatrixElement, SetMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;

use crate::error::GraphComputingError;

use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    SparseWeightedAdjacencyMatrix, SparseWeightedAdjacencyMatrixForValueType,
    WeightedAdjacencyMatrix,
};

use crate::graph::value_type::{ValueType};

pub(crate) trait UpdateEdgeWeight<T: ValueType> {
    fn update_edge_weight_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
        weigth: &T,
    ) -> Result<(), GraphComputingError>;
}

impl<
        T: ValueType + Copy + SparseWeightedAdjacencyMatrixForValueType<T> + SetMatrixElementTyped<T>,
    > UpdateEdgeWeight<T> for WeightedAdjacencyMatrix
{
    fn update_edge_weight_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
        weigth: &T,
    ) -> Result<(), GraphComputingError> {
        Ok(self
            .sparse_matrix_mut_ref()
            .set_element(MatrixElement::new(*coordinate, *weigth))?)
    }
}
