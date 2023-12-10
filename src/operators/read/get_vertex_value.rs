use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex};
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
            .vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default(vertex_type_index, vertex_index)
    }

    fn try_vertex_value(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_value(vertex_type_index, vertex_index)
    }
}

#[cfg(test)]
mod tests {}
