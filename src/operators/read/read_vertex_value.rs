use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementValueTyped;

use crate::error::GraphComputingError;

use crate::graph::graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex::{VertexKeyRef, VertexTypeKeyRef};
use crate::graph::vertex_store::vertex_operations::ReadVertex;
use crate::graph::vertex_store::{SparseVertexVector, VertexVector};

pub trait ReadVertexValue<T: ValueType> {
    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    // fn vertex_value_by_vertex_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<T, GraphComputingError>;
}

impl<T> ReadVertexValue<T> for Graph
where
    T: ValueType + GetVectorElementValueTyped<T> + Default,
    VertexVector: SparseVertexVector<T>,
{
    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_by_key(vertex_type_key, vertex_key)
    }

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default_by_key(vertex_type_key, vertex_key)
    }

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_value_by_key(vertex_type_key, vertex_key)
    }

    // fn vertex_value_by_vertex_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<$value_type, GraphComputingError> {
    //     self.vertex_store_ref()
    //         .vertex_value_by_type_index_and_vertex_key(vertex_type_index, vertex_key)
    // }

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_by_index(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_value_or_default_by_index(vertex_type_index, vertex_index)
    }

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_value_by_index(vertex_type_index, vertex_index)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::graph::vertex::VertexValue;

    // TODO
    // #[test]
    // fn new_graph() {
    //     let graph = Graph::new(10, 20);
    // }
}
