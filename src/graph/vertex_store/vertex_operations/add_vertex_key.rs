use crate::{
    error::GraphComputingError,
    graph::{
        indexer::{AssignedIndex, AssignedIndexTrait, IndexerTrait},
        vertex::vertex::VertexKeyRef,
        vertex_store::{VertexMatrixTrait, VertexStore, VertexStoreTrait},
    },
};

pub(crate) trait AddVertexKey {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<AssignedIndex, GraphComputingError>;
}

impl AddVertexKey for VertexStore {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let assigned_index = self.element_indexer_mut_ref().add_new_key(vertex_key)?;
        match assigned_index.new_index_capacity() {
            Some(new_capacity) => {
                self.vertex_matrix_mut_ref()
                    .set_vertex_capacity(new_capacity)?;
            }
            None => {}
        }
        Ok(assigned_index)
    }
}
