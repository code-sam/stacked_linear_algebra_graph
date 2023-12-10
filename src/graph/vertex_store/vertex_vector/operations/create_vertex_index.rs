use crate::{
    error::GraphComputingError,
    graph::{
        indexer::{AssignedIndex, GetAssignedIndexData, IndexerTrait},
        vertex_store::{VertexStore, VertexStoreTrait},
    },
};

pub(crate) trait CreateVertexIndex {
    fn new_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

impl CreateVertexIndex for VertexStore {
    fn new_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let assigned_index = self.element_indexer_mut_ref().new_index()?;
        match assigned_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;
            }
            None => {}
        }
        Ok(assigned_index)
    }
}
