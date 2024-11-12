use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    is_element, try_is_element,
};

use crate::{
    error::GraphComputingError,
    graph::{indexing::GetVertexIndexIndex, value_type::ValueType, vertex_store::VertexVector},
};

pub(crate) trait IsElementInVertexVector<T: ValueType> {
    fn is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}
