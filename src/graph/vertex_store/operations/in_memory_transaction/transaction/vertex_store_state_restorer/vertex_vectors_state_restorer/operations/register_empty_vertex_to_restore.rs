use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::graph::indexing::{ElementIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::RegisterEmptyVertexToRestore;
use crate::graph::value_type::ValueType;
use crate::transaction::in_memory::{RegisterSparseVectorChangeToRevert, SparseVectorStateReverter};

impl<T> RegisterEmptyVertexToRestore<T> for VertexVectorsStateRestorer
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
        let vertex_vector_state_reverter: &mut SparseVectorStateReverter<T> =
            self.vertex_vector_state_reverter_mut_ref(vertex_type_index);
        vertex_vector_state_reverter.register_empty_element_to_restore(vertex_index)
    }
}
