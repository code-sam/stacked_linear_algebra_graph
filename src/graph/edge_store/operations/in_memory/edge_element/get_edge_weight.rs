use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_element::{GetEdgeWeight, Indexing};
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;

use crate::graph::edge_store::EdgeStore;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::{IntoValueType, ValueType};
use crate::graph::weighted_adjacency_matrix::operations::GetEdgeWeight as GetEdgeWeightFromAdjacencyMatrix;

impl<T> GetEdgeWeight<T> for EdgeStore
where
    T: ValueType + Default,
    bool: IntoValueType<T>,
    i8: IntoValueType<T>,
    i16: IntoValueType<T>,
    i32: IntoValueType<T>,
    i64: IntoValueType<T>,
    u8: IntoValueType<T>,
    u16: IntoValueType<T>,
    u32: IntoValueType<T>,
    u64: IntoValueType<T>,
    f32: IntoValueType<T>,
    f64: IntoValueType<T>,
    isize: IntoValueType<T>,
    usize: IntoValueType<T>,
{
    fn edge_weight(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;
        self.edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .edge_weight_unchecked(tail, head)
    }

    fn edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_weight(
            vertex_indexer,
            edge_type_index,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }

    fn edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .edge_weight_at_coordinate_unchecked(coordinate)
    }

    fn edge_weight_or_default(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;
        self.edge_weight_or_default_unchecked(edge_type_index, tail, head)
    }

    fn edge_weight_or_default_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .edge_weight_or_default_unchecked(tail, head)
    }

    fn edge_weight_or_default_at_coordinate(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        self.edge_weight_or_default(
            vertex_indexer,
            edge_type_index,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }

    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .edge_weight_or_default_at_coordinate_unchecked(coordinate)
    }

    fn try_edge_weight(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;
        self.try_edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn try_edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .try_edge_weight_unchecked(tail, head)
    }

    fn try_edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError> {
        self.try_edge_weight(
            vertex_indexer,
            edge_type_index,
            coordinate.tail_ref(),
            coordinate.head_ref(),
        )
    }

    fn try_edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError> {
        self.adjacency_matrix_ref_unchecked(edge_type_index)
            .try_edge_weight_at_coordinate_unchecked(coordinate)
    }
}
