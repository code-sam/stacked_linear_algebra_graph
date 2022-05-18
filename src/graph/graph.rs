use std::sync::Arc;

use graphblas_sparse_linear_algebra::{
    context::{Context as GraphblasContext, Mode as GraphblasMode},
    util::ElementIndex,
    value_types::sparse_vector::SparseVector,
};
use hashbrown::HashMap;

use super::adjacency_matrix::{AdjacencyMatrix, EdgeCoordinate};
use super::edge::{DirectedEdge, EdgeType, EdgeTypeRef};
use super::vertex::{Vertex, VertexKey, VertexKeyRef};

use crate::error::{
    GraphComputingError, LogicError, LogicErrorType, SystemError, SystemErrorType, UserError,
    UserErrorType,
};
use crate::operations::{add_edge_type::AddEdgeType, drop_edge_type::DropEdgeType};
use crate::util::indexed_data_store::{Index as IndexedDataStoreIndexValue, IndexedDataStore};

// NOTE: by default, SuiteSparse:GraphBLAS uses Compressed Sparse Row (CSR) format.
// Row operations should therefore be faster.
// TODO: review performance optimizations by using row operations, instead of column operations.

// pub type VertexIndex = IndexedDataStoreIndex;

// pub type VertexIndex = ElementIndex;
// pub type EdgeTypeIndex = IndexedDataStoreIndex;

pub type ElementCount = ElementIndex;

// Use a struct instead of a type to discourage using and/or generating indices that are not coming from the pblic API.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VertexIndex {
    index: ElementIndex,
}

impl VertexIndex {
    pub(crate) fn new(index: ElementIndex) -> Self {
        VertexIndex { index }
    }
    pub(crate) fn index(self) -> ElementIndex {
        self.index
    }
    pub(crate) fn index_ref(&self) -> &ElementIndex {
        &self.index
    }
}

// TODO: Implementation leaks VertexIndex instantiation out of pub(crate) scope
// impl From<ElementIndex> for VertexIndex {
//     fn from(index: ElementIndex) -> Self {
//         VertexIndex::new(index)
//     }
// }
// impl From<VertexIndex> for ElementIndex {
//     fn from(index: VertexIndex) -> Self {
//         index.index()
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeTypeIndex {
    index: IndexedDataStoreIndexValue,
}

impl EdgeTypeIndex {
    pub(crate) fn new(index: IndexedDataStoreIndexValue) -> Self {
        EdgeTypeIndex { index }
    }
    pub(crate) fn index(self) -> ElementIndex {
        self.index
    }
    pub(crate) fn index_ref(&self) -> &IndexedDataStoreIndexValue {
        &self.index
    }
}

// impl From<EdgeTypeIndex> for IndexedDataStoreIndex {
//     fn from(index: EdgeTypeIndex) -> Self {
//         Self {index: *index.index()}
//     }
// }

// impl From<IndexedDataStoreIndex> for EdgeTypeIndex {
//     fn from(index: IndexedDataStoreIndex) -> Self {
//         Self {index: *index.index()}
//     }
// }

pub trait GraphTrait {
    fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError>;
    fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError>;
    // TODO: number of edges
    // TODO: number of edges per edge type, etc
}

// pub struct Graph<VertexKey: Hash + Eq + PartialEq, EdgeType: Hash + Eq + PartialEq> {
#[derive(Debug)]
pub struct Graph {
    graphblas_context: Arc<GraphblasContext>,

    vertex_store: IndexedDataStore<Vertex>,
    vertex_key_to_vertex_index_map: HashMap<VertexKey, VertexIndex>, // maps a vertex key to a Vertex
    // vertex_set: FxHashSet<String>,
    // edge_types: IndexedDataStore<EdgeType>,
    adjacency_matrices: IndexedDataStore<AdjacencyMatrix>,
    // edges: IndexedDataStore<Vec<DirectedEdge>>, // first dimension over edge_type, second over adjacency_matrix element index
    edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex>, // maps an edge type key to an adjacency matrix
                                                                        // edge_set: FxHashSet<String>,                // TODO: type, unique connections
}

// let mut map: FxHashMap<String, ElementIndex> = FxHashMap::default();

impl GraphTrait for Graph {
    fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError> {
        let number_of_vertices = self
            .index_mask_with_all_vertices()
            .number_of_stored_elements()?;
        Ok(number_of_vertices)
    }

    fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError> {
        let number_of_edge_types = self
            .index_mask_with_all_adjacency_matrices()
            .number_of_stored_elements()?;
        Ok(number_of_edge_types)
    }

    // TODO: number of edges
    // TODO: number of edges for edge type
}

// impl Graph<VertexKey, EdgeKey> {
impl<'g> Graph {
    pub fn new(
        initial_vertex_capacity: ElementCount,
        initial_edge_type_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking)?;

        let mut vertex_key_to_vertex_index_map: HashMap<VertexKey, VertexIndex> =
            HashMap::default();
        vertex_key_to_vertex_index_map.reserve(initial_vertex_capacity);

        let mut edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex> =
            HashMap::default();
        edge_type_to_edge_type_index_map.reserve(initial_edge_type_capacity);

        // let mut edge_set: FxHashSet<EdgeKey> = FxHashSet::default();
        // edge_set.reserve(initial_edge_capacity);

        let mut graph: Graph = Self {
            graphblas_context: graphblas_context.clone(),

            vertex_store: IndexedDataStore::with_capacity(
                initial_vertex_capacity,
                graphblas_context.clone(),
            )?,
            vertex_key_to_vertex_index_map,

            // edge_types: IndexedDataStore::with_capacity(&edge_capacity),
            adjacency_matrices: IndexedDataStore::with_capacity(
                initial_edge_type_capacity,
                graphblas_context.clone(),
            )?,
            // edges: IndexedDataStore::with_capacity(&initial_edge_capacity), // TODO: consider if this can be made more efficient by reserving less memory
            edge_type_to_edge_type_index_map,
            // edge_set,
        };

        // allocate a dummy adjacency matrix to support self.expand_adjacency_matrices_to_match_target_capacity(),
        // TODO: research a more elegant alternative
        let dummy_edge_type = EdgeType::from("Dummy_at_init");
        graph.add_new_edge_type(dummy_edge_type.clone())?;
        graph.drop_edge_type(dummy_edge_type.as_str())?;

        Ok(graph)
    }

    pub(crate) fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
    // pub(crate) fn graphblas_context_mut_ref(&mut self) -> &mut Arc<GraphblasContext> {
    //     &mut self.graphblas_context.clone()
    // }

    pub(crate) fn vertex_store_ref(&self) -> &IndexedDataStore<Vertex> {
        &self.vertex_store
    }
    pub(crate) fn vertex_store_mut_ref(&mut self) -> &mut IndexedDataStore<Vertex> {
        &mut self.vertex_store
    }

    pub(crate) fn vertex_key_to_vertex_index_map_ref(&self) -> &HashMap<VertexKey, VertexIndex> {
        &self.vertex_key_to_vertex_index_map
    }
    pub(crate) fn vertex_key_to_vertex_index_map_mut_ref(
        &mut self,
    ) -> &mut HashMap<VertexKey, VertexIndex> {
        &mut self.vertex_key_to_vertex_index_map
    }

    pub(crate) fn adjacency_matrices_ref(&self) -> &IndexedDataStore<AdjacencyMatrix> {
        &self.adjacency_matrices
    }
    pub(crate) fn adjacency_matrices_mut_ref(&mut self) -> &mut IndexedDataStore<AdjacencyMatrix> {
        &mut self.adjacency_matrices
    }

    pub(crate) fn edge_type_to_edge_type_index_map_ref(&self) -> &HashMap<EdgeType, EdgeTypeIndex> {
        &self.edge_type_to_edge_type_index_map
    }
    pub(crate) fn edge_type_to_edge_type_index_map_mut_ref(
        &mut self,
    ) -> &mut HashMap<EdgeType, EdgeTypeIndex> {
        &mut self.edge_type_to_edge_type_index_map
    }

    pub(crate) fn expand_adjacency_matrices_to_match_vertex_capacity(
        &mut self,
    ) -> Result<(), GraphComputingError> {
        // REVIEW: would it be more efficient to allocate a freed adjacency matrix at matrix initialization, instead of doing this check everytime?
        // if self.adjacency_matrices.get_number_of_stored_and_reusable_elements()? > 0 {
        match self.adjacency_matrices.get_ref(EdgeTypeIndex::new(0)) {
            // this line required the allocation of a dummy adjacency matrix at graph initialization. Review if a more elegant solution can be used.
            Err(_) => Ok(()), // TODO: check error type, pass error if not index-out-bounds
            Ok(adjacency_matrix) => {
                let target_capacity = self.vertex_capacity()?;
                if target_capacity > adjacency_matrix.get_vertex_capacity()? {
                    let resize_adjacency_matrix = |adjacency_matrix: &mut AdjacencyMatrix| -> Result<(), GraphComputingError> {
                            adjacency_matrix.resize(target_capacity) // REVIEW: return error instead of panic
                        };
                    self.adjacency_matrices.map_mut_all(resize_adjacency_matrix)
                } else {
                    Ok(())
                }
            }
        }
        // } else {
        //     Ok(())
        // }
    }

    pub(crate) fn set_edge_in_adjacency_matrix(
        &mut self,
        edge_to_set: &DirectedEdge,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let edge_coordinate = self.get_edge_coordinate(edge_to_set)?;
        match self.adjacency_matrices.get_mut_ref(edge_type_index) {
            Ok(edge_type_adjacency_matrix) => {
                edge_type_adjacency_matrix.add_edge(&edge_coordinate)?
            }
            Err(_) => {
                // TODO: check actual error type
                return Err(SystemError::new(
                    SystemErrorType::IndexOutOfBounds,
                    String::from("No adjacency matrix at expected index."),
                    None,
                )
                .into());
            }
        }
        Ok(())
    }

    // fn get_(&self) -> GraphSize {
    //     let number_of_vertices = self.vertex_values.length();
    //     GraphSize::new(number_of_vertices, number_of_vertices)
    // }

    pub(crate) fn get_edge_coordinate(
        &self,
        edge: &DirectedEdge,
    ) -> Result<EdgeCoordinate, GraphComputingError> {
        // let mut from_vertex_index = self
        //     .vertex_key_to_vertex_index_map
        //     .get(edge.originates_from_vertex());
        let from_vertex_index;
        match self
            .vertex_key_to_vertex_index_map
            .get(edge.originates_from_vertex())
        {
            Some(index) => from_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    String::from("Originating vertex must exist for edge"),
                    None,
                )
                .into())
            }
        }
        // let to_vertex_index = self
        //     .vertex_key_to_vertex_index_map
        //     .get(edge.goes_to_vertex())
        //     .unwrap();
        let to_vertex_index;
        match self
            .vertex_key_to_vertex_index_map
            .get(edge.points_to_vertex())
        {
            Some(index) => to_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    String::from("Destination vertex must exist for edge"),
                    None,
                )
                .into())
            }
        }

        Ok(EdgeCoordinate::new(
            *from_vertex_index.index_ref(),
            *to_vertex_index.index_ref(),
        ))
    }

    pub(crate) fn get_edge_adjacency_matrix_ref(
        &self,
        edge_type: &EdgeTypeRef,
    ) -> Result<&AdjacencyMatrix, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map.get(edge_type) {
            None => Err(UserError::new(
                UserErrorType::EdgeTypeDoesNotExist,
                format!("Edge type {} does not exist", edge_type),
                None,
            )
            .into()),
            Some(&index) => match self.adjacency_matrices.get_ref(index) {
                Ok(adjacency_matrix) => Ok(adjacency_matrix),
                Err(_) => Err(LogicError::new(
                    // TODO: match actual error type
                    LogicErrorType::Other,
                    format!(
                        "No adjacency matrix at mapped edge type index [{}]",
                        index.index()
                    ),
                    None,
                )
                .into()),
            },
        }
    }

    pub(crate) fn get_edge_adjacency_matrix_mut_ref(
        &mut self,
        edge_type: &EdgeTypeRef,
    ) -> Result<&mut AdjacencyMatrix, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map.get(edge_type) {
            None => Err(UserError::new(
                UserErrorType::EdgeTypeDoesNotExist,
                format!("Edge type {} does not exist", edge_type),
                None,
            )
            .into()),
            Some(&index) => match self.adjacency_matrices.get_mut_ref(index) {
                Ok(adjacency_matrix) => Ok(adjacency_matrix),
                Err(_) => Err(LogicError::new(
                    // TODO: match actual error type
                    LogicErrorType::Other,
                    format!(
                        "No adjacency matrix at mapped edge type index [{}]",
                        index.index()
                    ),
                    None,
                )
                .into()),
            },
        }
    }

    pub(crate) fn vertex_index_to_vertex_key_ref(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<&VertexKeyRef, GraphComputingError> {
        match self.vertex_store.get_ref(vertex_index) {
            Ok(vertex) => return Ok(vertex.key_ref()),
            Err(_) => {
                // TODO:match actual error type
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!("There is no vertex at index [{}]", vertex_index.index()),
                    None,
                )
                .into());
            }
        }
    }

    pub(crate) fn try_vertex_key_ref_to_vertex_index_ref(
        &self,
        key: &VertexKeyRef,
    ) -> Result<&VertexIndex, GraphComputingError> {
        match self.vertex_key_to_vertex_index_map_ref().get(key) {
            None => Err(SystemError::new(
                SystemErrorType::KeyNotFound,
                format!("Could not map vertex key '{}' to a vertex index", key),
                None,
            )
            .into()),
            Some(vertex_index) => Ok(vertex_index),
        }
    }

    pub(crate) fn edge_type_index_to_edge_type_ref(
        &self,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<&EdgeTypeRef, GraphComputingError> {
        match self.adjacency_matrices.get_ref(edge_type_index) {
            Ok(adjacency_matrix) => return Ok(adjacency_matrix.edge_type_ref()),
            Err(_) => {
                // TODO:match actual error type
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!("There is no vertex at index [{}]", edge_type_index.index()),
                    None,
                )
                .into());
            }
        }
    }

    pub(crate) fn try_edge_type_ref_to_edge_type_index_ref(
        &self,
        key: &EdgeTypeRef,
    ) -> Result<&EdgeTypeIndex, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map_ref().get(key) {
            None => Err(SystemError::new(
                SystemErrorType::KeyNotFound,
                format!(
                    "Could not map edge type key '{}' to an edge type index",
                    key
                ),
                None,
            )
            .into()),
            Some(edge_type_index) => Ok(edge_type_index),
        }
    }

    // fn get_adjacency_matrix_target_capacity(&self) -> Result<VertexIndex, GraphComputingError> {
    //     Ok(self.vertex_values.get_capacity()?)
    // }

    pub(crate) fn vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.vertex_store.get_capacity()?)
    }

    pub(crate) fn index_mask_with_all_vertices(&self) -> &SparseVector<bool> {
        self.vertex_store.mask_with_valid_indices_ref()
    }

    pub(crate) fn index_mask_with_all_adjacency_matrices(&self) -> &SparseVector<bool> {
        self.adjacency_matrices.mask_with_valid_indices_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::vertex::VertexValue;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_vertex_value::ReadVertexValue;

    #[test]
    fn new_graph() {
        let graph = Graph::new(10, 20).unwrap();
        assert_eq!(graph.number_of_vertices().unwrap(), 0);
        // assert_eq!(graph.number_of_edge_types().unwrap(), 0); // TODO: fix this
    }

    #[test]
    fn graph_isolation() {
        let mut graph_1 = Graph::new(10, 20).unwrap();
        let mut graph_2 = Graph::new(10, 20).unwrap();

        let vertex_key = String::from("A key");
        let vertex_property_1 = String::from("Graph 1");
        let vertex_property_2 = String::from("Graph 2");

        let vertex_to_add_1 = Vertex::new(vertex_key.clone(), vertex_property_1.clone().into());
        graph_1
            .add_or_replace_vertex(vertex_to_add_1.clone())
            .unwrap();

        let vertex_to_add_2 = Vertex::new(vertex_key.clone(), vertex_property_2.clone().into());
        graph_2
            .add_or_replace_vertex(vertex_to_add_2.clone())
            .unwrap();

        assert_eq!(
            *graph_1.vertex_value(&vertex_key).unwrap(),
            vertex_to_add_1.value()
        );

        assert_eq!(
            *graph_2.vertex_value(&vertex_key).unwrap(),
            vertex_to_add_2.value()
        );
    }

    #[test]
    fn test_convert_vertex_index_to_vertex_key_ref() {
        let mut graph = Graph::new(10, 20).unwrap();

        let vertex_key_1 = String::from("Vertex_1");
        let vertex_value_1 = String::from("Property_1");
        let vertex_1 = Vertex::new(vertex_key_1.clone(), vertex_value_1.into());
        graph.add_or_replace_vertex(vertex_1).unwrap();

        let vertex_key_2 = String::from("Vertex_2");
        let vertex_value_2 = String::from("Property_2");
        let vertex_2 = Vertex::new(vertex_key_2.clone(), vertex_value_2.into());
        graph.add_or_replace_vertex(vertex_2).unwrap();

        let index_vertex_1 = graph
            .vertex_key_to_vertex_index_map
            .get(vertex_key_1.as_str())
            .unwrap();
        assert_eq!(
            graph
                .vertex_index_to_vertex_key_ref(index_vertex_1.clone())
                .unwrap(),
            vertex_key_1
        );

        let index_vertex_2 = graph
            .vertex_key_to_vertex_index_map
            .get(vertex_key_2.as_str())
            .unwrap();
        assert_eq!(
            graph
                .vertex_index_to_vertex_key_ref(index_vertex_2.clone())
                .unwrap(),
            vertex_key_2
        );
    }

    #[test]
    fn test_convert_edge_type_index_to_edge_type_key_ref() {
        let mut graph = Graph::new(10, 20).unwrap();

        let edge_type_key_1 = String::from("Vertex_1");
        let adjacency_matrix_1 = AdjacencyMatrix::new(
            graph.graphblas_context_ref(),
            edge_type_key_1.clone(),
            graph.vertex_capacity().unwrap(),
        )
        .unwrap();

        let index_edge_type_1: EdgeTypeIndex = graph
            .adjacency_matrices_mut_ref()
            .push(adjacency_matrix_1)
            .unwrap()
            .into();
        assert_eq!(
            graph
                .edge_type_index_to_edge_type_ref(index_edge_type_1)
                .unwrap(),
            edge_type_key_1.as_str()
        )
    }
}
