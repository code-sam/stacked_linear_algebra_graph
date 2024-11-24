use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    InMemoryVertexStoreTransaction, GetVertexStore,
};
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;

impl<'s> CheckVertexIndex for InMemoryVertexStoreTransaction<'s> {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref().is_valid_vertex_index(vertex_index)
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_vertex_index_validity(vertex_index)
    }

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .is_valid_public_vertex_index(vertex_index)
    }

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_is_valid_public_vertex_index(vertex_index)
    }

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .is_valid_private_vertex_index(vertex_index)
    }

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_is_valid_private_vertex_index(vertex_index)
    }
}
