use std::mem;

use crate::error::GraphComputingError;
use crate::graph::vertex_store::{GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore};
use crate::operators::transaction::{RestoreState, UseAtomicTransaction};

use super::VertexStoreStateRestorer;

pub(crate) trait UseVertexStoreTransaction: UseAtomicTransaction {}

// #[derive(Clone, Debug)]
// pub(crate) struct VertexStore {
//     graphblas_context: Arc<GraphblasContext>,
//     vertex_type_indexer: VertexTypeIndexer,
//     vertex_vectors: Vec<VertexVector>,
//     element_indexer: VertexElementIndexer,
// }

pub(crate) struct AtomicInMemoryVertexStoreTransaction<'s> {
    pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store:
        &'s mut VertexStore,
    pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store_state_restorer:
        VertexStoreStateRestorer,
}

impl<'s> AtomicInMemoryVertexStoreTransaction<'s> {
    pub(crate) fn new(vertex_store: &'s mut VertexStore) -> Result<Self, GraphComputingError> {
        let vertex_store_state_restorer = VertexStoreStateRestorer::new_for_indexers(
            vertex_store.vertex_type_indexer_ref(),
            vertex_store.element_indexer_ref(),
        )?;

        Ok(Self {
            vertex_store,
            vertex_store_state_restorer,
        })
    }
}

pub(crate) trait GetVertexStore {
    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;
}

impl<'t> GetVertexStore for AtomicInMemoryVertexStoreTransaction<'t> {
    fn vertex_store_ref(&self) -> &VertexStore {
        &self.vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.vertex_store
    }
}

pub(crate) trait GetVertexStoreStateRestorer {
    fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer;
    fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer;
}

impl<'t> GetVertexStoreStateRestorer for AtomicInMemoryVertexStoreTransaction<'t> {
    fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer {
        &self.vertex_store_state_restorer
    }

    fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer {
        &mut self.vertex_store_state_restorer
    }
}

impl<'s> UseAtomicTransaction for AtomicInMemoryVertexStoreTransaction<'s> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        let reset_vertex_store_state_restorer = self
            .vertex_store_state_restorer
            .with_reset_state_to_restore();
        let vertex_store_state_restorer = mem::replace(
            &mut self.vertex_store_state_restorer,
            reset_vertex_store_state_restorer,
        );

        vertex_store_state_restorer.restore(&mut self.vertex_store)
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer =
            VertexStoreStateRestorer::new_for_vertex_store(self.vertex_store)?;
        Ok(())
    }
}

impl<'s> Drop for AtomicInMemoryVertexStoreTransaction<'s> {
    fn drop(&mut self) {
        self.revert();
    }
}
