use crate::{
    error::GraphComputingError,
    graph::{
        index::VertexTypeIndex,
        indexing::operations::FreeIndex,
        vertex_store::{GetVertexTypeIndexer, VertexStore},
    },
};

pub(crate) trait DeleteVertexType {
    fn delete_public_vertex_type(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_public_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_type(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_private_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

// TODO: deleting a vertex type, may result in vertices without any value, is this desired?
impl DeleteVertexType for VertexStore {
    fn delete_public_vertex_type(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref().free_public_index(index)
    }

    fn delete_public_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_public_index_unchecked(index)
    }

    fn delete_private_vertex_type(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref().free_private_index(index)
    }

    fn delete_private_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_private_index_unchecked(index)
    }
}
