use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, VertexIndex};

pub(crate) trait Indexing {
    fn is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError>;
    fn is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_edge_at_coordinate(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError>;
    fn try_is_edge(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;

    fn adjacency_matrix_coordinates(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError>;
    fn indices_of_vertices_with_outgoing_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;
    fn indices_of_vertices_with_incoming_edges(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;

    fn indices_of_connected_vertices(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;
}
