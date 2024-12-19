use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::indexing::{GetVertexIndexIndex, VertexIndex};
use crate::error::GraphComputingError;

pub(crate) trait Indexing<T> {
    fn is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError>;
    fn is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError>;
    fn try_is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn adjacency_matrix_coordinates(
        &self,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError>;
    fn indices_of_vertices_with_outgoing_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;
    fn indices_of_vertices_with_incoming_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;

    fn indices_of_connected_vertices(&self) -> Result<Vec<VertexIndex>, GraphComputingError>;
}
