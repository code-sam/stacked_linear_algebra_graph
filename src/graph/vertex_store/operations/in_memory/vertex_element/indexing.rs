use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::vertex_store::{GetVertexElementIndexer, VertexStore};

impl CheckVertexIndex for VertexStore {
    fn is_valid_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_index(vertex_index.index())
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())
    }

    fn is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_public_index(vertex_index.index())
    }

    fn try_is_valid_public_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_is_valid_public_index(vertex_index.index())
    }

    fn is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.element_indexer_ref()
            .is_valid_private_index(vertex_index.index())
    }

    fn try_is_valid_private_vertex_index(
        &self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_is_valid_private_index(vertex_index.index())
    }
}
