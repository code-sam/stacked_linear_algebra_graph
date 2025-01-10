use crate::error::GraphComputingError;
use crate::graph::indexing::AssignedIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    GetVertexStore, GetVertexStoreStateRestorer,
    InMemoryVertexStoreTransaction, RegisterNewVertexIndexToRevert,
};
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex;

impl<'s> CreateVertexIndex for InMemoryVertexStoreTransaction<'s> {
    fn new_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self.vertex_store_mut_ref().new_vertex_index()?;
        self.vertex_store_state_restorer_mut_ref().register_new_vertex_index_to_revert(&vertex_index)?;
        Ok(vertex_index)
    }
}
