use crate::error::GraphComputingError;
use crate::graph::edge_store::{
    traits::traits::edge_type::indexing::Indexing, EdgeStore, GetEdgeTypeIndicer,
};
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::{traits::CheckIndex, GetEdgeTypeIndex};

impl Indexing for EdgeStore {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_type_indexer_ref()
            .is_valid_index(edge_type_index.index())
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_ref()
            .try_index_validity(edge_type_index.index())
    }

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&EdgeTypeIndex>,
    ) -> Result<(), GraphComputingError> {
        match edge_type_index {
            Some(index) => self.try_edge_type_index_validity(index),
            None => Ok(()),
        }
    }
}
