use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::vertex_store::VertexStore;

impl<T> SetVertex<T> for VertexStore
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    fn set_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(vertex_type_index)?;
        self.try_vertex_index_validity(vertex_index)?;
        self.set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn set_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_unchecked(vertex_type_index)?;
        T::set_graphblas_vector_value(vertex_vector, vertex_index.index(), value)?;
        Ok(())
    }
}
