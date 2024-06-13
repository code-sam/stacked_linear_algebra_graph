use crate::graph::indexing::indexer::indexer::GetQueueWithIndicesForReuse;
use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::indexer::GetIndicesAvailableForReuse;
use crate::operators::transaction::RestoreState;
use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, Index, Indexer},
    operators::in_memory_transaction::transaction::{
        QueueStateReverter, SparseVectorStateReverter,
    },
};

pub(super) struct IndexerStateRestorer {
    index_capacity_to_restore: ElementCount,

    indices_available_for_reuse_restorer: QueueStateReverter<Index>,

    mask_with_valid_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_private_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_valid_private_indices_restorer: SparseVectorStateReverter<bool>,
    mask_with_valid_public_indices_restorer: SparseVectorStateReverter<bool>,
}

impl RestoreState<Indexer> for IndexerStateRestorer {
    fn restore(self, instance_to_restore: &mut Indexer) -> Result<(), GraphComputingError> {
        self.indices_available_for_reuse_restorer
            .restore(instance_to_restore.indices_available_for_reuse_mut_ref())?;

        self.mask_with_valid_indices_restorer
            .restore(instance_to_restore.mask_with_valid_indices_mut_ref())?;

        self.mask_with_private_indices_restorer
            .restore(instance_to_restore.mask_with_private_indices_mut_ref())?;

        self.mask_with_valid_private_indices_restorer
            .restore(instance_to_restore.mask_with_valid_private_indices_mut_ref())?;

        self.mask_with_valid_public_indices_restorer
            .restore(instance_to_restore.mask_with_valid_public_indices_mut_ref())?;

        Ok(())
    }

    fn with_reset_state_to_restore(&self) -> Self {
        let index_capacity_to_restore = self.index_capacity_to_restore;
        let indices_available_for_reuse_restorer = self
            .indices_available_for_reuse_restorer
            .with_reset_state_to_restore();

        let mask_with_valid_indices_restorer = self
            .mask_with_valid_indices_restorer
            .with_reset_state_to_restore();
        let mask_with_private_indices_restorer = self
            .mask_with_private_indices_restorer
            .with_reset_state_to_restore();
        let mask_with_valid_private_indices_restorer = self
            .mask_with_valid_private_indices_restorer
            .with_reset_state_to_restore();
        let mask_with_valid_public_indices_restorer = self
            .mask_with_valid_public_indices_restorer
            .with_reset_state_to_restore();

        Self {
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
            mask_with_private_indices_restorer,
            mask_with_valid_private_indices_restorer,
            mask_with_valid_public_indices_restorer,
        }
    }
}

impl IndexerStateRestorer {
    pub(super) fn new(
        index_capacity_to_restore: ElementCount,

        indices_available_for_reuse_restorer: QueueStateReverter<Index>,

        mask_with_valid_indices_restorer: SparseVectorStateReverter<bool>,
        mask_with_private_indices_restorer: SparseVectorStateReverter<bool>,
        mask_with_valid_private_indices_restorer: SparseVectorStateReverter<bool>,
        mask_with_valid_public_indices_restorer: SparseVectorStateReverter<bool>,
    ) -> Self {
        Self {
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
            mask_with_private_indices_restorer,
            mask_with_valid_private_indices_restorer,
            mask_with_valid_public_indices_restorer,
        }
    }

    pub(super) fn new_for_indexer(indexer: &Indexer) -> Result<Self, GraphComputingError> {
        let index_capacity_to_restore = indexer.capacity()?;

        let indices_available_for_reuse_restorer =
            QueueStateReverter::with_length_and_capacity_to_restore_from(
                indexer.queue_with_indices_for_reuse_ref(),
            );

        let mask_with_valid_indices_restorer =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(
                indexer.mask_with_valid_indices_ref(),
            )?;
        let mask_with_private_indices_restorer =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(
                indexer.mask_with_private_indices_ref(),
            )?;
        let mask_with_valid_private_indices_restorer =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(
                indexer.mask_with_valid_private_indices_ref(),
            )?;
        let mask_with_valid_public_indices_restorer =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(
                indexer.mask_with_valid_public_indices_ref(),
            )?;

        Ok(Self {
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
            mask_with_private_indices_restorer,
            mask_with_valid_private_indices_restorer,
            mask_with_valid_public_indices_restorer,
        })
    }
}
