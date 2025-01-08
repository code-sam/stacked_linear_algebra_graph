use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{operations::FreeIndex, GetVertexTypeIndex},
        vertex_store::{GetVertexTypeIndexer, VertexStore},
    },
};

pub(crate) trait DeleteVertexType<'a> {
    fn delete_vertex_type(
        &'a mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_type_unchecked(
        &'a mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) fn delete_vertex_type(
    vertex_store: &mut VertexStore,
    index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    vertex_store
        .vertex_type_indexer_mut_ref()
        .free_valid_index(index.index())
}

pub(crate) fn delete_vertex_type_unchecked(
    vertex_store: &mut VertexStore,
    index: &impl GetVertexTypeIndex,
) -> Result<(), GraphComputingError> {
    vertex_store
        .vertex_type_indexer_mut_ref()
        .free_index_unchecked(index.index())
}
