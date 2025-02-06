use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::operations::operations::edge_element::Indexing;
use crate::graph::edge_store::operations::operations::edge_element::SetEdge;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::weighted_adjacency_matrix::operations::SetEdge as AddEdgeToAdjacencyMatrix;

impl<T> SetEdge<T> for EdgeStore
where
    T: ValueType + Copy + SetSparseMatrixElementTyped<T>,
{
    fn set_weighted_directed_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.set_edge(
            vertex_indexer,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn set_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;

        self.set_edge_unchecked(edge_type_index, tail, head, weight)
    }

    fn set_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.set_edge_unchecked(
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn set_edge_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_mut_ref_unchecked(edge_type_index)?
            .set_edge_unchecked(tail, head, weight)
    }
}
