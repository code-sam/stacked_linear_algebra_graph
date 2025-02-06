use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::UpdateVertex;
use crate::graph::vertex_store::VertexStore;
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::update::UpdateVertexValue;

impl<'g, T> UpdateVertexValue<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType + Default + SetSparseVectorElementTyped<T>,
    VertexStore: UpdateVertex<T>,
{
    fn update_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction
            .update_vertex(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
