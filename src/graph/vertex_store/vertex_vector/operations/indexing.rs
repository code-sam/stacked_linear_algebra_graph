use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    is_element, try_is_element,
};

use crate::{
    error::GraphComputingError,
    graph::{indexing::VertexIndex, value_type::ValueType, vertex_store::VertexVector},
};

pub(crate) trait IsElementInVertexVector<T: ValueType> {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError>;
}

impl<T: ValueType> IsElementInVertexVector<T> for VertexVector {
    fn is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<bool, GraphComputingError> {
        Ok(is_element(self, *vertex_index)?)
    }

    fn try_is_vertex_element(&self, vertex_index: &VertexIndex) -> Result<(), GraphComputingError> {
        Ok(try_is_element(self, *vertex_index)?)
    }
}
