use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
    CreateWeightedAdjacencyMatrixWithCachedAttributes, WeightedAdjacencyMatrixWithCachedAttributes,
};

use crate::graph::edge_store::weighted_adjacency_matrix::GetGraphblasContext;
use crate::graph::edge_store::{GetAdjacencyMatrices, GetEdgeTypeIndicer};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        edge_store::EdgeStore,
        indexer::{GetAssignedIndexData, IndexerTrait},
    },
};

pub(crate) trait AddEdgeType<T: ValueType> {
    fn add_new_edge_type(&mut self) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for EdgeStore {
    fn add_new_edge_type(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_type_index = self.edge_type_indexer_mut_ref().new_index()?;
        if let Some(new_capacity) = new_type_index.new_index_capacity() {
            let current_capacity = self.adjacency_matrices_ref().len();
            self.adjacency_matrices_mut()
                .reserve(new_capacity - current_capacity);
        }
        let new_adjacency_matrix =
            <WeightedAdjacencyMatrixWithCachedAttributes as CreateWeightedAdjacencyMatrixWithCachedAttributes<T>>::new(
                self.graphblas_context_ref(),
                self.adjacency_matrix_size_ref(),
            )?;
        if *new_type_index.index_ref() >= self.adjacency_matrices_ref().len() {
            self.adjacency_matrices_mut().push(new_adjacency_matrix);
        } else {
            self.adjacency_matrices_mut_ref()[*new_type_index.index_ref()] = new_adjacency_matrix;
        }
        Ok(*new_type_index.index_ref())
    }
}
