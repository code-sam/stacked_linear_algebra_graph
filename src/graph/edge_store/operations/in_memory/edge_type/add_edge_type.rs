use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
    CreateWeightedAdjacencyMatrixWithCachedAttributes, WeightedAdjacencyMatrixWithCachedAttributes,
};

use crate::graph::edge_store::operations::operations::edge_type::add_edge_type::{
    AddPrivateEdgeType, AddPublicEdgeType,
};
use crate::graph::edge_store::{GetAdjacencyMatrices, GetEdgeTypeIndicer};
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::operations::{GeneratePrivateIndex, GeneratePublicIndex};
use crate::graph::indexing::{AssignedIndex, EdgeTypeIndex, GetAssignedIndexData};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::{error::GraphComputingError, graph::edge_store::EdgeStore};

impl<T: ValueType + GetValueTypeIdentifier> AddPublicEdgeType<T> for EdgeStore {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_type_index = self.edge_type_indexer_mut_ref().new_public_index()?;
        self.add_edge_type_at_assigned_index::<T>(new_type_index)
    }
}

impl<T: ValueType + GetValueTypeIdentifier> AddPrivateEdgeType<T> for EdgeStore {
    fn apply(&mut self) -> Result<EdgeTypeIndex, GraphComputingError> {
        let new_type_index = self.edge_type_indexer_mut_ref().new_private_index()?;
        self.add_edge_type_at_assigned_index::<T>(new_type_index)
    }
}

impl EdgeStore {
    fn add_edge_type_at_assigned_index<T: ValueType + GetValueTypeIdentifier>(
        &mut self,
        edge_type_index: AssignedIndex,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        if let Some(new_capacity) = edge_type_index.new_index_capacity() {
            let current_capacity = self.adjacency_matrices_ref().len();
            self.adjacency_matrices_mut()
                .reserve(new_capacity - current_capacity);
        }
        let new_adjacency_matrix =
            <WeightedAdjacencyMatrixWithCachedAttributes as CreateWeightedAdjacencyMatrixWithCachedAttributes<T>>::new(
                self.graphblas_context(),
                self.adjacency_matrix_size(),
            )?;
        if *edge_type_index.index_ref() >= self.adjacency_matrices_ref().len() {
            self.adjacency_matrices_mut().push(new_adjacency_matrix);
        } else {
            self.adjacency_matrices_mut_ref()[*edge_type_index.index_ref()] = new_adjacency_matrix;
        }
        Ok(EdgeTypeIndex::new(edge_type_index.index()))
    }
}
