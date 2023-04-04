use crate::{graph::{edge::{EdgeTypeIndex, EdgeTypeKeyRef}, edge_store::{WeightedAdjacencyMatrix, EdgeStore, EdgeStoreTrait}}, error::{GraphComputingError, LogicError, LogicErrorType}};
use crate::graph::indexer::IndexerTrait;
use crate::graph::indexer::Indexer as EdgeTypeIndexer;

pub(crate) trait GetAdjacencyMatrix {
    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix;

    fn adjacency_matrix_ref_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn adjacency_matrix_mut_ref_for_key(
        &mut self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

}

impl GetAdjacencyMatrix for EdgeStore {
    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix {
        &self.adjacency_matrices_ref()[*edge_type_index]
    }

    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix {
        &mut self.adjacency_matrices_mut_ref()[*edge_type_index]
    }

    fn adjacency_matrix_ref_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        Ok(&self.adjacency_matrices_ref()[*self.edge_type_indexer_ref().try_index_for_key(edge_type_key)?])
    }

    fn adjacency_matrix_mut_ref_for_key(
        &mut self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        Ok(&mut self.adjacency_matrices_mut_ref()
            [*self.edge_type_indexer_ref().try_index_for_key(edge_type_key)?])
    }

    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        match self.adjacency_matrices_ref().get(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }

    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError> {
        match self.adjacency_matrices_mut_ref().get_mut(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }
}