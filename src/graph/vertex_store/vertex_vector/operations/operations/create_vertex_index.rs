use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{operations::GeneratePublicIndex, AssignedIndex, GetAssignedIndexData},
        vertex_store::{operations::ResizeVertexVectors, GetVertexElementIndexer, VertexStore},
    },
};

pub(crate) trait CreateVertexIndex {
    fn new_public_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
    // fn new_private_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}
