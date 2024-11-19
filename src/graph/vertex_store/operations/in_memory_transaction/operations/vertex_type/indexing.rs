use crate::error::GraphComputingError;
use crate::graph::indexing::{operations::CheckIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore,
};
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::{GetVertexElementIndexer, GetVertexTypeIndexer};

impl<'s> CheckVertexTypeIndex for AtomicInMemoryVertexStoreTransaction<'s> {
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

    fn is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_public_index(vertex_type_index.index())
    }

    fn try_is_valid_public_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())
    }

    fn is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_private_index(vertex_type_index.index())
    }

    fn try_is_valid_private_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())
    }
}

