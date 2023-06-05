use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    context::{Context as GraphblasContext, Mode as GraphblasMode},
    index::ElementIndex as GraphblasElementIndex,
    operators::apply::BinaryOperatorApplier,
};
use hashbrown::HashMap;

use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType, UserError, UserErrorType},
    graph::{
        edge_store::EdgeStore,
        value_type::implement_macro_for_all_native_value_types,
        vertex::{VertexDefinedByKeyTrait, VertexKeyRef},
        vertex_store::VertexStoreTrait,
    }, operators::GraphblasOperatorApplierCollection,
};
// use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
// use crate::graph::edge::{EdgeType, EdgeTypeIndex, EdgeTypeRef};
// use crate::graph::indexed_data_store::data_store::IndexedDataStore;
// use crate::graph::graph::indexed_matrix_store::indexed_matrix_store::{
//     IndexedMatrixStore, VertexData,
// };
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::index::ElementCount;
use crate::graph::vertex::{VertexDefinedByKey, VertexKey};
// use crate::graph::vertex_store::vertex_operations::AddVertex;
// use crate::operations::{add_edge_type::AddEdgeType, drop_edge_type::DropEdgeType};

use crate::graph::value_type::NativeDataType;
use crate::graph::value_type::ValueType;

// use crate::graph::vertex_store::vertex_operations::Indexing;
use crate::graph::{
    indexer::{Index, Indexer, IndexerTrait},
    vertex_store::VertexStore,
};

// NOTE: by default, SuiteSparse:GraphBLAS uses Compressed Sparse Row (CSR) format.
// Row operations should therefore be faster.
// TODO: review performance optimizations by using row operations, instead of column operations.

// pub type VertexIndex = IndexedDataStoreIndex;

// pub type VertexIndex = ElementIndex;
// pub type EdgeTypeIndex = IndexedDataStoreIndex;

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

// pub(crate) trait IndexedVerticesAndEdgeMatrices<T: NativeDataType, I: IndexTrait + Debug> {
//     fn indexed_vertices_and_edge_matrices_mut_ref(
//         &mut self,
//     ) -> &mut impl IndexedVertexAndEdgeMatrixStoreTrait<T, I>;
// }

// TODO: is this the best place to define these?
pub type VertexIndex = Index;
pub type VertexTypeIndex = Index;
pub type EdgeTypeIndex = Index;

pub(crate) trait GraphTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;

    // fn update_vertex_value_by_index(&mut self, index: &VertexIndex, value: T) -> Result<(), GraphComputingError>;

    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;

    fn edge_store_ref(&self) -> &EdgeStore;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore;

    fn update_vertex_capacity(
        &mut self,
        vertex_capacity: &ElementCount,
    ) -> Result<(), GraphComputingError>;

    // Encapsulate indexer-related capabilities to enable generality over how the indexer in implemented
    // (i.e. possibly by Arc<RwLock<Indexer>>)
    // fn contains_vertex_key(&self, key: &VertexKeyRef) -> bool;
    // fn vertex_key_to_index(&self, key: &VertexKeyRef) -> Option<&VertexIndex>;
}

impl GraphTrait for Graph {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }

    // fn contains_vertex_key(&self, key: &VertexKeyRef) -> bool {
    //     self.vertex_store.is_valid_key(key)
    // }

    // fn vertex_key_to_index(&self, key: &VertexKeyRef) -> Option<&VertexIndex> {
    //     self.vertex_indexer.index_for_key(key)
    // }

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

    // pub(crate) fn vertex_key_to_vertex_index_map_ref(&self) -> &HashMap<VertexKey, VertexIndex> {
    //     &self.vertex_key_to_vertex_index_map
    // }
    // pub(crate) fn vertex_key_to_vertex_index_map_mut_ref(
    //     &mut self,
    // ) -> &mut HashMap<VertexKey, VertexIndex> {
    //     &mut self.vertex_key_to_vertex_index_map
    // }

    // fn update_vertex_value_by_index(&mut self, index: &VertexIndex, value: T) -> Result<(), GraphComputingError> {
    //     self.indexed_matrix_store.set_vertex_value(index.index(), value)?;
    //     Ok(())
    // }
}

// TODO: placed in Graph module to enable per-field mutable borrow.
// impl Graph<bool> {
//     fn add_new_vertex(
//         &mut self,
//         vertex: Vertex<bool>,
//     ) -> Result<VertexIndex, GraphComputingError> {
//         self.vertex_store.add_new_vertex(
//             vertex,
//             |vertex_capacity: ElementCount| {
//                 self.edge_store
//                     .resize_adjacency_matrices(vertex_capacity)
//             },
//         )
//     }
// }

// pub struct Graph<VertexKey: Hash + Eq + PartialEq, EdgeType: Hash + Eq + PartialEq> {
#[derive(Clone, Debug)]
pub struct Graph {
    graphblas_context: Arc<GraphblasContext>,
    vertex_store: VertexStore,
    edge_store: EdgeStore,

    graphblas_operator_applier_collection: GraphblasOperatorApplierCollection,
}

// let mut map: FxHashMap<String, ElementIndex> = FxHashMap::default();

impl Graph {
    // pub fn new(
    //     initial_vertex_capacity: ElementCount,
    //     initial_edge_type_capacity: ElementCount,
    // ) -> Result<Self, GraphComputingError> {}

    pub fn with_initial_capacity(
        initial_vertex_type_capacity: &ElementCount,
        initial_vertex_capacity: &ElementCount,
        initial_edge_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        // let mut edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex> =
        // HashMap::default();
        // edge_type_to_edge_type_index_map.reserve(initial_edge_type_capacity);

        // let mut edge_set: FxHashSet<EdgeKey> = FxHashSet::default();
        // edge_set.reserve(initial_edge_capacity);

        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking)?;

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
            graphblas_operator_applier_collection: GraphblasOperatorApplierCollection::new(),
        };

        Ok(graph)
    }

    pub(crate) fn vertex_store_mut_ref_unsafe(&mut self) -> *mut VertexStore {
        &mut self.vertex_store
    }

    pub(crate) fn edge_store_mut_ref_unsafe(&mut self) -> *mut EdgeStore {
        &mut self.edge_store
    }

    pub(crate) fn graphblas_operator_applier_collection_ref(&self) -> &GraphblasOperatorApplierCollection {
        &self.graphblas_operator_applier_collection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // TODO: Test vertex capacity
    // TODO: test number of stored vertices
    // TODO: test number of stored edge types
}
