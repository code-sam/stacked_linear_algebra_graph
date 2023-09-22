use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;

use crate::error::GraphComputingError;
use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    SparseWeightedAdjacencyMatrix, SparseWeightedAdjacencyMatrixForValueType,
    WeightedAdjacencyMatrix,
};

use crate::graph::value_type::ValueType;

pub(crate) trait DeleteEdge<T: ValueType> {
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseWeightedAdjacencyMatrixForValueType<T>> DeleteEdge<T>
    for WeightedAdjacencyMatrix
{
    fn delete_edge_unchecked(
        &mut self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError> {
        SparseWeightedAdjacencyMatrix::<T>::sparse_matrix_mut_ref(self)
            .drop_element(coordinate.clone())?;
        Ok(())
    }
}
