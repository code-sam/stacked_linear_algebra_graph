use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory_transaction::transaction::RegisterSparseVectorChangeToRevert;

pub(crate) trait RegisterTypedVertexValueToRestore<T: ValueType> {
    fn register_vertex_value_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        vertex_value: T,
    );
}

impl<T> RegisterTypedVertexValueToRestore<T> for VertexVectorsStateRestorer
where
    T: ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn register_vertex_value_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        vertex_value: T,
    ) {
        self.vertex_vector_state_reverter_mut_ref(vertex_type_index)
            .register_element_value_to_restore(vertex_index.index(), vertex_value)
    }
}
