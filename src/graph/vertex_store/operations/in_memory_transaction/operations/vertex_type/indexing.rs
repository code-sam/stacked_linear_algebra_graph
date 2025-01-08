use crate::error::GraphComputingError;
use crate::graph::indexing::{operations::CheckIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction,
};
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::GetVertexTypeIndexer;

impl<'s> CheckVertexTypeIndex for InMemoryVertexStoreTransaction<'s> {
    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_index(vertex_type_index.index())
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index.index())
    }
}
