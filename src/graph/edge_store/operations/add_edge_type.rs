use crate::{graph::{edge::{EdgeTypeKeyRef, EdgeTypeIndex}, edge_store::{EdgeStore, WeightedAdjacencyMatrix}, indexer::{IndexerTrait, NewIndexTrait}}, error::GraphComputingError};
use crate::graph::edge_store::edge_store::EdgeStoreTrait;

pub(crate) trait AddEdgeType {
    fn add_new_edge_type(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl AddEdgeType for EdgeStore {
    fn add_new_edge_type(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_type_index = self.edge_type_indexer_mut_ref().add_new_key(key)?;
        if let Some(new_capacity) = new_type_index.new_index_capacity() {
            let current_capacity = self.adjacency_matrices_ref().len();
            self.adjacency_matrices_mut()
                .reserve(new_capacity - current_capacity);
        }
        let new_adjacency_matrix = WeightedAdjacencyMatrix::new(
            self.graphblas_context_ref(),
            key,
            self.adjacency_matrix_size_ref(),
        )?;
        if *new_type_index.index_ref() >= self.adjacency_matrices_ref().len() {
            self.adjacency_matrices_mut()
                .push(new_adjacency_matrix);
        } else {
            self.adjacency_matrices_mut_ref()[*new_type_index.index_ref()] =
                new_adjacency_matrix;
        }
        Ok(*new_type_index.index_ref())
    }
}
