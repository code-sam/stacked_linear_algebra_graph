use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{
    GetEdgeStore, InMemoryEdgeStoreTransaction,
};
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing;
use crate::graph::indexing::traits::try_index_validity;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};

impl<'s> Indexing for InMemoryEdgeStoreTransaction<'s> {
    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref()
            .is_valid_edge_type_index(edge_type_index)
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_edge_type_index_validity(edge_type_index)
    }

    fn try_optional_edge_type_index_validity(
        &self,
        edge_type_index: Option<&EdgeTypeIndex>,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .try_optional_edge_type_index_validity(edge_type_index)
    }
}
