use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    is_element, try_is_element,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::VertexVector;

pub(crate) trait IsElementInVertexVector {
    fn is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_vertex_element(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl IsElementInVertexVector for VertexVector {
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
