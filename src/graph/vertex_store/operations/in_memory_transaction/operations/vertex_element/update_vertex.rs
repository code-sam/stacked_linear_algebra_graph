use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::RegisterVertexValueToRestore;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::UpdateVertex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;

impl<'s, T> UpdateVertex<T> for InMemoryVertexStoreTransaction<'s>
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    fn update_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_type_index_validity(vertex_type_index)?;
        self.vertex_store_ref()
            .try_vertex_index_validity(vertex_index)?;

        self.update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn update_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .vertex_vector_ref_unchecked(vertex_type_index);
        self.vertex_store_state_restorer
            .register_vertex_value_to_restore(vertex_vector, vertex_type_index, vertex_index)?;
        self.vertex_store_mut_ref()
            .update_vertex_unchecked(vertex_type_index, vertex_index, value)
    }
}
