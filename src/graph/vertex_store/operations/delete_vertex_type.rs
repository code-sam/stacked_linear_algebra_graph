use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::IndexerTrait,
        vertex_store::{vertex_store::VertexStoreTrait, VertexStore},
    },
};

pub(crate) trait DeleteVertexType {
    fn delete_vertex_type(&mut self, index: VertexTypeIndex) -> Result<(), GraphComputingError>;

    fn delete_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

// TODO: deleting a vertex type, may result in vertices without any value, is this desired?
impl DeleteVertexType for VertexStore {
    fn delete_vertex_type(&mut self, index: VertexTypeIndex) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref().free_index(index)
    }

    fn delete_vertex_type_unchecked(
        &mut self,
        index: VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_mut_ref()
            .free_index_unchecked(index)
    }
}
