use crate::{
    error::GraphComputingError,
    graph::{
        indexer::{AssignedIndex, AssignedIndexTrait, IndexerTrait},
        vertex::VertexKeyRef,
        vertex_store::{VertexStore, VertexStoreTrait},
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
                self.resize_vertex_vectors(new_capacity)?;
            }
            None => {}
        }
        Ok(assigned_index)
    }
}
