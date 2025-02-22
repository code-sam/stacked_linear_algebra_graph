use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::{IntoValueType, ValueType};

use crate::graph::vertex_store::traits::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction,
};
use crate::graph::vertex_store::traits::vertex_element::GetVertexValue;
use crate::graph::vertex_store::ToSparseVectorForValueType;

impl<'s, T> GetVertexValue<T> for InMemoryVertexStoreTransaction<'s>
where
    T: ValueType + ToSparseVectorForValueType<T> + GetSparseVectorElementValueTyped<T> + Default,
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
    fn vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value(vertex_type_index, vertex_index)
    }

    fn try_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn try_vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }
}
