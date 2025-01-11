use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::GetIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait SetEdge<T: ValueType> {
    fn set_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn set_edge_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> SetEdge<T> for WeightedAdjacencyMatrix
where
    T: ValueType + Copy + SetSparseMatrixElementTyped<T>,
{
    fn set_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        T::set_graphblas_matrix_value(
            self,
            edge.tail_ref().index(),
            edge.head_ref().index(),
            *edge.weight_ref(),
        )?;
        Ok(())
    }

    fn set_edge_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        T::set_graphblas_matrix_value(self, tail.index(), head.index(), weight)?;
        Ok(())
    }
}
