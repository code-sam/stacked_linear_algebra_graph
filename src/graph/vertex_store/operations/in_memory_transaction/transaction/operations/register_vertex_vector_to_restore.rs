use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_vectors_state_restorer::RegisterVertexVectorToRestore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::vertex_store::vertex_vector::AsSparseVector;
use crate::graph::vertex_store::VertexVector;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::indexing::VertexTypeIndex;
use crate::error::GraphComputingError;
pub(crate) trait RegisterVertexVectorToRestoreTyped<'t> {
    fn register_vertex_vector_to_restore(
        vertex_vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
        vertex_vector: &VertexVector,
        vertex_type_index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_register_vertex_value_to_restore_typed {
    ($value_type:ty) => {
        impl<'t> RegisterVertexVectorToRestoreTyped<'t> for $value_type {
            fn register_vertex_vector_to_restore(
                vertex_store_state_restorer: &'t mut VertexStoreStateRestorer,
                vertex_vector_to_restore: &VertexVector,
                vertex_type_index: VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let sparse_vertex_vector = vertex_vector_to_restore.sparse_vector()?;

                RegisterVertexVectorToRestore::<'t, $value_type>::register_vertex_vector_to_restore(
                    vertex_store_state_restorer.vertex_vectors_state_restorer_mut_ref(),
                    vertex_type_index,
                    sparse_vertex_vector,
                );

                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_register_vertex_value_to_restore_typed);
