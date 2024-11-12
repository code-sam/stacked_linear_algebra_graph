use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    is_element, try_is_element,
};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::GetVertexIndexIndex,
        value_type::ValueType,
        vertex_store::{IsElementInVertexVector, VertexVector},
    },
};

impl<T: ValueType> IsElementInVertexVector<T> for VertexVector {
    fn is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(is_element(self, *vertex_index.index_ref())?)
    }

    fn try_is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(try_is_element(self, *vertex_index.index_ref())?)
    }
}
