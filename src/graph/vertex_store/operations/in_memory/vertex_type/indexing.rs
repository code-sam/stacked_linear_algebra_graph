use crate::error::GraphComputingError;
use crate::graph::indexing::{operations::CheckIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::{GetVertexTypeIndexer, VertexStore};

impl CheckVertexTypeIndex for VertexStore {
    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .is_valid_index(vertex_type_index.index())
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index.index())
    }

    fn try_optional_vertex_type_index_validity(
        &self,
        vertex_type_index: Option<&crate::graph::indexing::VertexTypeIndex>,
    ) -> Result<(), GraphComputingError> {
        match vertex_type_index {
            Some(index) => self.try_vertex_type_index_validity(index),
            None => Ok(()),
        }
    }
}
