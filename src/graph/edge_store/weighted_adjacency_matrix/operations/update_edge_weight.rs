use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    GetAdjacencyMatrixCoordinateIndices, WeightedAdjacencyMatrix,
};
use crate::graph::index::VertexIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait UpdateEdgeWeight<T> {
    fn update_edge_weight_unchecked(
        &mut self,
        tail: &VertexIndex,
        head: &VertexIndex,
        weigth: T,
    ) -> Result<(), GraphComputingError>;

    fn update_edge_weight_at_coordinate_unchecked(
        &mut self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SetSparseMatrixElementTyped<T>> UpdateEdgeWeight<T>
    for WeightedAdjacencyMatrix
{
    fn update_edge_weight_unchecked(
        &mut self,
        tail: &VertexIndex,
        head: &VertexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        Ok(T::set_graphblas_matrix_value(self, tail, head, weight)?)
    }

    fn update_edge_weight_at_coordinate_unchecked(
        &mut self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
        weigth: T,
    ) -> Result<(), GraphComputingError> {
        UpdateEdgeWeight::<T>::update_edge_weight_unchecked(
            self,
            coordinate.tail_ref(),
            coordinate.head_ref(),
            weigth,
        )
    }
}
