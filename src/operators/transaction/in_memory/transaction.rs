use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::InMemoryEdgeStoreTransaction;
use crate::graph::graph::{
    GetGraphblasContext, GetGraphblasOperatorApplierCollection, Graph,
    GraphblasOperatorApplierCollection,
};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::operators::transaction::UseTransaction;

// pub struct Graph {
//     pub(crate) graphblas_context: Arc<GraphblasContext>,
//     pub(crate) graphblas_operator_applier_collection: GraphblasOperatorApplierCollection,

//     pub(crate) public_vertex_store: VertexStore,
//     pub(crate) public_edge_store: EdgeStore,

//     pub(crate) private_vertex_store: VertexStore,
//     pub(crate) private_edge_store: EdgeStore,
// }

pub struct InMemoryGraphTransaction<'g> {
    graphblas_context: Arc<GraphblasContext>,
    pub(crate) graphblas_operator_applier_collection: &'g GraphblasOperatorApplierCollection,
    // graph: &'g mut Graph,
    pub(crate) vertex_store_transaction: InMemoryVertexStoreTransaction<'g>,
    pub(crate) edge_store_transaction: InMemoryEdgeStoreTransaction<'g>,
}

impl<'g> UseTransaction for InMemoryGraphTransaction<'g> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction.revert()?;
        self.edge_store_transaction.revert()?;
        Ok(())
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        self.vertex_store_transaction.commit()?;
        self.edge_store_transaction.commit()?;
        Ok(())
    }
}

impl<'t> InMemoryGraphTransaction<'t> {
    pub fn new(graph: &'t mut Graph) -> Result<Self, GraphComputingError> {
        let graphblas_context = graph.graphblas_context();
        let graphblas_operator_applier_collection = &graph.graphblas_operator_applier_collection;

        let vertex_store_transaction =
            InMemoryVertexStoreTransaction::new(&mut graph.public_vertex_store)?;
        let edge_store_transaction =
            InMemoryEdgeStoreTransaction::new(&mut graph.public_edge_store)?;

        Ok(Self {
            graphblas_context,
            graphblas_operator_applier_collection,
            // graph,
            vertex_store_transaction,
            edge_store_transaction,
        })
    }
}

// impl<'t> GetGraph for InMemoryGraphTransaction<'t> {
//     fn graph_ref(&self) -> &Graph {
//         &self.graph
//     }

//     fn graph_mut_ref(&mut self) -> &mut Graph {
//         &mut self.graph
//     }
// }

impl<'t> Drop for InMemoryGraphTransaction<'t> {
    fn drop(&mut self) {
        if let Err(e) = self.revert() {
            println!("Failed to revert transaction: {:?}", e);

            #[cfg(debug_assertions)]
            panic!("Failed to revert transaction: {:?}", e);
        }
    }
}

// impl GetVertexStore for Graph {
//     fn vertex_store_ref(&self) -> &VertexStore {
//         &self.public_vertex_store
//     }

//     fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
//         &mut self.public_vertex_store
//     }

//     fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
//         &mut self.public_vertex_store
//     }
// }

// impl GetEdgeStore for Graph {
//     fn edge_store_ref(&self) -> &EdgeStore {
//         &self.public_edge_store
//     }

//     fn edge_store_mut_ref(&mut self) -> &mut EdgeStore {
//         &mut self.public_edge_store
//     }

//     fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
//         &mut self.public_edge_store
//     }
// }

impl<'g> GetGraphblasContext for InMemoryGraphTransaction<'g> {
    fn graphblas_context(&self) -> Arc<GraphblasContext> {
        self.graphblas_context.to_owned()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
}

impl<'g> GetGraphblasOperatorApplierCollection for InMemoryGraphTransaction<'g> {
    fn graphblas_operator_applier_collection_ref(&self) -> &GraphblasOperatorApplierCollection {
        &self.graphblas_operator_applier_collection
    }
}

// use std::mem;

// use crate::error::GraphComputingError;
// use crate::graph::vertex_store::{GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore};
// use crate::operators::transaction::{RestoreState, UseTransaction};

// use super::VertexStoreStateRestorer;

// pub(crate) trait UseVertexStoreTransaction: UseTransaction {}

// // #[derive(Clone, Debug)]
// // pub(crate) struct VertexStore {
// //     graphblas_context: Arc<GraphblasContext>,
// //     vertex_type_indexer: VertexTypeIndexer,
// //     vertex_vectors: Vec<VertexVector>,
// //     element_indexer: VertexElementIndexer,
// // }

// pub(crate) struct InMemoryVertexStoreTransaction<'s> {
//     pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store:
//         &'s mut VertexStore,
//     pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store_state_restorer:
//         VertexStoreStateRestorer,
// }

// impl<'s> InMemoryVertexStoreTransaction<'s> {
//     pub(crate) fn new(vertex_store: &'s mut VertexStore) -> Result<Self, GraphComputingError> {
//         let vertex_store_state_restorer = VertexStoreStateRestorer::new_for_indexers(
//             vertex_store.vertex_type_indexer_ref(),
//             vertex_store.element_indexer_ref(),
//         )?;

//         Ok(Self {
//             vertex_store,
//             vertex_store_state_restorer,
//         })
//     }
// }

// pub(crate) trait GetVertexStore {
//     fn vertex_store_ref(&self) -> &VertexStore;
//     fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;
// }

// impl<'t> GetVertexStore for InMemoryVertexStoreTransaction<'t> {
//     fn vertex_store_ref(&self) -> &VertexStore {
//         &self.vertex_store
//     }

//     fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
//         &mut self.vertex_store
//     }
// }

// pub(crate) trait GetVertexStoreStateRestorer {
//     fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer;
//     fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer;
// }

// impl<'t> GetVertexStoreStateRestorer for InMemoryVertexStoreTransaction<'t> {
//     fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer {
//         &self.vertex_store_state_restorer
//     }

//     fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer {
//         &mut self.vertex_store_state_restorer
//     }
// }

// impl<'s> UseTransaction for InMemoryVertexStoreTransaction<'s> {
//     fn revert(&mut self) -> Result<(), GraphComputingError> {
//         let reset_vertex_store_state_restorer =
//             VertexStoreStateRestorer::new_for_vertex_store(self.vertex_store)?;
//         let reset_vertex_store_state_restorer = self
//             .vertex_store_state_restorer
//             .with_reset_state_to_restore();
//         let vertex_store_state_restorer = mem::replace(
//             &mut self.vertex_store_state_restorer,
//             reset_vertex_store_state_restorer,
//         );

//         vertex_store_state_restorer.restore(&mut self.vertex_store)
//     }

//     fn commit(&mut self) -> Result<(), GraphComputingError> {
//         self.vertex_store_state_restorer =
//             VertexStoreStateRestorer::new_for_vertex_store(self.vertex_store)?;
//         Ok(())
//     }
// }

// impl<'s> Drop for InMemoryVertexStoreTransaction<'s> {
//     fn drop(&mut self) {
//         if let Err(e) = self.revert() {
//             println!("Failed to revert transaction: {:?}", e);

//             #[cfg(debug_assertions)]
//             panic!("Failed to revert transaction: {:?}", e);
//         }
//     }
// }
