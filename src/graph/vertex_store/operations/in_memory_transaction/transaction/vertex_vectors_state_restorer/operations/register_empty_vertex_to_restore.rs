use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::graph::indexing::{ElementIndex, GetVertexIndexIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};
use crate::graph::{indexing::{VertexIndex, VertexTypeIndex}, value_type::ValueType};
use crate::operators::in_memory_transaction::transaction::{CreateSparseVectorStateReverter, RegisterSparseVectorChangeToRevert, SparseVectorStateReverter};

pub(crate) trait RegisterEmptyVertexToRestore<'a, T: ValueType> {
    fn register_empty_vertex_to_restore(
        &'a mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: ElementIndex,
    );
}

impl<'a, T> RegisterEmptyVertexToRestore<'a, T> for VertexVectorsStateRestorer
where
    T: 'a
        + ValueType
        + Default
        + GetSparseVectorElementValueTyped<T>
        + SetSparseVectorElementTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<'a, T>
        + CreateSparseVectorStateReverter<T>,
{
    fn register_empty_vertex_to_restore(
        &'a mut self,
        vertex_type_index: VertexTypeIndex,
        vertex_index: ElementIndex,
    ) {
        let vertex_vector_state_reverter: &mut SparseVectorStateReverter<T> =
            self.vertex_vector_state_reverter_mut_ref(vertex_type_index);
        vertex_vector_state_reverter.register_empty_element_to_restore(vertex_index)
    }
}
