use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge_store::operations::operations::edge_element::GetEdgeWeight as GetEdgeWeihtFromEdgeStore;
use crate::graph::edge_store::weighted_adjacency_matrix::IntoSparseMatrixForValueType;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::{IntoValueType, ValueType};

use crate::graph_operators::operator_traits::read::GetEdgeWeight;
use crate::transaction::in_memory::InMemoryGraphTransaction;

impl<'g, T> GetEdgeWeight<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType + IntoSparseMatrixForValueType<T> + GetSparseMatrixElementValueTyped<T> + Default,
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
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_transaction.edge_weight(
            &self.vertex_store_transaction,
            edge_type,
            tail,
            head,
        )
    }

    fn edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.edge_store_transaction.edge_weight(
            &self.vertex_store_transaction,
            edge_coordinate.edge_type_ref(),
            edge_coordinate.tail_ref(),
            edge_coordinate.head_ref(),
        )
    }

    fn try_edge_weight(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_transaction.try_edge_weight(
            &self.vertex_store_transaction,
            edge_type,
            tail,
            head,
        )
    }

    fn try_edge_weight_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_transaction.try_edge_weight(
            &self.vertex_store_transaction,
            edge_coordinate.edge_type_ref(),
            edge_coordinate.tail_ref(),
            edge_coordinate.head_ref(),
        )
    }

    /// Requires valid coordinate
    fn edge_weight_or_default(
        &self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_transaction.edge_weight_or_default(
            &self.vertex_store_transaction,
            edge_type,
            tail,
            head,
        )
    }

    /// Requires valid coordinate
    fn edge_weight_or_default_for_coordinate(
        &self,
        edge_coordinate: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<T, GraphComputingError> {
        self.edge_store_transaction.edge_weight_or_default(
            &self.vertex_store_transaction,
            edge_coordinate.edge_type_ref(),
            edge_coordinate.tail_ref(),
            edge_coordinate.head_ref(),
        )
    }
}

#[cfg(test)]
mod tests {}
