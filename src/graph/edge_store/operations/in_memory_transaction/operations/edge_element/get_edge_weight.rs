use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_element::GetEdgeWeight;
use crate::graph::edge_store::weighted_adjacency_matrix::GetAdjacencyMatrixCoordinateIndices;

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::{IntoValueType, ValueType};
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;

impl<'s, T> GetEdgeWeight<T> for InMemoryEdgeStoreTransaction<'s>
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
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight(vertex_indexer, edge_type_index, tail, head)
    }

    fn edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_at_coordinate(vertex_indexer, edge_type_index, coordinate)
    }

    fn edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_at_coordinate_unchecked(edge_type_index, coordinate)
    }

    fn edge_weight_or_default(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_or_default(vertex_indexer, edge_type_index, tail, head)
    }

    fn edge_weight_or_default_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_or_default_unchecked(edge_type_index, tail, head)
    }

    fn edge_weight_or_default_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref().edge_weight_or_default_at_coordinate(
            vertex_indexer,
            edge_type_index,
            coordinate,
        )
    }

    fn edge_weight_or_default_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .edge_weight_or_default_at_coordinate_unchecked(edge_type_index, coordinate)
    }

    fn try_edge_weight(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .try_edge_weight(vertex_indexer, edge_type_index, tail, head)
    }

    fn try_edge_weight_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &(impl GetVertexIndexIndex + Debug),
        head: &(impl GetVertexIndexIndex + Debug),
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .try_edge_weight_unchecked(edge_type_index, tail, head)
    }

    fn try_edge_weight_at_coordinate(
        &self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref().try_edge_weight_at_coordinate(
            vertex_indexer,
            edge_type_index,
            coordinate,
        )
    }

    fn try_edge_weight_at_coordinate_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
        coordinate: &impl GetAdjacencyMatrixCoordinateIndices,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_ref()
            .try_edge_weight_at_coordinate_unchecked(edge_type_index, coordinate)
    }
}
