use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::GetIndexerStateReverters;
use crate::graph::vertex_store::VertexVector;
use crate::operators::transaction::RestoreState;
use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::graph::indexing::Indexer;
use crate::graph::indexing::operations::in_memory_transaction::transaction::indexer_state_restorer::IndexerStateRestorer;

impl RestoreState<Indexer> for IndexerStateRestorer {
    fn restore(self, instance_to_restore: &mut Indexer) -> Result<(), GraphComputingError> {
        self.indices_available_for_reuse_restorer_ref()
            .to_owned()
            .restore(instance_to_restore.indices_available_for_reuse_mut_ref())?;

        self.mask_with_valid_indices_restorer_ref()
            .to_owned()
            .restore(instance_to_restore.mask_with_valid_indices_mut_ref())?;
        Ok(())
    }

    fn with_reset_state_to_restore(&self) -> Self {
        let index_capacity_to_restore = self.index_capacity_to_restore();
        let indices_available_for_reuse_restorer = self
            .indices_available_for_reuse_restorer_ref()
            .with_reset_state_to_restore();

        let mask_with_valid_indices_restorer =
            RestoreState::<VertexVector>::with_reset_state_to_restore(
                self.mask_with_valid_indices_restorer_ref(),
            );

        IndexerStateRestorer::new(
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
        )
    }
}
