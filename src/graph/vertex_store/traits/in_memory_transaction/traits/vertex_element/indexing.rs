use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::{
    GetVertexStore, InMemoryVertexStoreTransaction,
};
use crate::graph::vertex_store::traits::vertex_element::CheckVertexIndex;

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

    fn is_valid_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .is_valid_vertex_element(vertex_type_index, vertex_index)
    }

    fn try_is_valid_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_is_valid_vertex_element(vertex_type_index, vertex_index)
    }

    fn is_empty_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .is_empty_vertex_element(vertex_type_index, vertex_index)
    }

    fn try_is_empty_vertex_element(
        &self,
        vertex_type_index: &impl crate::graph::indexing::GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .try_is_empty_vertex_element(vertex_type_index, vertex_index)
    }
}
