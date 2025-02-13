use crate::error::GraphComputingError;
use crate::graph::indexing::indexer::indexer::GetQueueWithIndicesForReuse;
use crate::graph::indexing::indexer::GetIndexMask;
use crate::graph::indexing::GetIndexCapacity;
use crate::graph::indexing::{ElementCount, Index, Indexer};
use crate::operators::transaction::in_memory::{QueueStateReverter, SparseVectorStateReverter};

#[derive(Debug)]
pub(crate) struct IndexerStateRestorer {
    index_capacity_to_restore: ElementCount,
    indices_available_for_reuse_restorer: QueueStateReverter<Index>,
    mask_with_valid_indices_restorer: SparseVectorStateReverter<bool>,
}

pub(super) trait GetIndexerStateReverters {
    // TODO: should this go into a separate trait?
    fn index_capacity_to_restore(&self) -> ElementCount;
    fn index_capacity_to_restore_mut_ref(&mut self) -> &mut ElementCount;

    fn indices_available_for_reuse_restorer_ref(&self) -> &QueueStateReverter<Index>;
    fn indices_available_for_reuse_restorer_mut_ref(&mut self) -> &mut QueueStateReverter<Index>;

    fn mask_with_valid_indices_restorer_ref(&self) -> &SparseVectorStateReverter<bool>;
    fn mask_with_valid_indices_restorer_mut_ref(&mut self) -> &mut SparseVectorStateReverter<bool>;
}

impl GetIndexerStateReverters for IndexerStateRestorer {
    fn index_capacity_to_restore(&self) -> ElementCount {
        self.index_capacity_to_restore
    }

    fn index_capacity_to_restore_mut_ref(&mut self) -> &mut ElementCount {
        &mut self.index_capacity_to_restore
    }

    fn indices_available_for_reuse_restorer_ref(&self) -> &QueueStateReverter<Index> {
        &self.indices_available_for_reuse_restorer
    }

    fn mask_with_valid_indices_restorer_ref(&self) -> &SparseVectorStateReverter<bool> {
        &self.mask_with_valid_indices_restorer
    }

    fn indices_available_for_reuse_restorer_mut_ref(&mut self) -> &mut QueueStateReverter<Index> {
        &mut self.indices_available_for_reuse_restorer
    }

    fn mask_with_valid_indices_restorer_mut_ref(&mut self) -> &mut SparseVectorStateReverter<bool> {
        &mut self.mask_with_valid_indices_restorer
    }
}

impl IndexerStateRestorer {
    pub(crate) fn new(
        index_capacity_to_restore: ElementCount,
        indices_available_for_reuse_restorer: QueueStateReverter<Index>,
        mask_with_valid_indices_restorer: SparseVectorStateReverter<bool>,
    ) -> Self {
        Self {
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
        }
    }

    pub(crate) fn new_for_indexer(indexer: &Indexer) -> Result<Self, GraphComputingError> {
        let index_capacity_to_restore = indexer.capacity()?;

        let indices_available_for_reuse_restorer =
            QueueStateReverter::with_length_and_capacity_to_restore_from(
                indexer.queue_with_indices_for_reuse_ref(),
            );

        let mask_with_valid_indices_restorer =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(
                indexer.mask_with_valid_indices_ref(),
            )?;

        Ok(Self {
            index_capacity_to_restore,
            indices_available_for_reuse_restorer,
            mask_with_valid_indices_restorer,
        })
    }
}
