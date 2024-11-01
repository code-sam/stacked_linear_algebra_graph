use crate::graph::indexing::{operations::FreeIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::{
    delete_private_vertex_type, delete_public_vertex_type, delete_public_vertex_type_unchecked,
};
use crate::{
    error::GraphComputingError,
    graph::vertex_store::{operations::DeleteVertexType, GetVertexTypeIndexer, VertexStore},
};

// TODO: deleting a vertex type, may result in vertices without any value, is this desired?
impl<'store> DeleteVertexType<'store> for VertexStore {
    fn delete_public_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_public_vertex_type(self, index)
    }

    fn delete_public_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_public_vertex_type_unchecked(self, index)
    }

    fn delete_private_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_private_vertex_type(self, index)
    }

    fn delete_private_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_public_vertex_type_unchecked(self, index)
    }
}
