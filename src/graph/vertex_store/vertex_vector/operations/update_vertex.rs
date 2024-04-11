use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::try_is_element;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::graph::VertexIndex;
use crate::graph::graph::VertexTypeIndex;
use crate::graph::indexer::CheckIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::VertexVector;

pub(crate) trait UpdateVertex<T: ValueType> {
    fn update_vertex(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

impl<T> UpdateVertex<T> for VertexStore
where
    T: ValueType + Copy + SetVectorElementTyped<T>,
{
    fn update_vertex(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_index_validity(vertex_index)?;
        let vertex_vector: &mut VertexVector = self.vertex_vector_mut_ref(vertex_type_index)?;
        try_is_element(vertex_vector, *vertex_index)?;

        T::set_value(vertex_vector, vertex_index, value)?;
        Ok(())
    }

    fn update_vertex_unchecked(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_unchecked(vertex_type_index);
        try_is_element(vertex_vector, *vertex_index)?;
        T::set_value(vertex_vector, vertex_index, value)?;
        Ok(())
    }
}
