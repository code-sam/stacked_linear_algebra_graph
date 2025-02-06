use crate::error::GraphComputingError;
use crate::graph::edge_store::{EdgeStore, GetAdjacencyMatrices, GetEdgeTypeIndicer};
use crate::graph::indexing::operations::in_memory_transaction::IndexerStateRestorer;
use crate::graph::indexing::{GetIndexCapacity, Indexer};
use crate::operators::transaction::RestoreState;

use super::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::AdjacencyMatricesWithCachedAttributesStateRestorer;

#[derive(Debug)]
pub(crate) struct EdgeStoreStateRestorer {
    edge_type_indexer_state_restorer: IndexerStateRestorer,

    adjacency_matrices_state_restorer: AdjacencyMatricesWithCachedAttributesStateRestorer,
}

// TODO: moving the implementation of RestoreState to operations likely requires cloning the restorers.
impl RestoreState<EdgeStore> for EdgeStoreStateRestorer {
    fn restore(self, instance_to_restore: &mut EdgeStore) -> Result<(), GraphComputingError> {
        self.edge_type_indexer_state_restorer
            .restore(instance_to_restore.edge_type_indexer_mut_ref())?;
        // self.element_indexer_state_restorer
        //     .restore(instance_to_restore.element_indexer_mut_ref())?;

        self.adjacency_matrices_state_restorer
            .restore(instance_to_restore.adjacency_matrices_mut())?;

        Ok(())
    }

    fn with_reset_state_to_restore(&self) -> Self {
        let edge_type_indexer_state_restorer = self
            .edge_type_indexer_state_restorer
            .with_reset_state_to_restore();

        let adjacency_matrices_state_restorer = self
            .adjacency_matrices_state_restorer
            .with_reset_state_to_restore();

        EdgeStoreStateRestorer::new(
            edge_type_indexer_state_restorer,
            adjacency_matrices_state_restorer,
        )
    }
}

pub(crate) trait GetEdgeStoreStateReverters {
    fn edge_type_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer;
    fn edge_type_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer;

    fn adjacency_matrices_state_restorer_ref(
        &self,
    ) -> &AdjacencyMatricesWithCachedAttributesStateRestorer;
    fn adjacency_matrices_state_restorer_mut_ref(
        &mut self,
    ) -> &mut AdjacencyMatricesWithCachedAttributesStateRestorer;
}

impl GetEdgeStoreStateReverters for EdgeStoreStateRestorer {
    fn edge_type_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer {
        &self.edge_type_indexer_state_restorer
    }

    fn edge_type_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer {
        &mut self.edge_type_indexer_state_restorer
    }

    fn adjacency_matrices_state_restorer_ref(
        &self,
    ) -> &AdjacencyMatricesWithCachedAttributesStateRestorer {
        &self.adjacency_matrices_state_restorer
    }

    fn adjacency_matrices_state_restorer_mut_ref(
        &mut self,
    ) -> &mut AdjacencyMatricesWithCachedAttributesStateRestorer {
        &mut self.adjacency_matrices_state_restorer
    }
}

impl EdgeStoreStateRestorer {
    fn new(
        edge_type_indexer_state_restorer: IndexerStateRestorer,
        adjacency_matrices_state_restorer: AdjacencyMatricesWithCachedAttributesStateRestorer,
    ) -> Self {
        Self {
            edge_type_indexer_state_restorer,
            adjacency_matrices_state_restorer,
        }
    }

    pub(crate) fn new_for_edge_store(edge_store: &EdgeStore) -> Result<Self, GraphComputingError> {
        Ok(Self::new_for_edge_tyoe_indexer(
            edge_store.edge_type_indexer_ref(),
        )?)
    }

    pub(crate) fn new_for_edge_tyoe_indexer(
        edge_type_indexer: &Indexer,
    ) -> Result<Self, GraphComputingError> {
        let edge_type_indexer_state_restorer =
            IndexerStateRestorer::new_for_indexer(edge_type_indexer)?;

        let adjacency_matrices_state_restorer =
            AdjacencyMatricesWithCachedAttributesStateRestorer::with_edge_type_length_to_restore(
                edge_type_indexer.capacity()?,
            );

        Ok(Self {
            edge_type_indexer_state_restorer,
            adjacency_matrices_state_restorer: adjacency_matrices_state_restorer,
        })
    }
}
