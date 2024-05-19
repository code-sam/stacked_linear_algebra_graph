use std::fmt::Debug;
use std::sync::Arc;

pub(crate) use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;
use graphblas_sparse_linear_algebra::context::{
    MatrixStorageFormat as GraphblasMatrixStorageFormat, Mode as GraphblasMode,
};

use crate::graph::indexing::ElementCount;
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

pub(crate) trait GetGraphblasContext {
    fn graphblas_context(&self) -> Arc<GraphblasContext>;
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;
}

pub(crate) trait GetVertexStore {
    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;
    fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore;
}

pub(crate) trait GetEdgeStore {
    fn edge_store_ref(&self) -> &EdgeStore;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore;
    fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore;
}

impl GetVertexStore for Graph {
    fn vertex_store_ref(&self) -> &VertexStore {
        &self.vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.vertex_store
    }

    fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
        &mut self.vertex_store
    }
}

impl GetEdgeStore for Graph {
    fn edge_store_ref(&self) -> &EdgeStore {
        &self.edge_store
    }

    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore {
        &mut self.edge_store
    }

    fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
        &mut self.edge_store
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
    graphblas_context: Arc<GraphblasContext>,
    graphblas_operator_applier_collection: GraphblasOperatorApplierCollection,

    vertex_store: VertexStore,
    edge_store: EdgeStore,
}

impl Graph {
    pub fn with_initial_capacity(
        initial_vertex_type_capacity: &ElementCount,
        initial_vertex_capacity: &ElementCount,
        initial_edge_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let graphblas_context = GraphblasContext::init(
            GraphblasMode::NonBlocking,
            GraphblasMatrixStorageFormat::ByColumn,
        )?;

        let vertex_store = VertexStore::with_initial_capacity(
            &graphblas_context,
            initial_vertex_type_capacity,
            initial_vertex_capacity,
        )?;
        let edge_store = EdgeStore::with_initial_capacity(
            &graphblas_context,
            &initial_vertex_capacity,
            &initial_edge_type_capacity,
        )?;

        let graph: Graph = Self {
            graphblas_context: graphblas_context.clone(),
            graphblas_operator_applier_collection: GraphblasOperatorApplierCollection::new(
                &graphblas_context,
            ),

            vertex_store,
            edge_store,
        };

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use crate::operators::operators::{
        add::{AddVertex, AddVertexType},
        read::GetVertexValue,
        update::UpdateVertexValue,
    };

    use super::*;

    #[test]
    fn graph_isolation() {
        let mut graph_1 = Graph::with_initial_capacity(&10, &20, &20).unwrap();
        let mut graph_2 = Graph::with_initial_capacity(&10, &20, &20).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph_1).unwrap();
        let vertex_type_2_index = AddVertexType::<u8>::apply(&mut graph_2).unwrap();

        let vertex_1_index = graph_1
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph_2
            .add_vertex(&vertex_type_2_index, vertex_value_2.clone())
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
        let mut graph_1 = Graph::with_initial_capacity(&10, &20, &20).unwrap();

        let vertex_type_11_index = AddVertexType::<u8>::apply(&mut graph_1).unwrap();

        let vertex_11_index = graph_1
            .add_vertex(&vertex_type_11_index, 1.clone())
            .unwrap();

        let mut graph_2 = graph_1.clone();

        let vertex_type_21_index = AddVertexType::<u8>::apply(&mut graph_2).unwrap();

        assert_ne!(vertex_type_11_index, vertex_type_21_index);

        let vertex_21_index = graph_2.add_vertex(&vertex_type_11_index, 2).unwrap();

        assert_ne!(vertex_11_index, vertex_21_index);

        assert_eq!(
            GetVertexValue::<u8>::vertex_value(&graph_2, &vertex_type_11_index, &vertex_11_index)
                .unwrap(),
            Some(1)
        );

        graph_1
            .update_vertex_value(&vertex_type_11_index, &vertex_11_index, 4)
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
