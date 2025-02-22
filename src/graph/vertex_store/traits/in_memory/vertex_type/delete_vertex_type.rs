use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::traits::vertex_type::{
    delete_vertex_type, delete_vertex_type_unchecked, DeleteVertexType,
};
use crate::{error::GraphComputingError, graph::vertex_store::VertexStore};

// TODO: deleting a vertex type, may result in vertices without any value, is this desired?
impl DeleteVertexType for VertexStore {
    fn delete_vertex_type(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_vertex_type(self, index)
    }

    fn delete_vertex_type_unchecked(
        &mut self,
        index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        delete_vertex_type_unchecked(self, index)
    }
}
