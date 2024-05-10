use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait AddEdge<T: ValueType> {
    fn add_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn add_edge_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> AddEdge<T> for WeightedAdjacencyMatrix
where
    T: ValueType + Copy + SetSparseMatrixElementTyped<T>,
{
    fn add_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        T::set_graphblas_matrix_value(
            self,
            edge.tail_ref().index_ref(),
            edge.head_ref().index_ref(),
            *edge.weight_ref(),
        )?;
        Ok(())
    }

    fn add_edge_unchecked(
        &mut self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        T::set_graphblas_matrix_value(self, tail.index_ref(), head.index_ref(), weight)?;
        Ok(())
    }
}
