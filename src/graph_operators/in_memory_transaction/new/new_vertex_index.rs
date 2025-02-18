use crate::error::GraphComputingError;
use crate::graph::indexing::VertexIndex;
use crate::graph_operators::in_memory::new::new_vertex_index;
use crate::graph_operators::operator_traits::new::NewVertexIndex;
use crate::transaction::in_memory::InMemoryGraphTransaction;

impl<'g> NewVertexIndex for InMemoryGraphTransaction<'g> {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError> {
        new_vertex_index(
            &mut self.vertex_store_transaction,
            &mut self.edge_store_transaction,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge_store::GetAdjacencyMatrices;
    use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};
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
