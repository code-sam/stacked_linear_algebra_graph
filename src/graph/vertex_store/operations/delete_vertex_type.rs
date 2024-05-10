use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{operations::FreeIndex, GetVertexTypeIndex},
        vertex_store::{GetVertexTypeIndexer, VertexStore},
    },
};

pub(crate) trait DeleteVertexType {
    fn delete_public_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_public_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

// TODO: deleting a vertex type, may result in vertices without any value, is this desired?
impl DeleteVertexType for VertexStore {
    fn delete_public_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_public_index(*index.index_ref())
    }

    fn delete_public_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_public_index_unchecked(*index.index_ref())
    }

    fn delete_private_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_private_index(*index.index_ref())
    }

    fn delete_private_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_private_index_unchecked(*index.index_ref())
    }
}
