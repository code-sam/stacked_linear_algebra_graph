use std::fmt::Debug;
use std::sync::Arc;

pub(crate) use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;
use graphblas_sparse_linear_algebra::context::{
    MatrixStorageFormat as GraphblasMatrixStorageFormat, Mode as GraphblasMode,
};

use crate::graph::edge_store::operations::resize_adjacency_matrices::ResizeAdjacencyMatrices;
use crate::graph::index::ElementCount;
use crate::graph::{indexer::Index, vertex_store::VertexStore};
use crate::{
    error::GraphComputingError,
    graph::{edge_store::EdgeStore, vertex_store::VertexStoreTrait},
};

use super::GraphblasOperatorApplierCollection;

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

// TODO: is this the best place to define these?
pub type VertexIndex = Index;
pub type VertexTypeIndex = Index;
pub type EdgeTypeIndex = Index;

pub(crate) trait GraphTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;

    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;

    fn edge_store_ref(&self) -> &EdgeStore;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore;

    fn update_vertex_capacity(
        &mut self,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl GraphTrait for Graph {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }

    fn vertex_store_ref(&self) -> &VertexStore {
        &self.vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.vertex_store
    }

    fn edge_store_ref(&self) -> &EdgeStore {
        &self.edge_store
    }

    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore {
        &mut self.edge_store
    }

    fn update_vertex_capacity(
        &mut self,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .resize_vertex_vectors(*vertex_capacity)?;
        self.edge_store_mut_ref()
            .resize_adjacency_matrices(*vertex_capacity)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    graphblas_context: Arc<GraphblasContext>,
    vertex_store: VertexStore,
    edge_store: EdgeStore,

    graphblas_operator_applier_collection: GraphblasOperatorApplierCollection,
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

        let graph: Graph = Self {
            graphblas_context: graphblas_context.clone(),

            vertex_store: VertexStore::with_initial_capacity(
                &graphblas_context,
                initial_vertex_type_capacity,
                initial_vertex_capacity,
            )?,
            edge_store: EdgeStore::with_initial_capacity(
                &graphblas_context,
                &initial_vertex_capacity,
                &initial_edge_type_capacity,
            )?,
            graphblas_operator_applier_collection: GraphblasOperatorApplierCollection::new(
                &graphblas_context,
            ),
        };

        Ok(graph)
    }

    pub(crate) fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
        &mut self.vertex_store
    }

    pub(crate) fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
        &mut self.edge_store
    }

    pub(crate) fn graphblas_operator_applier_collection_ref(
        &self,
    ) -> &GraphblasOperatorApplierCollection {
        &self.graphblas_operator_applier_collection
    }
}

#[cfg(test)]
mod tests {

    // use crate::operations::add_vertex::AddVertex;
    // use crate::operations::read_vertex_value::ReadVertexValue;

    // #[test]
    // fn new_graph() {
    //     let graph = Graph::with_initial_capacity(10, 20).unwrap();
    //     // assert_eq!(graph.number_of_vertices().unwrap(), 0);
    //     // assert_eq!(graph.number_of_edge_types().unwrap(), 0); // TODO: fix this
    // }

    // #[test]
    // fn graph_isolation() {
    //     let mut graph_1 = Graph::new(10, 20).unwrap();
    //     let mut graph_2 = Graph::new(10, 20).unwrap();

    //     let vertex_key = String::from("A key");
    //     let vertex_property_1 = String::from("Graph 1");
    //     let vertex_property_2 = String::from("Graph 2");

    //     let vertex_to_add_1 = Vertex::new(vertex_key.clone(), vertex_property_1.clone().into());
    //     graph_1
    //         .add_or_replace_vertex(vertex_to_add_1.clone())
    //         .unwrap();

    //     let vertex_to_add_2 = Vertex::new(vertex_key.clone(), vertex_property_2.clone().into());
    //     graph_2
    //         .add_or_replace_vertex(vertex_to_add_2.clone())
    //         .unwrap();

    //     assert_eq!(
    //         *graph_1.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add_1.value()
    //     );

    //     assert_eq!(
    //         *graph_2.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add_2.value()
    //     );
    // }
}
