use std::mem;

use crate::error::GraphComputingError;
use crate::graph::edge_store::{EdgeStore, GetEdgeTypeIndicer};
use crate::operators::transaction::{RestoreState, UseTransaction};

use super::EdgeStoreStateRestorer;

pub(crate) trait UseEdgeStoreTransaction: UseTransaction {}

// #[derive(Clone, Debug)]
// pub(crate) struct VertexStore {
//     graphblas_context: Arc<GraphblasContext>,
//     vertex_type_indexer: VertexTypeIndexer,
//     vertex_vectors: Vec<VertexVector>,
//     element_indexer: VertexElementIndexer,
// }

#[derive(Debug)]
pub(crate) struct InMemoryEdgeStoreTransaction<'s> {
    pub(in crate::graph::edge_store::operations::in_memory_transaction) edge_store:
        &'s mut EdgeStore,
    pub(in crate::graph::edge_store::operations::in_memory_transaction) edge_store_state_restorer:
        EdgeStoreStateRestorer,
}

impl<'s> InMemoryEdgeStoreTransaction<'s> {
    pub(crate) fn new(edge_store: &'s mut EdgeStore) -> Result<Self, GraphComputingError> {
        let edge_store_state_restorer =
            EdgeStoreStateRestorer::new_for_edge_tyoe_indexer(edge_store.edge_type_indexer_ref())?;

        Ok(Self {
            edge_store,
            edge_store_state_restorer,
        })
    }
}

pub(crate) trait GetEdgeStore {
    fn edge_store_ref(&self) -> &EdgeStore;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore;
}

impl<'t> GetEdgeStore for InMemoryEdgeStoreTransaction<'t> {
    fn edge_store_ref(&self) -> &EdgeStore {
        &self.edge_store
    }

    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore {
        &mut self.edge_store
    }
}

pub(crate) trait GetEdgeStoreStateRestorer {
    fn edge_store_state_restorer_ref(&self) -> &EdgeStoreStateRestorer;
    fn edge_store_state_restorer_mut_ref(&mut self) -> &mut EdgeStoreStateRestorer;
}

impl<'t> GetEdgeStoreStateRestorer for InMemoryEdgeStoreTransaction<'t> {
    fn edge_store_state_restorer_ref(&self) -> &EdgeStoreStateRestorer {
        &self.edge_store_state_restorer
    }

    fn edge_store_state_restorer_mut_ref(&mut self) -> &mut EdgeStoreStateRestorer {
        &mut self.edge_store_state_restorer
    }
}

impl<'s> UseTransaction for InMemoryEdgeStoreTransaction<'s> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        let reset_edge_store_state_restorer =
            self.edge_store_state_restorer.with_reset_state_to_restore();
        let edge_store_state_restorer = mem::replace(
            &mut self.edge_store_state_restorer,
            reset_edge_store_state_restorer,
        );

        edge_store_state_restorer.restore(&mut self.edge_store)
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        self.edge_store_state_restorer =
            EdgeStoreStateRestorer::new_for_edge_store(self.edge_store)?;
        Ok(())
    }
}

impl<'s> Drop for InMemoryEdgeStoreTransaction<'s> {
    fn drop(&mut self) {
        if let Err(e) = self.revert() {
            println!("Failed to revert transaction: {:?}", e);

            #[cfg(debug_assertions)]
            panic!("Failed to revert transaction: {:?}", e);
        }
    }
}
