use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetGraphblasSparseMatrix;

use crate::graph::edge_store::edge_store::EdgeStoreTrait;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    CreateWeightedAdjacencyMatrix, WeightedAdjacencyMatrix,
};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::{
    error::GraphComputingError,
    graph::{
        edge::{EdgeTypeIndex, EdgeTypeKeyRef},
        edge_store::EdgeStore,
        indexer::{AssignedIndexTrait, IndexerTrait},
    },
};

pub(crate) trait AddEdgeType<T: ValueType> {
    fn add_new_edge_type(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;

    fn add_new_edge_type_or_return_existing_index(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for EdgeStore {
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
        let new_adjacency_matrix =
            <WeightedAdjacencyMatrix as CreateWeightedAdjacencyMatrix<T>>::new(
                self.graphblas_context_ref(),
                key,
                self.adjacency_matrix_size_ref(),
            )?;
        if *new_type_index.index_ref() >= self.adjacency_matrices_ref().len() {
            self.adjacency_matrices_mut().push(new_adjacency_matrix);
        } else {
            self.adjacency_matrices_mut_ref()[*new_type_index.index_ref()] = new_adjacency_matrix;
        }
        Ok(*new_type_index.index_ref())
    }

    fn add_new_edge_type_or_return_existing_index(
        &mut self,
        key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        // TODO: review if there are checks than can be dropped in the process. This should improve performance.
        match self.edge_type_indexer_mut_ref().index_for_key(key) {
            Some(index) => Ok(*index),
            None => <EdgeStore as AddEdgeType<T>>::add_new_edge_type(self, key),
        }
    }
}
