use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph::value_type::ValueType;
use crate::graph::indexing::{ElementIndex, GetVertexTypeIndex};

pub(crate) trait RegisterEmptyVertexToRestore<T: ValueType> {
    fn register_empty_vertex_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: ElementIndex,
    );
}

impl<T> RegisterEmptyVertexToRestore<T> for VertexStoreStateRestorer
where
    T: ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn register_empty_vertex_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: ElementIndex,
    ) {
        RegisterEmptyVertexToRestore::<T>::register_empty_vertex_to_restore(
            self.vertex_vectors_state_restorer_mut_ref(),
            vertex_type_index,
            vertex_index,
        );
    }
}
