use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementListTyped, GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::operators::monoid::AnyMonoidTyped;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::{
    AddEdge as AddEdgeToAdjacencyMatrix, Indexing,
};
use crate::graph::edge_store::weighted_adjacency_matrix::IntoSparseMatrixForValueType;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::operators::indexing::CheckIndex as GraphIndexing;

pub trait AddEdge<T: ValueType> {
    fn add_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait AddPrivateEdge<T: ValueType> {
    fn add_private_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_private_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
