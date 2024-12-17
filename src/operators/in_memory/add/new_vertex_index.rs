use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_type::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::Graph;
use crate::graph::graph::{GetEdgeStore, GetVertexStore};
use crate::graph::indexing::{GetAssignedIndexData, VertexIndex};
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex as CreateVertexIndexInVertexStore;
use crate::operators::operators::add::CreateVertexIndex;

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
