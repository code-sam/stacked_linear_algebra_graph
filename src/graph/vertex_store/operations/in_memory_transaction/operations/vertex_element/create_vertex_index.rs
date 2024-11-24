use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::AssignedIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    InMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    GetVertexStoreStateReverters,
};
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex;

impl<'s> CreateVertexIndex for InMemoryVertexStoreTransaction<'s> {
    fn new_public_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self.vertex_store_mut_ref().new_public_vertex_index()?;
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_new_public_index_to_revert(&vertex_index)?;
        Ok(vertex_index)
    }

    fn new_private_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self.vertex_store_mut_ref().new_private_vertex_index()?;
        self.vertex_store_state_restorer_mut_ref()
            .element_indexer_state_restorer_mut_ref()
            .register_new_private_index_to_revert(&vertex_index)?;
        Ok(vertex_index)
    }
}
