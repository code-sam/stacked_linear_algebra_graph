use crate::error::GraphComputingError;
use crate::graph::edge_store::traits::traits::edge_type::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetAssignedIndexData, VertexIndex};
use crate::graph::vertex_store::traits::vertex_element::CreateVertexIndex as CreateVertexIndexInVertexStore;
use crate::graph_operators::operator_traits::new::NewVertexIndex;

impl NewVertexIndex for Graph {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError> {
        new_vertex_index(&mut self.public_vertex_store, &mut self.public_edge_store)
    }
}

pub(crate) fn new_vertex_index(
    vertex_store: &mut impl CreateVertexIndexInVertexStore,
    edge_store: &mut impl ResizeAdjacencyMatrices,
) -> Result<VertexIndex, GraphComputingError> {
    let assigned_vertex_index = vertex_store.new_vertex_index()?;

    match assigned_vertex_index.new_index_capacity() {
        Some(new_vertex_capacity) => edge_store.resize_adjacency_matrices(new_vertex_capacity)?,
        None => {}
    }

    Ok(VertexIndex::new(assigned_vertex_index.index()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge_store::GetAdjacencyMatrices;
    use crate::graph::graph::{GetEdgeStore, GetVertexStore};
    use crate::graph::indexing::GetIndexCapacity;
    use crate::graph::vertex_store::GetVertexElementIndexer;

    #[test]
    fn add_new_vertex() {
        let mut graph = Graph::with_initial_capacity(1, 1, 1).unwrap();

        for _ in 0..50 {
            graph.new_vertex_index().unwrap();
        }

        assert_eq!(
            graph.edge_store_ref().adjacency_matrix_size(),
            graph
                .vertex_store_ref()
                .element_indexer_ref()
                .capacity()
                .unwrap()
        )
    }
}
