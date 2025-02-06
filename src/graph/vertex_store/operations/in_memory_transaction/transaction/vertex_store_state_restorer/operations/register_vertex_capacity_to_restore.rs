use crate::graph::vertex_store::operations::in_memory_transaction::transaction::VertexStoreStateRestorer;
use crate::graph::indexing::ElementCount;
use crate::error::GraphComputingError;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::RegisterVertexVectorCapacityToRestore;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::GetVertexStoreStateReverters;

pub(crate) trait RegisterVertexCapacityToRestore<'t> {
    fn register_vertex_capacity_to_restore(
        &mut self,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterVertexCapacityToRestore<'t> for VertexStoreStateRestorer {
    fn register_vertex_capacity_to_restore(
        &mut self,
        vertex_capacity_to_restore: &ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.vertex_vectors_state_restorer_mut_ref()
            .register_vertex_vector_capacity_to_restore(vertex_capacity_to_restore);
        Ok(())
    }
}
