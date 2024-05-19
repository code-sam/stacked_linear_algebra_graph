use crate::graph::edge_store::operations::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::{GetEdgeStore, GetVertexStore};
use crate::graph::indexing::VertexIndex;
use crate::{
    error::GraphComputingError,
    graph::indexing::GetAssignedIndexData,
    graph::{graph::Graph, vertex_store::CreateVertexIndex as AddVertexKeyToVertexStore},
};

pub trait CreateVertexIndex {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError>;
}
