use crate::error::GraphComputingError;
use crate::graph::indexing::VertexIndex;

pub trait NewVertexIndex {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError>;
}
