use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::operators::operator_traits::set::SetVertexValue;
use crate::operators::transaction::in_memory::InMemoryGraphTransaction;

impl<'g, T> SetVertexValue<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType
        + SetSparseVectorElementTyped<T>
        + Default
        + GetSparseVectorElementValueTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn set_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction
            .set_vertex(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
