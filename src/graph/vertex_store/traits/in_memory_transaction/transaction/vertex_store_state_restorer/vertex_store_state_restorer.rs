use crate::error::GraphComputingError;
use crate::graph::indexing::traits::in_memory_transaction::IndexerStateRestorer;
use crate::graph::indexing::{GetIndexCapacity, Indexer};
use crate::graph::vertex_store::{
    GetVertexElementIndexer, GetVertexTypeIndexer, GetVertexVectors, VertexStore,
};
use crate::transaction::RestoreState;

use super::VertexVectorsStateRestorer;

pub(crate) struct VertexStoreStateRestorer {
    vertex_type_indexer_state_restorer: IndexerStateRestorer,
    element_indexer_state_restorer: IndexerStateRestorer,

    vertex_vectors_state_restorer: VertexVectorsStateRestorer,
}

// TODO: moving the implementation of RestoreState to operations likely requires cloning the restorers.
impl RestoreState<VertexStore> for VertexStoreStateRestorer {
    fn restore(self, instance_to_restore: &mut VertexStore) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_state_restorer
            .restore(instance_to_restore.vertex_type_indexer_mut_ref())?;
        self.element_indexer_state_restorer
            .restore(instance_to_restore.element_indexer_mut_ref())?;

        self.vertex_vectors_state_restorer
            .restore(instance_to_restore.vertex_vector_for_all_vertex_types_mut())?;

        Ok(())
    }

    fn with_reset_state_to_restore(&self) -> Self {
        let vertex_type_indexer_state_restorer = self
            .vertex_type_indexer_state_restorer
            .with_reset_state_to_restore();
        let element_indexer_state_restorer = self
            .element_indexer_state_restorer
            .with_reset_state_to_restore();

        let vertex_vectors_state_restorer = self
            .vertex_vectors_state_restorer
            .with_reset_state_to_restore();

        VertexStoreStateRestorer::new(
            vertex_type_indexer_state_restorer,
            element_indexer_state_restorer,
            vertex_vectors_state_restorer,
        )
    }
}

pub(crate) trait GetVertexStoreStateReverters {
    fn vertex_type_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer;
    fn vertex_type_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer;

    fn element_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer;
    fn element_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer;

    fn vertex_vectors_state_restorer_ref(&self) -> &VertexVectorsStateRestorer;
    fn vertex_vectors_state_restorer_mut_ref(&mut self) -> &mut VertexVectorsStateRestorer;
}

impl GetVertexStoreStateReverters for VertexStoreStateRestorer {
    fn vertex_type_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer {
        &self.vertex_type_indexer_state_restorer
    }

    fn vertex_type_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer {
        &mut self.vertex_type_indexer_state_restorer
    }

    fn element_indexer_state_restorer_ref(&self) -> &IndexerStateRestorer {
        &self.element_indexer_state_restorer
    }

    fn element_indexer_state_restorer_mut_ref(&mut self) -> &mut IndexerStateRestorer {
        &mut self.element_indexer_state_restorer
    }

    fn vertex_vectors_state_restorer_ref(&self) -> &VertexVectorsStateRestorer {
        &self.vertex_vectors_state_restorer
    }

    fn vertex_vectors_state_restorer_mut_ref(&mut self) -> &mut VertexVectorsStateRestorer {
        &mut self.vertex_vectors_state_restorer
    }
}

impl VertexStoreStateRestorer {
    fn new(
        vertex_type_indexer_state_restorer: IndexerStateRestorer,
        element_indexer_state_restorer: IndexerStateRestorer,
        vertex_vectors_state_restorer: VertexVectorsStateRestorer,
    ) -> Self {
        Self {
            vertex_type_indexer_state_restorer,
            element_indexer_state_restorer,
            vertex_vectors_state_restorer,
        }
    }

    pub(crate) fn new_for_vertex_store(
        vertex_store: &VertexStore,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self::new_for_indexers(
            vertex_store.vertex_type_indexer_ref(),
            vertex_store.element_indexer_ref(),
        )?)
    }

    pub(crate) fn new_for_indexers(
        vertex_type_indexer: &Indexer,
        vertex_element_indexer: &Indexer,
    ) -> Result<Self, GraphComputingError> {
        let vertex_type_indexer_state_restorer =
            IndexerStateRestorer::new_for_indexer(vertex_type_indexer)?;
        let element_indexer_state_restorer =
            IndexerStateRestorer::new_for_indexer(vertex_element_indexer)?;

        let vertex_vectors_state_restorer =
            VertexVectorsStateRestorer::with_vertex_type_vector_length_to_restore(
                vertex_type_indexer.capacity()?,
            );

        Ok(Self {
            vertex_type_indexer_state_restorer,
            element_indexer_state_restorer,
            vertex_vectors_state_restorer,
        })
    }
}
