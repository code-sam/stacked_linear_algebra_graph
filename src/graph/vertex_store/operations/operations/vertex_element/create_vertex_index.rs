use crate::{error::GraphComputingError, graph::indexing::AssignedIndex};

pub(crate) trait CreateVertexIndex {
    fn new_public_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
    fn new_private_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}
