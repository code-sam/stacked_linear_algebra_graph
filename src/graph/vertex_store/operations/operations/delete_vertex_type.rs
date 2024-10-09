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
