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

impl CreateVertexIndex for Graph {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError> {
        let assigned_index = self.vertex_store_mut_ref().new_public_vertex_index()?;
        match assigned_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.edge_store_mut_ref()
                    .resize_adjacency_matrices(new_vertex_capacity)?;
            }
            None => {}
        }
        return Ok(VertexIndex::new(*assigned_index.index_ref()));
    }
}

// pub trait CreatePrivateVertexIndex {
//     fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError>;
// }

// impl CreatePrivateVertexIndex for Graph {
//     fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError> {
//         let assigned_index = self.vertex_store_mut_ref().new_public_vertex_index()?;
//         match assigned_index.new_index_capacity() {
//             Some(new_vertex_capacity) => {
//                 self.edge_store_mut_ref()
//                     .resize_adjacency_matrices(new_vertex_capacity)?;
//             }
//             None => {}
//         }
//         return Ok(*assigned_index.index_ref());
//     }
// }
