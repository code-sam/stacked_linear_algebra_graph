use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::in_memory_transaction::InMemoryEdgeStoreTransaction;
use crate::graph::graph::{
    GetGraphblasContext, GetGraphblasOperatorApplierCollection, Graph,
    GraphblasOperatorApplierCollection,
};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::InMemoryVertexStoreTransaction;
use crate::transaction::UseTransaction;

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
