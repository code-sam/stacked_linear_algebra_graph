use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_element::Indexing;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, VertexIndex};

impl<'s> Indexing for InMemoryEdgeStoreTransaction<'s> {
    fn is_valid_edge(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store
            .is_valid_edge(vertex_indexer, edge_type_index, tail, head)
    }

    fn is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref().is_edge(edge_type_index, tail, head)
    }

    fn is_empty_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref().is_empty_edge(edge_type_index, tail, head)
    }

    fn is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref()
            .is_edge_at_coordinate(edge_type_index, coordinate)
    }

    fn try_is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_is_edge(edge_type_index, tail, head)
    }

    fn try_is_empty_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_is_empty_edge(edge_type_index, tail, head)
    }

    fn try_is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_is_edge_at_coordinate(edge_type_index, coordinate)
    }

    fn adjacency_matrix_coordinates(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError> {
        self.edge_store_ref()
            .adjacency_matrix_coordinates(edge_type_index)
    }

    fn indices_of_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.edge_store_ref()
            .indices_of_vertices_with_outgoing_edges(edge_type_index)
    }

    fn indices_of_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.edge_store_ref()
            .indices_of_vertices_with_incoming_edges(edge_type_index)
    }

    fn indices_of_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.edge_store_ref()
            .indices_of_connected_vertices(edge_type_index)
    }

    fn try_is_valid_edge(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store
            .try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)
    }
}
