use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::graph::{GetVertexStore, Graph};
use crate::graph::indexing::{VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::{
    GetVertexValue as GetVertexValueFromVertexStore, IntoSparseVectorForValueType,
};

pub trait GetVertexValue<T: ValueType> {
    fn vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

pub(crate) trait GetPrivateVertexValue<T: ValueType> {
    fn private_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<T> GetVertexValue<T> for Graph
where
    T: ValueType + GetVectorElementValueTyped<T> + IntoSparseVectorForValueType<T> + Default,
{
    fn vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .public_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .public_vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn try_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
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
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .private_vertex_value(vertex_type_index, vertex_index)
    }

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .private_vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_private_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }
}

#[cfg(test)]
mod tests {}
