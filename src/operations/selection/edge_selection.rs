use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::extract::{
    MatrixColumnExtractor, MatrixRowExtractor,
};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::util::ElementIndexSelector;
use graphblas_sparse_linear_algebra::value_types::sparse_matrix::Size;
use graphblas_sparse_linear_algebra::value_types::sparse_vector::SparseVector;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::adjacency_matrix::AdjacencyMatrix;
use crate::graph::edge::{EdgeType, EdgeTypeRef};
use crate::graph::graph::{EdgeTypeIndex, Graph, VertexIndex};
use crate::graph::vertex::{Vertex, VertexKeyRef};

use super::vertex_selection::VertexSelection;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_MATRIX_COLUMN_EXTRACTOR: Lazy<MatrixColumnExtractor<bool, bool>> =
    Lazy::new(|| {
        MatrixColumnExtractor::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, None)
    });

static GRAPHBLAS_MATRIX_ROW_EXTRACTOR: Lazy<MatrixRowExtractor<bool, bool>> =
    Lazy::new(|| MatrixRowExtractor::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, None));

static GRAPHBLAS_ALL_ELEMENTS_SELECTOR: Lazy<ElementIndexSelector> =
    Lazy::new(|| ElementIndexSelector::All);

// #[derive(Debug)]
pub struct EdgeSelection<'g> {
    graph: &'g Graph,
    edge_type_index: EdgeTypeIndex,
    adjacency_matrix_mask: AdjacencyMatrix,
}

impl<'g> EdgeSelection<'g> {
    pub(crate) fn new(
        graph: &'g Graph,
        edge_type_index: EdgeTypeIndex,
        adjacency_matrix_mask: AdjacencyMatrix,
    ) -> Result<Self, GraphComputingError> {
        #[cfg(debug_assertions)]
        check_adjancency_matrix_size(graph, &adjacency_matrix_mask)?;

        Ok(Self {
            graph,
            edge_type_index,
            adjacency_matrix_mask,
        })
    }

    pub(crate) fn new_for_edge_type(
        graph: &'g Graph,
        edge_type: EdgeType,
        adjacency_matrix_mask: AdjacencyMatrix,
    ) -> Result<Self, GraphComputingError> {
        let edge_type_index;
        match graph.edge_type_to_edge_type_index_map_ref().get(&edge_type) {
            Some(index) => edge_type_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::EdgeTypeMustExist,
                    format!("EdgeType {:?} does not exist", edge_type),
                    None,
                )
                .into());
            }
        };

        EdgeSelection::new(graph, edge_type_index.clone(), adjacency_matrix_mask)
    }

    pub fn edge_type_index_ref(&self) -> &EdgeTypeIndex {
        &self.edge_type_index
    }

    pub fn edge_type_ref(&self) -> Result<&EdgeTypeRef, GraphComputingError> {
        // TODO: review if it is more efficient to store the EdgeType directly
        match self
            .graph
            .adjacency_matrices_ref()
            .get_ref(self.edge_type_index.clone())
        {
            Ok(adjacency_matrix) => Ok(adjacency_matrix.edge_type_ref()),
            Err(_) => Err(LogicError::new(
                // TODO: technically, another system could have occured
                LogicErrorType::IndexOutOfBounds,
                format!(
                    "No Adjacency matrix for EdgeTypeIndex {:?}",
                    self.edge_type_index
                ),
                None,
            )
            .into()),
        }
    }

    pub(crate) fn adjacency_matrix_mask_ref(&self) -> &AdjacencyMatrix {
        &self.adjacency_matrix_mask
    }

    // pub(crate) fn edge_type_ref(&self) -> &EdgeType {
    //     self.adjacency_matrix_mask.edge_type_ref()
    // }

    pub fn get_from_vertices(&self) -> Result<Vec<Vertex>, GraphComputingError> {
        let from_vertex_indices = self.adjacency_matrix_mask.get_from_vertex_indices()?;
        self.get_vertices_for_vertex_indices(from_vertex_indices)
    }

    pub fn get_to_vertices(&self) -> Result<Vec<Vertex>, GraphComputingError> {
        let to_vertex_indices = self.adjacency_matrix_mask.get_to_vertex_indices()?;
        self.get_vertices_for_vertex_indices(to_vertex_indices)
    }

    pub fn get_vertices(&self) -> Result<Vec<Vertex>, GraphComputingError> {
        let vertices = self.adjacency_matrix_mask.get_vertex_indices()?;
        self.get_vertices_for_vertex_indices(vertices)
    }

    fn get_vertices_for_vertex_indices(
        &self,
        vertex_indices: Vec<VertexIndex>,
    ) -> Result<Vec<Vertex>, GraphComputingError> {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(vertex_indices.len());
        for vertex_index in vertex_indices.into_iter() {
            match self.graph.vertex_store_ref().get_ref(vertex_index) {
                Ok(vertex) => vertices.push(vertex.clone()),
                Err(_) => {
                    // TODO: technically, another system error may have occured
                    return Err(LogicError::new(
                        LogicErrorType::VertexMustExist,
                        format!("Expected a vertex at index {:?}", vertex_index),
                        None,
                    )
                    .into());
                }
            }
        }
        Ok(vertices)
    }

    pub fn select_from_vertices(&self) -> Result<VertexSelection, GraphComputingError> {
        let from_vertex_mask = self.adjacency_matrix_mask.get_from_vertex_index_mask()?;
        VertexSelection::new(self.graph, from_vertex_mask)
    }

    pub fn select_to_vertices(&self) -> Result<VertexSelection, GraphComputingError> {
        let to_vertex_mask = self.adjacency_matrix_mask.get_to_vertex_index_mask()?;
        VertexSelection::new(self.graph, to_vertex_mask)
    }

    pub fn select_vertices(&self) -> Result<VertexSelection, GraphComputingError> {
        let vertex_mask = self.adjacency_matrix_mask.get_vertex_index_mask()?;
        VertexSelection::new(self.graph, vertex_mask)
    }

    pub fn select_vertices_connected_to_vertex(
        &self,
        to_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let to_vertex_index = self
            .graph
            .try_vertex_key_ref_to_vertex_index_ref(to_vertex_key)?;
        self.select_vertices_connected_to_vertex_by_index(to_vertex_index)
    }
    pub fn select_vertices_connected_to_vertex_by_index(
        &self,
        to_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let mut vertex_selection_mask = SparseVector::new(
            self.graph.graphblas_context_ref(),
            &self.graph.vertex_capacity()?,
        )?;

        // TODO: review if extraction is the most efficient method here
        GRAPHBLAS_MATRIX_COLUMN_EXTRACTOR.apply(
            self.adjacency_matrix_mask.as_sparse_matrix(),
            to_vertex_index.index_ref(),
            &GRAPHBLAS_ALL_ELEMENTS_SELECTOR,
            &mut vertex_selection_mask,
        )?;
        VertexSelection::new(self.graph, vertex_selection_mask)
    }

    pub fn select_vertices_connected_from_vertex(
        &self,
        from_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let from_vertex_index = self
            .graph
            .try_vertex_key_ref_to_vertex_index_ref(from_vertex_key)?;
        self.select_vertices_connected_from_vertex_by_index(from_vertex_index)
    }
    pub fn select_vertices_connected_from_vertex_by_index(
        &self,
        from_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let mut vertex_selection_mask = SparseVector::new(
            self.graph.graphblas_context_ref(),
            &self.graph.vertex_capacity()?,
        )?;

        // TODO: review if extraction is the most efficient method here
        GRAPHBLAS_MATRIX_ROW_EXTRACTOR.apply(
            self.adjacency_matrix_mask.as_sparse_matrix(),
            from_vertex_index.index_ref(),
            &GRAPHBLAS_ALL_ELEMENTS_SELECTOR,
            &mut vertex_selection_mask,
        )?;
        VertexSelection::new(self.graph, vertex_selection_mask)
    }
}

fn check_adjancency_matrix_size(
    graph: &Graph,
    adjacency_matrix: &AdjacencyMatrix,
) -> Result<(), GraphComputingError> {
    let graph_vertex_capacity = graph.vertex_capacity()?;
    if adjacency_matrix.size()? == Size::from_tuple((graph_vertex_capacity, graph_vertex_capacity))
    {
        Ok(())
    } else {
        Err(LogicError::new(LogicErrorType::DimensionMismatch, format!("Size of adjacency_matrix_mask {:?}, does not match the graph's vertex capacity {:?}", adjacency_matrix.size()?, graph_vertex_capacity), None).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::DirectedEdge;
    use crate::graph::vertex::VertexValue;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::select_edge_type::EdgeTypeSelectorTrait;
    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn test_select_vertices_connected_to_vertex() {
        let graph = standard_graph_for_testing();

        let smaller_than_edge_selection = graph
            .select_edge_type(String::from("smaller_than"))
            .unwrap();
        let selection_vertices_smaller_than_minus_one = smaller_than_edge_selection
            .select_vertices_connected_to_vertex(&"-1")
            .unwrap();
        let vertices_smaller_than_minus_one = selection_vertices_smaller_than_minus_one
            .vertex_values_ref()
            .unwrap();

        assert_eq!(
            vertices_smaller_than_minus_one,
            vec!(&VertexValue::FloatingPoint32Bit(-1.1))
        );
    }

    #[test]
    fn test_select_vertices_connected_from_vertex() {
        let graph = standard_graph_for_testing();

        let smaller_than_edge_selection =
            graph.select_edge_type(String::from("larger_than")).unwrap();
        let selection_vertices_larger_than_1_dot_2 = smaller_than_edge_selection
            .select_vertices_connected_to_vertex(&"1.2")
            .unwrap();
        let vertices_larger_than_one_dot_two = selection_vertices_larger_than_1_dot_2
            .vertex_values_ref()
            .unwrap();

        assert_eq!(
            vertices_larger_than_one_dot_two,
            vec!(&VertexValue::UnsignedInteger8Bit(2))
        );
    }

    // TODO: reduce code duplication
    #[test]
    fn test_get_from_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1, vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2, vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3, vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_1 = edge_type1_selection.get_from_vertices().unwrap();
        assert_eq!(from_vertices_for_edge_type_1.len(), 3);
        assert!(from_vertices_for_edge_type_1.contains(&vertex_1));
        assert!(from_vertices_for_edge_type_1.contains(&vertex_2));
        assert!(from_vertices_for_edge_type_1.contains(&vertex_3));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_2 = edge_type2_selection.get_from_vertices().unwrap();
        assert_eq!(from_vertices_for_edge_type_2.len(), 2);
        assert!(from_vertices_for_edge_type_2.contains(&vertex_1));
        assert!(from_vertices_for_edge_type_2.contains(&vertex_2));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_3 = edge_type3_selection.get_from_vertices().unwrap();
        assert_eq!(from_vertices_for_edge_type_3.len(), 2);
        assert!(from_vertices_for_edge_type_3.contains(&vertex_1));
        assert!(from_vertices_for_edge_type_3.contains(&vertex_3));
    }

    #[test]
    fn test_get_to_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1, vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2, vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3, vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_1 = edge_type1_selection.get_to_vertices().unwrap();
        assert_eq!(to_vertices_for_edge_type_1.len(), 3);
        assert!(to_vertices_for_edge_type_1.contains(&vertex_1));
        assert!(to_vertices_for_edge_type_1.contains(&vertex_2));
        assert!(to_vertices_for_edge_type_1.contains(&vertex_3));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_2 = edge_type2_selection.get_to_vertices().unwrap();
        assert_eq!(to_vertices_for_edge_type_2.len(), 2);
        assert!(to_vertices_for_edge_type_2.contains(&vertex_1));
        assert!(to_vertices_for_edge_type_2.contains(&vertex_2));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_3 = edge_type3_selection.get_to_vertices().unwrap();
        assert_eq!(to_vertices_for_edge_type_3.len(), 2);
        assert!(to_vertices_for_edge_type_3.contains(&vertex_2));
        assert!(to_vertices_for_edge_type_3.contains(&vertex_3));
    }

    #[test]
    fn test_get_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1, vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2, vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3, vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_1 = edge_type1_selection.get_vertices().unwrap();
        assert_eq!(vertices_for_edge_type_1.len(), 3);
        assert!(vertices_for_edge_type_1.contains(&vertex_1));
        assert!(vertices_for_edge_type_1.contains(&vertex_2));
        assert!(vertices_for_edge_type_1.contains(&vertex_3));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_2 = edge_type2_selection.get_vertices().unwrap();
        assert_eq!(vertices_for_edge_type_2.len(), 2);
        assert!(vertices_for_edge_type_2.contains(&vertex_1));
        assert!(vertices_for_edge_type_2.contains(&vertex_2));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_3 = edge_type3_selection.get_vertices().unwrap();
        assert_eq!(vertices_for_edge_type_3.len(), 3);
        assert!(vertices_for_edge_type_3.contains(&vertex_1));
        assert!(vertices_for_edge_type_3.contains(&vertex_2));
        assert!(vertices_for_edge_type_3.contains(&vertex_3));
    }

    #[test]
    fn test_select_from_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1.clone(), vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2.clone(), vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3.clone(), vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_1 = edge_type1_selection.select_from_vertices().unwrap();
        let from_vertex_keys_for_edge_type_1 =
            from_vertices_for_edge_type_1.vertex_keys_ref().unwrap();
        assert_eq!(from_vertex_keys_for_edge_type_1.len(), 3);
        assert!(from_vertex_keys_for_edge_type_1.contains(&vertex_key_1.as_str()));
        assert!(from_vertex_keys_for_edge_type_1.contains(&vertex_key_2.as_str()));
        assert!(from_vertex_keys_for_edge_type_1.contains(&vertex_key_3.as_str()));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_2 = edge_type2_selection.select_from_vertices().unwrap();
        let from_vertex_keys_for_edge_type_2 =
            from_vertices_for_edge_type_2.vertex_keys_ref().unwrap();
        assert_eq!(from_vertex_keys_for_edge_type_2.len(), 2);
        assert!(from_vertex_keys_for_edge_type_2.contains(&vertex_key_1.as_str()));
        assert!(from_vertex_keys_for_edge_type_2.contains(&vertex_key_2.as_str()));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let from_vertices_for_edge_type_3 = edge_type3_selection.select_from_vertices().unwrap();
        let from_vertex_keys_for_edge_type_3 =
            from_vertices_for_edge_type_3.vertex_keys_ref().unwrap();
        assert_eq!(from_vertex_keys_for_edge_type_3.len(), 2);
        assert!(from_vertex_keys_for_edge_type_3.contains(&vertex_key_1.as_str()));
        assert!(from_vertex_keys_for_edge_type_3.contains(&vertex_key_3.as_str()));
    }

    #[test]
    fn test_select_to_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1.clone(), vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2.clone(), vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3.clone(), vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_1 = edge_type1_selection.select_to_vertices().unwrap();
        let to_vertex_keys_for_edge_type_1 = to_vertices_for_edge_type_1.vertex_keys_ref().unwrap();
        assert_eq!(to_vertex_keys_for_edge_type_1.len(), 3);
        assert!(to_vertex_keys_for_edge_type_1.contains(&vertex_key_1.as_str()));
        assert!(to_vertex_keys_for_edge_type_1.contains(&vertex_key_2.as_str()));
        assert!(to_vertex_keys_for_edge_type_1.contains(&vertex_key_3.as_str()));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_2 = edge_type2_selection.select_to_vertices().unwrap();
        let to_vertex_keys_for_edge_type_2 = to_vertices_for_edge_type_2.vertex_keys_ref().unwrap();
        assert_eq!(to_vertex_keys_for_edge_type_2.len(), 2);
        assert!(to_vertex_keys_for_edge_type_2.contains(&vertex_key_1.as_str()));
        assert!(to_vertex_keys_for_edge_type_2.contains(&vertex_key_2.as_str()));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let to_vertices_for_edge_type_3 = edge_type3_selection.select_to_vertices().unwrap();
        let to_vertex_keys_for_edge_type_3 = to_vertices_for_edge_type_3.vertex_keys_ref().unwrap();
        assert_eq!(to_vertex_keys_for_edge_type_3.len(), 2);
        assert!(to_vertex_keys_for_edge_type_3.contains(&vertex_key_2.as_str()));
        assert!(to_vertex_keys_for_edge_type_3.contains(&vertex_key_3.as_str()));
    }

    #[test]
    fn test_select_vertices() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_2");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_key_3 = String::from("vertex_3");
        let vertex_value_3 = String::from("value_3").into();

        let vertex_1 = Vertex::new(vertex_key_1.clone(), vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2.clone(), vertex_value_2);
        let vertex_3 = Vertex::new(vertex_key_3.clone(), vertex_value_3);

        let edge_type_1 = String::from("edge_type_1");
        let edge_type_2 = String::from("edge_type_2");
        let edge_type_3 = String::from("edge_type_3");

        let edge_vertex1_vertex2_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex2_vertex1_type1 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex1_vertex3_type1 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_1.clone(),
        );
        let edge_vertex3_vertex2_type1 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_1.clone(),
        );

        let edge_vertex1_vertex2_type2 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_2.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex2_vertex1_type2 = DirectedEdge::new(
            vertex_2.clone().into(),
            vertex_1.clone().into(),
            edge_type_2.clone(),
        );
        let edge_vertex1_vertex3_type3 = DirectedEdge::new(
            vertex_1.clone().into(),
            vertex_3.clone().into(),
            edge_type_3.clone(),
        );
        let edge_vertex3_vertex2_type3 = DirectedEdge::new(
            vertex_3.clone().into(),
            vertex_2.clone().into(),
            edge_type_3.clone(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_3.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type1.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex3_type1.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type1.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();
        graph.add_edge(edge_vertex2_vertex1_type2.clone()).unwrap();
        graph.add_edge(edge_vertex1_vertex2_type2.clone()).unwrap();

        graph.add_edge(edge_vertex1_vertex3_type3.clone()).unwrap();
        graph.add_edge(edge_vertex3_vertex2_type3.clone()).unwrap();

        let edge_type_1_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_1.as_str())
            .unwrap();
        let edge_type1_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_1.clone(),
            edge_type_1_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_1 = edge_type1_selection.select_vertices().unwrap();
        let vertex_keys_for_edge_type_1 = vertices_for_edge_type_1.vertex_keys_ref().unwrap();
        assert_eq!(vertex_keys_for_edge_type_1.len(), 3);
        assert!(vertex_keys_for_edge_type_1.contains(&vertex_key_1.as_str()));
        assert!(vertex_keys_for_edge_type_1.contains(&vertex_key_2.as_str()));
        assert!(vertex_keys_for_edge_type_1.contains(&vertex_key_3.as_str()));

        let edge_type_2_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_2.as_str())
            .unwrap();
        let edge_type2_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_2.clone(),
            edge_type_2_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_2 = edge_type2_selection.select_vertices().unwrap();
        let vertex_keys_for_edge_type_2 = vertices_for_edge_type_2.vertex_keys_ref().unwrap();
        assert_eq!(vertex_keys_for_edge_type_2.len(), 2);
        assert!(vertex_keys_for_edge_type_2.contains(&vertex_key_1.as_str()));
        assert!(vertex_keys_for_edge_type_2.contains(&vertex_key_2.as_str()));

        let edge_type_3_adjacency_matrix_mask = graph
            .get_edge_adjacency_matrix_ref(edge_type_3.as_str())
            .unwrap();
        let edge_type3_selection = EdgeSelection::new_for_edge_type(
            &graph,
            edge_type_3.clone(),
            edge_type_3_adjacency_matrix_mask.clone(),
        )
        .unwrap();

        let vertices_for_edge_type_3 = edge_type3_selection.select_vertices().unwrap();
        let vertex_keys_for_edge_type_3 = vertices_for_edge_type_3.vertex_keys_ref().unwrap();
        assert_eq!(vertex_keys_for_edge_type_3.len(), 3);
        assert!(vertex_keys_for_edge_type_3.contains(&vertex_key_1.as_str()));
        assert!(vertex_keys_for_edge_type_3.contains(&vertex_key_2.as_str()));
        assert!(vertex_keys_for_edge_type_3.contains(&vertex_key_3.as_str()));
    }
}
