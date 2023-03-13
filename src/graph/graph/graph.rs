use std::fmt::Debug;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    context::{Context as GraphblasContext, Mode as GraphblasMode},
    index::ElementIndex as GraphblasElementIndex,
};
use hashbrown::HashMap;

use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType, UserError, UserErrorType},
    graph::{
        edge_store::EdgeStore,
        value_type::implement_macro_for_all_native_value_types,
        vertex::{VertexKeyRef, VertexTrait},
    },
};
// use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
// use crate::graph::edge::{EdgeType, EdgeTypeIndex, EdgeTypeRef};
// use crate::graph::indexed_data_store::data_store::IndexedDataStore;
// use crate::graph::graph::indexed_matrix_store::indexed_matrix_store::{
//     IndexedMatrixStore, VertexData,
// };
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::index::{ElementCount, IndexTrait};
use crate::graph::vertex::{Vertex, VertexKey};
use crate::graph::vertex_store::operations::AddVertex;
// use crate::operations::{add_edge_type::AddEdgeType, drop_edge_type::DropEdgeType};

use crate::graph::value_type::NativeDataType;
use crate::graph::value_type::ValueType;

use crate::graph::vertex_store::operations::Indexing;
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
pub type EdgeTypeIndex = Index;

pub(crate) trait GraphTrait<T: ValueType> {
    // fn update_vertex_value_by_index(&mut self, index: &VertexIndex, value: T) -> Result<(), GraphComputingError>;

    fn vertex_store_ref(&self) -> &VertexStore<T>;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore<T>;

    fn edge_store_ref(&self) -> &EdgeStore<T>;
    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore<T>;

    // Encapsulate indexer-related capabilities to enable generality over how the indexer in implemented
    // (i.e. possibly by Arc<RwLock<Indexer>>)
    // fn contains_vertex_key(&self, key: &VertexKeyRef) -> bool;
    // fn vertex_key_to_index(&self, key: &VertexKeyRef) -> Option<&VertexIndex>;
}

impl<T: ValueType> GraphTrait<T> for Graph<T> {
    // fn contains_vertex_key(&self, key: &VertexKeyRef) -> bool {
    //     self.vertex_store.is_valid_key(key)
    // }

    // fn vertex_key_to_index(&self, key: &VertexKeyRef) -> Option<&VertexIndex> {
    //     self.vertex_indexer.index_for_key(key)
    // }

    fn vertex_store_ref(&self) -> &VertexStore<T> {
        &self.vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore<T> {
        &mut self.vertex_store
    }

    fn edge_store_ref(&self) -> &EdgeStore<T> {
        &self.edge_store
    }

    fn edge_store_mut_ref(&mut self) -> &mut EdgeStore<T> {
        &mut self.edge_store
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
pub struct Graph<T: ValueType> {
    graphblas_context: Arc<GraphblasContext>,
    vertex_store: VertexStore<T>,
    edge_store: EdgeStore<T>,
    // vertex_indexer: Indexer,
    // edge_type_indexer: Indexer,
    // vertex_store: IndexedDataStore<Vertex>,
    // vertex_key_to_vertex_index_map: HashMap<VertexKey, VertexIndex>, // maps a vertex key to a Vertex
    // vertex_set: FxHashSet<String>,
    // edge_types: IndexedDataStore<EdgeType>,
    // adjacency_matrices: IndexedDataStore<AdjacencyMatrix>,
    // edges: IndexedDataStore<Vec<DirectedEdge>>, // first dimension over edge_type, second over adjacency_matrix element index
    // edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex>, // maps an edge type key to an adjacency matrix
    // edge_set: FxHashSet<String>,                // TODO: type, unique connections
}

// let mut map: FxHashMap<String, ElementIndex> = FxHashMap::default();

impl<T: ValueType> Graph<T> {
    // pub fn new(
    //     initial_vertex_capacity: ElementCount,
    //     initial_edge_type_capacity: ElementCount,
    // ) -> Result<Self, GraphComputingError> {}

    pub fn with_initial_capacity(
        graphblas_context: Arc<GraphblasContext>,
        initial_vertex_capacity: ElementCount,
        initial_edge_type_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        // let mut edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex> =
        // HashMap::default();
        // edge_type_to_edge_type_index_map.reserve(initial_edge_type_capacity);

        // let mut edge_set: FxHashSet<EdgeKey> = FxHashSet::default();
        // edge_set.reserve(initial_edge_capacity);

        let mut graph: Graph<T> = Self {
            graphblas_context: graphblas_context.clone(),

            vertex_store: VertexStore::with_initial_capacity(
                &graphblas_context,
                &initial_vertex_capacity,
            )?,
            edge_store: EdgeStore::with_initial_capacity(
                &graphblas_context,
                &initial_vertex_capacity,
                &initial_edge_type_capacity,
            )?,
            // vertex_indexer: Indexer::with_initial_capacity(
            //     &graphblas_context,
            //     &initial_vertex_capacity,
            // )?,
            // edge_type_indexer: Indexer::with_initial_capacity(
            //     &graphblas_context,
            //     &initial_edge_type_capacity,
            // )?,
            // edge_type_to_edge_type_index_map,
        };

        // allocate a dummy adjacency matrix to support self.expand_adjacency_matrices_to_match_target_capacity(),
        // TODO: research a more elegant alternative
        // let dummy_edge_type = EdgeType::from("Dummy_at_init");
        // graph.add_new_edge_type(dummy_edge_type.clone())?;
        // graph.drop_edge_type_with_key(dummy_edge_type.as_str())?;

        Ok(graph)
    }

    // TODO: consider to introduce a sepate data type, like GraphWithSharedInders,
    // to avoid Mutex or RwLock overhead if functionality not required.
    // pub fn with_indexers() -> Result<Self, GraphComputingError> {}

    // pub(crate) fn indexed_vertex_and_edge_matrices_ref(
    //     &self,
    // ) -> &IndexedVertexAndAdjacencyMatrixStore {
    //     &self.indexed_vertices_and_edge_matrices
    // }

    // pub(crate) fn indexed_vertices_and_edge_matrices_mut_ref(
    //     &mut self,
    // ) -> &mut IndexedVertexAndAdjacencyMatrixStore {
    //     &mut self.indexed_vertices_and_edge_matrices
    // }

    // pub(crate) fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
    //     &self.graphblas_context
    // }
    // pub(crate) fn graphblas_context_mut_ref(&mut self) -> &mut Arc<GraphblasContext> {
    //     &mut self.graphblas_context.clone()
    // }

    // pub(crate) fn vertex_store_ref(&self) -> &IndexedDataStore<Vertex> {
    //     &self.vertex_store
    // }
    // pub(crate) fn vertex_store_mut_ref(&mut self) -> &mut IndexedDataStore<Vertex> {
    //     &mut self.vertex_store
    // }

    // pub(crate) fn adjacency_matrices_ref(&self) -> &IndexedDataStore<AdjacencyMatrix> {
    //     &self.adjacency_matrices
    // }
    // pub(crate) fn adjacency_matrices_mut_ref(&mut self) -> &mut IndexedDataStore<AdjacencyMatrix> {
    //     &mut self.adjacency_matrices
    // }

    // pub(crate) fn edge_type_to_edge_type_index_map_ref(&self) -> &HashMap<EdgeType, EdgeTypeIndex> {
    //     &self.edge_type_to_edge_type_index_map
    // }
    // pub(crate) fn edge_type_to_edge_type_index_map_mut_ref(
    //     &mut self,
    // ) -> &mut HashMap<EdgeType, EdgeTypeIndex> {
    //     &mut self.edge_type_to_edge_type_index_map
    // }

    // pub(crate) fn expand_adjacency_matrices_to_match_vertex_capacity(
    //     &mut self,
    // ) -> Result<(), GraphComputingError> {
    //     // REVIEW: would it be more efficient to allocate a freed adjacency matrix at matrix initialization, instead of doing this check everytime?
    //     // if self.adjacency_matrices.get_number_of_stored_and_reusable_elements()? > 0 {
    //     match self.adjacency_matrices.get_ref(EdgeTypeIndex::new(0)) {
    //         // this line required the allocation of a dummy adjacency matrix at graph initialization. Review if a more elegant solution can be used.
    //         Err(_) => Ok(()), // TODO: check error type, pass error if not index-out-bounds
    //         Ok(adjacency_matrix) => {
    //             let target_capacity = self.vertex_capacity()?;
    //             if target_capacity > adjacency_matrix.get_vertex_capacity()? {
    //                 let resize_adjacency_matrix = |adjacency_matrix: &mut AdjacencyMatrix| -> Result<(), GraphComputingError> {
    //                         adjacency_matrix.resize(target_capacity) // REVIEW: return error instead of panic
    //                     };
    //                 self.adjacency_matrices.map_mut_all(resize_adjacency_matrix)
    //             } else {
    //                 Ok(())
    //             }
    //         }
    //     }
    //     // } else {
    //     //     Ok(())
    //     // }
    // }

    // pub(crate) fn get_edge_adjacency_matrix_ref(
    //     &self,
    //     edge_type: &EdgeTypeRef,
    // ) -> Result<&AdjacencyMatrix, GraphComputingError> {
    //     match self.edge_type_to_edge_type_index_map.get(edge_type) {
    //         None => Err(UserError::new(
    //             UserErrorType::EdgeTypeDoesNotExist,
    //             format!("Edge type {} does not exist", edge_type),
    //             None,
    //         )
    //         .into()),
    //         Some(&index) => match self.adjacency_matrices.get_ref(index) {
    //             Ok(adjacency_matrix) => Ok(adjacency_matrix),
    //             Err(_) => Err(LogicError::new(
    //                 // TODO: match actual error type
    //                 LogicErrorType::Other,
    //                 format!(
    //                     "No adjacency matrix at mapped edge type index [{}]",
    //                     index.index()
    //                 ),
    //                 None,
    //             )
    //             .into()),
    //         },
    //     }
    // }

    // pub(crate) fn get_edge_adjacency_matrix_mut_ref(
    //     &mut self,
    //     edge_type: &EdgeTypeRef,
    // ) -> Result<&mut AdjacencyMatrix, GraphComputingError> {
    //     match self.edge_type_to_edge_type_index_map.get(edge_type) {
    //         None => Err(UserError::new(
    //             UserErrorType::EdgeTypeDoesNotExist,
    //             format!("Edge type {} does not exist", edge_type),
    //             None,
    //         )
    //         .into()),
    //         Some(&index) => match self.adjacency_matrices.get_mut_ref(index) {
    //             Ok(adjacency_matrix) => Ok(adjacency_matrix),
    //             Err(_) => Err(LogicError::new(
    //                 // TODO: match actual error type
    //                 LogicErrorType::Other,
    //                 format!(
    //                     "No adjacency matrix at mapped edge type index [{}]",
    //                     index.index()
    //                 ),
    //                 None,
    //             )
    //             .into()),
    //         },
    //     }
    // }

    // fn get_adjacency_matrix_target_capacity(&self) -> Result<VertexIndex, GraphComputingError> {
    //     Ok(self.vertex_values.get_capacity()?)
    // }

    // pub(crate) fn index_mask_with_all_vertices(&self) -> &SparseVector<bool> {
    //     self.vertex_store.mask_with_valid_indices_ref()
    // }

    // pub(crate) fn index_mask_with_all_adjacency_matrices(&self) -> &SparseVector<bool> {
    //     self.adjacency_matrices.mask_with_valid_indices_ref()
    // }
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
