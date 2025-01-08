use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_element::Indexing;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::Indexing as AdjacencyMatrixIndexing;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, VertexIndex};

impl Indexing for EdgeStore {
    fn is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .is_edge(tail, head)
    }

    fn is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .is_edge_at_coordinate(coordinate)
    }

    fn try_is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .try_is_edge(tail, head)
    }

    fn try_is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .try_is_edge_at_coordinate(coordinate)
    }

    fn adjacency_matrix_coordinates(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .adjacency_matrix_coordinates()
    }

    fn indices_of_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .indices_of_vertices_with_outgoing_edges()
    }

    fn indices_of_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .indices_of_vertices_with_incoming_edges()
    }

    fn indices_of_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        self.adjacency_matrix_ref(edge_type_index)?
            .indices_of_connected_vertices()
    }
}
