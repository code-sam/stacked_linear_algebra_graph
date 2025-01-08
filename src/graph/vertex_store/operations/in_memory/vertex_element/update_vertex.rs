use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::graph::vertex_store::operations::vertex_element::UpdateVertex;
use crate::graph::vertex_store::vertex_store::VertexStore;

impl<T> UpdateVertex<T> for VertexStore
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    fn update_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        // TODO: the same vertex vector is retrieved multiple, once for checking, once for updating. Loading the vector only once should improve performance.
        self.try_is_valid_vertex_element(vertex_type_index, vertex_index)?;
        self.update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn update_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }
}
