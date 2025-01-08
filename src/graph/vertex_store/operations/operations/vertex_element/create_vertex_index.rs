use crate::{error::GraphComputingError, graph::indexing::AssignedIndex};

pub(crate) trait CreateVertexIndex {
    fn new_vertex_index(&mut self) -> Result<AssignedIndex, GraphComputingError>;
}
