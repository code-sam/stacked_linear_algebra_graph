use std::fmt::Debug;
use std::sync::Arc;

pub(crate) use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;
use graphblas_sparse_linear_algebra::context::{
    MatrixStorageFormat as GraphblasMatrixStorageFormat, Mode as GraphblasMode,
};

use crate::graph::indexing::{ElementCount, MINIMUM_INDEXER_CAPACITY};
use crate::graph::vertex_store::VertexStore;
use crate::{error::GraphComputingError, graph::edge_store::EdgeStore};

use super::{GetGraphblasOperatorApplierCollection, GraphblasOperatorApplierCollection};

// NOTE: by default, SuiteSparse:GraphBLAS uses Compressed Sparse Row (CSR) format.
// Row operations should therefore be faster.
// TODO: review performance optimizations by using row operations, instead of column operations.

// pub trait GraphTrait {
// Should support sharing data between multiple graphs. REVIEW: is this really useful?
// This exposes the GraphBLAS implementation in the public API. If implemented, consider to
// wrap the GraphBLAS context into some more generic and crate-specified data struct.
// fn in_context(
//     graphblas_context: Arc<GraphblasContext>
//     initial_vertex_capacity: ElementCount,
//     initial_edge_type_capacity: ElementCount,
// ) -> Result<Self, GraphComputingError>;
// }

// TODO: set these values for better performance
const INITIAL_PRIVATE_VERTEX_TYPE_CAPACITY: ElementCount = MINIMUM_INDEXER_CAPACITY;
const INITIAL_PRIVATE_VERTEX_CAPACITY: ElementCount = MINIMUM_INDEXER_CAPACITY;
const INITIAL_PRIVATE_EDGE_TYPE_CAPACITY: ElementCount = MINIMUM_INDEXER_CAPACITY;

pub(crate) trait GetGraphblasContext {
    fn graphblas_context(&self) -> Arc<GraphblasContext>;
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;
}

pub(crate) trait GetVertexStore {
    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;
    fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore;
}

pub(crate) trait GetPrivateVertexStore {
    fn private_vertex_store_ref(&self) -> &VertexStore;
    fn private_vertex_store_mut_ref(&mut self) -> &mut VertexStore;
    fn private_vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore;
}

pub(crate) trait GetEdgeStore {
    fn edge_store_ref(&self) -> &EdgeStore;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore;
    fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore;
}

pub(crate) trait GetPrivateEdgeStore {
    fn private_edge_store_ref(&self) -> &EdgeStore;
    fn private_edge_store_mut_ref(&mut self) -> &mut EdgeStore;
    fn private_edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore;
}

impl GetVertexStore for Graph {
    fn vertex_store_ref(&self) -> &VertexStore {
        &self.public_vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.public_vertex_store
    }

    fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
        &mut self.public_vertex_store
    }
}

impl GetPrivateVertexStore for Graph {
    fn private_vertex_store_ref(&self) -> &VertexStore {
        &self.private_vertex_store
    }

    fn private_vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.private_vertex_store
    }

    fn private_vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
        &mut self.private_vertex_store
    }
}

impl GetEdgeStore for Graph {
    fn edge_store_ref(&self) -> &EdgeStore {
        &self.public_edge_store
    }

    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore {
        &mut self.public_edge_store
    }

    fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
        &mut self.public_edge_store
    }
}

impl GetPrivateEdgeStore for Graph {
    fn private_edge_store_ref(&self) -> &EdgeStore {
        &self.public_edge_store
    }

    fn private_edge_store_mut_ref(&mut self) -> &mut EdgeStore {
        &mut self.public_edge_store
    }

    fn private_edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
        &mut self.public_edge_store
    }
}

impl GetGraphblasContext for Graph {
    fn graphblas_context(&self) -> Arc<GraphblasContext> {
        self.graphblas_context.to_owned()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
}

impl GetGraphblasOperatorApplierCollection for Graph {
    fn graphblas_operator_applier_collection_ref(&self) -> &GraphblasOperatorApplierCollection {
        &self.graphblas_operator_applier_collection
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub(crate) graphblas_context: Arc<GraphblasContext>,
    pub(crate) graphblas_operator_applier_collection: GraphblasOperatorApplierCollection,

    pub(crate) public_vertex_store: VertexStore,
    pub(crate) public_edge_store: EdgeStore,

    pub(crate) private_vertex_store: VertexStore,
    pub(crate) private_edge_store: EdgeStore,
}

impl Graph {
    pub fn with_initial_capacity(
        initial_vertex_type_capacity: ElementCount,
        initial_vertex_capacity: ElementCount,
        initial_edge_type_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let graphblas_context = GraphblasContext::init(
            GraphblasMode::NonBlocking,
            GraphblasMatrixStorageFormat::ByColumn,
        )?;

        let public_vertex_store = VertexStore::with_initial_capacity(
            graphblas_context.clone(),
            initial_vertex_type_capacity,
            initial_vertex_capacity,
        )?;
        let public_edge_store = EdgeStore::with_initial_capacity(
            graphblas_context.clone(),
            initial_vertex_capacity,
            initial_edge_type_capacity,
        )?;

        let private_vertex_store = VertexStore::with_initial_capacity(
            graphblas_context.clone(),
            INITIAL_PRIVATE_VERTEX_TYPE_CAPACITY,
            INITIAL_PRIVATE_VERTEX_CAPACITY,
        )?;
        let private_edge_store = EdgeStore::with_initial_capacity(
            graphblas_context.clone(),
            INITIAL_PRIVATE_VERTEX_CAPACITY,
            INITIAL_PRIVATE_EDGE_TYPE_CAPACITY,
        )?;

        let graph: Graph = Self {
            graphblas_context: graphblas_context.clone(),
            graphblas_operator_applier_collection: GraphblasOperatorApplierCollection::new(
                graphblas_context,
            ),

            public_vertex_store,
            public_edge_store,

            private_vertex_store,
            private_edge_store,
        };

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use crate::operators::operators::{
        new::{NewVertex, NewVertexType},
        read::GetVertexValue,
        update::UpdateVertexValue,
    };

    use super::*;

    #[test]
    fn graph_isolation() {
        let mut graph_1 = Graph::with_initial_capacity(10, 20, 20).unwrap();
        let mut graph_2 = Graph::with_initial_capacity(10, 20, 20).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph_1).unwrap();
        let vertex_type_2_index = NewVertexType::<u8>::apply(&mut graph_2).unwrap();

        let vertex_1_index = graph_1
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph_2
            .new_vertex(&vertex_type_2_index, vertex_value_2.clone())
            .unwrap();

        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_1, &vertex_type_1_index, &vertex_1_index)
                .unwrap(),
            Some(vertex_value_1)
        );
        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_2, &vertex_type_2_index, &vertex_2_index)
                .unwrap(),
            Some(vertex_value_2)
        );
    }

    #[test]
    fn graph_cloning() {
        let mut graph_1 = Graph::with_initial_capacity(10, 20, 20).unwrap();

        let vertex_type_11_index = NewVertexType::<u8>::apply(&mut graph_1).unwrap();

        let vertex_11_index = graph_1
            .new_vertex(&vertex_type_11_index, 1.clone())
            .unwrap();

        let mut graph_2 = graph_1.clone();

        let vertex_type_21_index = NewVertexType::<u8>::apply(&mut graph_2).unwrap();

        assert_ne!(vertex_type_11_index, vertex_type_21_index);

        let vertex_21_index = graph_2.new_vertex(&vertex_type_11_index, 2).unwrap();

        assert_ne!(vertex_11_index, vertex_21_index);

        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_2, &vertex_type_11_index, &vertex_11_index)
                .unwrap(),
            Some(1)
        );

        graph_1
            .add_or_update_vertex(&vertex_type_11_index, &vertex_11_index, 4)
            .unwrap();

        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_2, &vertex_type_11_index, &vertex_11_index)
                .unwrap(),
            Some(1)
        );
        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_1, &vertex_type_11_index, &vertex_11_index)
                .unwrap(),
            Some(4)
        );
    }
}
