use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{
            operations::{GeneratePrivateIndex, GeneratePublicIndex},
            AssignedIndex, GetAssignedIndexData,
        },
        vertex_store::{
            operations::resize_vertex_vectors::ResizeVertexVectors, GetVertexElementIndexer,
            VertexStore,
        },
    },
};

pub(crate) trait CreateVertexIndex {
    fn new_public_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
    // fn new_private_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}

impl CreateVertexIndex for VertexStore {
    fn new_public_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
        let assigned_index = self.element_indexer_mut_ref().new_public_index()?;
        self.update_vertex_vectors_size(&assigned_index)?;
        Ok(assigned_index)
    }

    // fn new_private_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError> {
    //     let assigned_index = self.element_indexer_mut_ref().new_private_index()?;
    //     self.update_vertex_vectors_size(&assigned_index)?;
    //     Ok(assigned_index)
    // }
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
