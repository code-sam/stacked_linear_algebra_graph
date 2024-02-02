use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::index::ElementIndex;
use crate::graph::value_type::ValueType;

pub(crate) trait AddEdge<T: ValueType> {
    fn add_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetAdjacencyMatrixCoordinateIndices + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError>;

    fn add_edge_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
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
        T::set_graphblas_matrix_value(self, edge.tail_ref(), edge.head_ref(), *edge.weight_ref())?;
        Ok(())
    }

    fn add_edge_unchecked(
        &mut self,
        tail: &ElementIndex,
        head: &ElementIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        T::set_graphblas_matrix_value(self, tail, head, weight)?;
        Ok(())
    }
}
