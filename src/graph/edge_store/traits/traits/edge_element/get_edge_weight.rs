use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;

pub(crate) trait GetEdgeWeight<T> {
    fn edge_weight(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;
    fn edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;
    fn edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError>;

    fn edge_weight_or_default(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
    fn edge_weight_or_default_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError>;

    fn edge_weight_or_default_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError>;
    fn try_edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError>;

    fn try_edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError>;
    fn try_edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError>;
}
