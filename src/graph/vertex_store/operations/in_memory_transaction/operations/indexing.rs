use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{operations::CheckIndex, GetVertexIndexIndex, GetVertexTypeIndex},
        vertex_store::{
            operations::{
                in_memory_transaction::transaction::{
                    AtomicInMemoryVertexStoreTransaction, GetVertexStore,
                },
                CheckVertexIndex, CheckVertexTypeIndex,
            },
            GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore,
        },
    },
};

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

impl<'s> CheckVertexIndex for AtomicInMemoryVertexStoreTransaction<'s> {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_index(vertex_index.index())
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_index_validity(vertex_index.index())
    }

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_public_index(vertex_index.index())
    }

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_is_valid_public_index(vertex_index.index())
    }

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_private_index(vertex_index.index())
    }

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_is_valid_private_index(vertex_index.index())
    }
}
