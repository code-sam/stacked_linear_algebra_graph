use crate::error::GraphComputingError;
use crate::graph::graph::Graph;
use crate::graph::vertex::{Vertex, VertexKey, VertexValue, VertexValueType};
use crate::graph::edge::{DirectedEdge, EdgeKey, EdgeType};

pub trait ConnectedVertices {
    pub fn vertices_connected_to_vertex(&self, vertex: VertexKey) -> Result(Vec<Vertex>, GraphComputingError);
};

impl ConnectedVertices for Graph {
    pub fn vertices_connected_to_vertex(&self, vertex: VertexKey, ) -> Result(Vec<Vertex>, GraphComputingError) {

    }
}

