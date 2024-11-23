use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};
use graphblas_sparse_linear_algebra::index::ElementCount;

use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory_transaction::transaction::{CreateSparseVectorStateReverter, SparseVectorStateReverter};

pub(crate) trait RegisterVertexVectorCapacityToRestore<T: ValueType> {
    fn register_vertex_vector_capacity_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_capacity: &ElementCount,
    );
}

impl<T> RegisterVertexVectorCapacityToRestore<T> for VertexVectorsStateRestorer
where
    T: ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>
        + CreateSparseVectorStateReverter<T>,
{
    fn register_vertex_vector_capacity_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        _vertex_capacity: &ElementCount,
    ) {
        // REVIEW: this methodology may not be self-documenting and may hint at a need for refactoring
        // Retrieving a vertex_vector_state_reverter_mut_ref will instantiate a new reverter to restore the capacity of the vertex vector at the start of the transaction.
        let _vertex_vector_state_reverter: &mut SparseVectorStateReverter<T> =
            self.vertex_vector_state_reverter_mut_ref(vertex_type_index);
    }
}
