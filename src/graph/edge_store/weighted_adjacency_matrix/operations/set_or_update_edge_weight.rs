use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    GetAdjacencyMatrixCoordinateIndices, WeightedAdjacencyMatrix,
};
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;

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

impl<T: ValueType + SetSparseMatrixElementTyped<T>> SetOrUpdateEdgeWeight<T>
    for WeightedAdjacencyMatrix
{
    fn set_or_update_edge_weight_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        Ok(T::set_graphblas_matrix_value(
            self,
            tail.index(),
            head.index(),
            weight,
        )?)
    }

    fn set_or_update_edge_weight_at_coordinate_unchecked(
        &mut self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
        weigth: T,
    ) -> Result<(), GraphComputingError> {
        SetOrUpdateEdgeWeight::<T>::set_or_update_edge_weight_unchecked(
            self,
            coordinate.tail_ref(),
            coordinate.head_ref(),
            weigth,
        )
    }
}
