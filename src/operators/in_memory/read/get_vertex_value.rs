use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::graph::{GetVertexStore, Graph};
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::{
    GetVertexValue as GetVertexValueFromVertexStore, IntoSparseVectorForValueType,
};
use crate::operators::operators::read::{GetPrivateVertexValue, GetVertexValue};

impl<T> GetVertexValue<T> for Graph
where
    T: ValueType + GetVectorElementValueTyped<T> + IntoSparseVectorForValueType<T> + Default,
{
    fn vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .public_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .public_vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn try_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_public_vertex_value(vertex_type_index, vertex_index)
    }
}

impl<T> GetPrivateVertexValue<T> for Graph
where
    T: ValueType + GetVectorElementValueTyped<T> + IntoSparseVectorForValueType<T> + Default,
{
    fn private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .private_vertex_value(vertex_type_index, vertex_index)
    }

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .private_vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_private_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_unchecked(vertex_type_index, vertex_index)
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

#[cfg(test)]
mod tests {}
