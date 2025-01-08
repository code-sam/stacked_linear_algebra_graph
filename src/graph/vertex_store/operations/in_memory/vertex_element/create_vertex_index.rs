use crate::error::GraphComputingError;
use crate::graph::indexing::operations::GenerateIndex;
use crate::graph::indexing::{AssignedIndex, GetAssignedIndexData};
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex;
use crate::graph::vertex_store::operations::vertex_type::ResizeVertexVectors;
use crate::graph::vertex_store::{GetVertexElementIndexer, VertexStore};

impl CreateVertexIndex for VertexStore {
    fn new_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let assigned_index = self.element_indexer_mut_ref().new_index()?;
        self.update_vertex_vectors_size(&assigned_index)?;
        Ok(assigned_index)
    }
}

impl VertexStore {
    fn update_vertex_vectors_size(
        &mut self,
        assigned_index: &AssignedIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(match assigned_index.new_index_capacity() {
            Some(new_capacity) => {
                self.resize_vertex_vectors(new_capacity)?;
            }
            None => {}
        })
    }
}
