use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::{EdgeStoreStateRestorer, GetEdgeStoreStateReverters};
use crate::graph::indexing::operations::in_memory_transaction::RegisterNewIndexToRevert;
use crate::graph::indexing::GetAssignedIndexData;

pub(crate) trait RegisterNewEdgeTypeToRevert {
    fn register_new_public_edge_type_to_revert(
        &mut self,
        edge_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;

    fn register_new_private_edge_type_to_revert(
        &mut self,
        edge_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError>;
}

impl RegisterNewEdgeTypeToRevert for EdgeStoreStateRestorer {
    fn register_new_public_edge_type_to_revert(
        &mut self,
        edge_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_state_restorer_mut_ref()
            .register_new_public_index_to_revert(edge_type_index)
    }

    fn register_new_private_edge_type_to_revert(
        &mut self,
        edge_type_index: &impl GetAssignedIndexData,
    ) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_state_restorer_mut_ref()
            .register_new_private_index_to_revert(edge_type_index)
    }
}
