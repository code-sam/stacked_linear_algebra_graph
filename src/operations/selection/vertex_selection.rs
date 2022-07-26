use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::extract::SubVectorExtractor;
use graphblas_sparse_linear_algebra::operators::insert::{
    InsertScalarIntoVector, InsertScalarIntoVectorTrait,
};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::{
    element_wise_multiplication::ElementWiseVectorMultiplicationMonoidOperator, monoid::LogicalAnd,
};
use graphblas_sparse_linear_algebra::util::ElementIndexSelector;
use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
    GetVectorElementList, SparseVector, VectorElementList,
};

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::vertex::{Vertex, VertexIndex, VertexValue};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_OPERATOR_OPTIONS_TO_USE_MASK_COMPLEMENT: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new(false, false, true, false, false));

static GRAPHBLAS_SUB_VECTOR_EXTRACTOR: Lazy<SubVectorExtractor<bool, bool>> =
    Lazy::new(|| SubVectorExtractor::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, None));

static GRAPHBLAS_SCALAR_INTO_VECTOR_INSERTER_WITH_MASK_COMPLEMENT: Lazy<
    InsertScalarIntoVector<bool, bool>,
> = Lazy::new(|| {
    InsertScalarIntoVector::<bool, bool>::new(
        &GRAPHBLAS_OPERATOR_OPTIONS_TO_USE_MASK_COMPLEMENT,
        None,
    )
});

static GRAPHBLAS_VECTOR_AND_OPERATOR: Lazy<ElementWiseVectorMultiplicationMonoidOperator<bool>> =
    Lazy::new(|| {
        ElementWiseVectorMultiplicationMonoidOperator::<bool>::new(
            &LogicalAnd::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            None,
        )
    });

// pub trait VertexSelectionTrait {
//     fn select_vertex_ref(&self) -> Result<Vec<&Vertex>, GraphComputingError>;
//     // fn select_vertex_index_ref(&self) -> Result<Vec<VertexIndex>, GraphComputingError>;
//     fn select_vertex_key_ref(&self) -> Result<Vec<&str>, GraphComputingError>;
//     fn select_vertex_value_ref(&self) -> Result<Vec<&VertexValue>, GraphComputingError>;
// }

#[derive(Clone, Debug)]
pub struct VertexSelection<'g> {
    graph: &'g Graph,
    vertex_mask: SparseVector<bool>,
}

impl<'g> VertexSelection<'g> {
    pub(crate) fn new(
        graph: &'g Graph,
        vertex_mask: SparseVector<bool>,
    ) -> Result<Self, GraphComputingError> {
        #[cfg(debug_assertions)]
        let graph_vertex_capacity = graph.vertex_capacity()?;
        if vertex_mask.length()? != graph_vertex_capacity {
            return Err(LogicError::new(
                LogicErrorType::DimensionMismatch,
                format!(
                    "Length of vertex_mask {:?}, does not match the graph's vertex capacity {:?}",
                    vertex_mask.length()?,
                    graph_vertex_capacity
                ),
                None,
            )
            .into());
        }

        Ok(Self { graph, vertex_mask })
    }

    pub(crate) fn vertex_mask_ref(&self) -> &SparseVector<bool> {
        &self.vertex_mask
    }

    pub(crate) fn vertex_mask_mut_ref(&mut self) -> &mut SparseVector<bool> {
        &mut self.vertex_mask
    }

    pub(crate) fn to_full_vertex_mask(&self) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut full_vertex_mask = self.vertex_mask.clone();
        GRAPHBLAS_SCALAR_INTO_VECTOR_INSERTER_WITH_MASK_COMPLEMENT.apply_with_mask(
            &mut full_vertex_mask,
            &ElementIndexSelector::Index(
                &self
                    .graph
                    .index_mask_with_all_vertices()
                    .get_element_list()?
                    .indices_ref()
                    .to_owned(),
            ),
            &false,
            &self.vertex_mask, // TODO: technically, the complement of false can cause unnecessary updates
        )?;
        Ok(full_vertex_mask)
    }

    /// The mask selects which empty values shall be set to false. The mask does not remove elements already in the selection.
    pub(crate) fn to_full_vertex_mask_with_mask(
        &self,
        mask: &SparseVector<bool>,
    ) -> Result<SparseVector<bool>, GraphComputingError> {
        let mut filtered_mask = SparseVector::new(
            self.graph_ref().graphblas_context_ref(),
            &self.graph_ref().vertex_capacity()?,
        )?;
        GRAPHBLAS_VECTOR_AND_OPERATOR.apply(
            self.graph_ref().index_mask_with_all_vertices(),
            mask,
            &mut filtered_mask,
        )?;

        let mut filtered_vertex_mask = SparseVector::new(
            self.graph_ref().graphblas_context_ref(),
            &self.graph_ref().vertex_capacity()?,
        )?;
        GRAPHBLAS_VECTOR_AND_OPERATOR.apply(&self.vertex_mask, mask, &mut filtered_vertex_mask)?;

        let mut full_vertex_mask = SparseVector::new(
            self.graph_ref().graphblas_context_ref(),
            &self.graph_ref().vertex_capacity()?,
        )?;
        GRAPHBLAS_VECTOR_AND_OPERATOR.apply(
            &filtered_vertex_mask,
            &filtered_mask,
            &mut full_vertex_mask,
        )?;

        GRAPHBLAS_SCALAR_INTO_VECTOR_INSERTER_WITH_MASK_COMPLEMENT.apply_with_mask(
            &mut full_vertex_mask,
            &ElementIndexSelector::Index(
                &filtered_mask.get_element_list()?.indices_ref().to_owned(),
            ),
            &false,
            &self.vertex_mask,
        )?;

        Ok(full_vertex_mask)
    }

    pub(crate) fn graph_ref(&'g self) -> &'g Graph {
        self.graph
    }

    fn get_selected_elements(&self) -> Result<VectorElementList<bool>, GraphComputingError> {
        let mut selected_vertices_mask = SparseVector::new(
            self.graph.graphblas_context_ref(),
            &self.graph.vertex_capacity()?,
        )?;
        GRAPHBLAS_SUB_VECTOR_EXTRACTOR.apply_with_mask(
            &self.vertex_mask,
            &ElementIndexSelector::All,
            &mut selected_vertices_mask,
            &self.vertex_mask,
        )?;
        Ok(selected_vertices_mask.get_element_list()?)
    }

    pub(crate) fn vertex_indices_ref(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        let selected_vertex_elements = self.get_selected_elements()?;
        let raw_vertex_indices = selected_vertex_elements.indices_ref();

        // TODO: parallelization
        let mut vertex_indices = Vec::with_capacity(raw_vertex_indices.len());
        for index in raw_vertex_indices.into_iter() {
            let vertex_index = VertexIndex::new(index.clone());
            vertex_indices.push(vertex_index);
        }
        Ok(vertex_indices)
    }

    pub fn vertices_ref(&self) -> Result<Vec<&Vertex>, GraphComputingError> {
        let selected_vertex_elements = self.get_selected_elements()?;
        let vertex_indices = selected_vertex_elements.indices_ref();

        let mut selected_vertices = Vec::with_capacity(vertex_indices.len());
        for vertex_index in vertex_indices.into_iter() {
            let selected_vertex = self
                .graph
                .vertex_store_ref()
                .get_ref(VertexIndex::new(vertex_index.clone()));

            match selected_vertex {
                Ok(vertex) => selected_vertices.push(vertex),
                Err(_) => {
                    // TODO: match actual error type
                    return Err(LogicError::new(
                        LogicErrorType::VertexMustExist,
                        String::from("A vertex was selected that does not exist"),
                        None,
                    )
                    .into());
                }
            }
        }
        Ok(selected_vertices)
    }

    pub fn vertex_keys_ref(&self) -> Result<Vec<&str>, GraphComputingError> {
        let selected_vertex_elements = self.get_selected_elements()?;
        let vertex_indices = selected_vertex_elements.indices_ref();

        let mut selected_vertex_keys = Vec::with_capacity(vertex_indices.len());
        for vertex_index in vertex_indices.into_iter() {
            let selected_vertex = self
                .graph
                .vertex_store_ref()
                .get_ref(VertexIndex::new(vertex_index.clone()));

            match selected_vertex {
                Ok(vertex) => selected_vertex_keys.push(vertex.key_ref()),
                Err(_) => {
                    // TODO: match actual error type
                    return Err(LogicError::new(
                        LogicErrorType::VertexMustExist,
                        String::from("A vertex was selected that does not exist"),
                        None,
                    )
                    .into());
                }
            }
        }
        Ok(selected_vertex_keys)
    }

    pub fn vertex_values_ref(&self) -> Result<Vec<&VertexValue>, GraphComputingError> {
        let selected_vertex_elements = self.get_selected_elements()?;
        let vertex_indices = selected_vertex_elements.indices_ref();

        let mut selected_vertex_values = Vec::with_capacity(vertex_indices.len());
        for vertex_index in vertex_indices.into_iter() {
            let selected_vertex = self
                .graph
                .vertex_store_ref()
                .get_ref(VertexIndex::new(vertex_index.clone()));

            match selected_vertex {
                Ok(vertex) => selected_vertex_values.push(vertex.value_ref()),
                Err(_) => {
                    // TODO: match actual error type
                    return Err(LogicError::new(
                        LogicErrorType::VertexMustExist,
                        String::from("A vertex was selected that does not exist"),
                        None,
                    )
                    .into());
                }
            }
        }
        Ok(selected_vertex_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::value_types::sparse_vector::GetVectorElementValue;

    use crate::graph::edge::DirectedEdgeDefinedByKeys;
    use crate::graph::graph::GraphTrait;
    use crate::graph::vertex::VertexKeyAndIndexConversion;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::select_vertex::SelectVertex;

    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn test_query_result() {
        let initial_vertex_capacity = 10;
        let initial_edge_type_capacity = 10;
        let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

        let vertex_key_1 = String::from("vertex_1");
        let vertex_value_1 = String::from("value_1").into();

        let vertex_key_2 = String::from("vertex_1");
        let vertex_value_2 = String::from("value_2").into();

        let vertex_1 = Vertex::new(vertex_key_1, vertex_value_1);
        let vertex_2 = Vertex::new(vertex_key_2, vertex_value_2);

        let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_1"),
            vertex_2.clone().into(),
        );
        let edge_vertex2_vertex1 = DirectedEdgeDefinedByKeys::new(
            vertex_2.clone().into(),
            String::from("edge_type_1"),
            vertex_1.clone().into(),
        );
        let edge_vertex1_vertex2_type2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_2"),
            vertex_2.clone().into(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
            .unwrap();

        let vertex_mask =
            SparseVector::new(&graph.graphblas_context_ref(), &initial_vertex_capacity).unwrap();
        let vertex_selection = VertexSelection::new(&graph, vertex_mask).unwrap();

        let vertices = vertex_selection.vertices_ref().unwrap();
        assert_eq!(vertices.len(), 0);
    }

    #[test]
    fn to_vertex_full_mask() {
        let graph = standard_graph_for_testing();

        let negative_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"negative")
            .unwrap();

        let negative_selection_as_full_mask = negative_selection.to_full_vertex_mask().unwrap();
        assert_eq!(
            negative_selection_as_full_mask.length().unwrap(),
            graph.vertex_capacity().unwrap()
        );
        assert_eq!(
            negative_selection_as_full_mask
                .number_of_stored_elements()
                .unwrap(),
            graph.number_of_vertices().unwrap()
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("0".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("-1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            true
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("-1.1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            true
        );
    }

    #[test]
    fn to_vertex_full_mask_with_mask() {
        let graph = standard_graph_for_testing();

        let negative_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"negative")
            .unwrap();

        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let negative_selection_as_full_mask = negative_selection
            .to_full_vertex_mask_with_mask(integer_selection.vertex_mask_ref())
            .unwrap();

        assert_eq!(
            negative_selection_as_full_mask
                .number_of_stored_elements()
                .unwrap(),
            integer_selection
                .vertex_mask_ref()
                .number_of_stored_elements()
                .unwrap()
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("1_duplicate".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("0".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("2".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("-1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            true
        );

        let index = graph
            .vertex_key_ref_to_vertex_index_ref("-1.1".into())
            .unwrap();
        assert_eq!(
            negative_selection_as_full_mask
                .get_element_value(index.index_ref())
                .unwrap(),
            false
        );
    }
}
