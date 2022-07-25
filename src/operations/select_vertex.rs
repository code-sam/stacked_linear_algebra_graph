use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::binary_operator::Second;
use graphblas_sparse_linear_algebra::util::ElementIndex;
use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
    FromVectorElementList, SetVectorElement, SparseVector, VectorElement, VectorElementList,
};

use crate::error::GraphComputingError;

use crate::graph::edge::{EdgeType, EdgeTypeIndex};
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::vertex::{VertexIndex, VertexKeyAndIndexConversion, VertexKeyRef};
use crate::operations::selection::vertex_selection::VertexSelection;

use super::select_edge_type::EdgeTypeSelectorTrait;

static SECOND_BINARY_OPERATOR: Lazy<Second<bool, bool, bool>> =
    Lazy::new(|| Second::<bool, bool, bool>::new());

pub trait SelectVertex {
    fn select_vertex_by_key(
        &self,
        vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection, GraphComputingError>;
    fn select_vertex_by_index(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<VertexSelection, GraphComputingError>;

    fn select_vertices_by_key(
        &self,
        vertex_keys: Vec<&VertexKeyRef>,
    ) -> Result<VertexSelection, GraphComputingError>;
    fn select_vertices_by_index(
        &self,
        vertex_indices: Vec<VertexIndex>,
    ) -> Result<VertexSelection, GraphComputingError>;

    fn select_vertices_connected_to_vertex_by_key(
        &self,
        edge_type: EdgeType,
        to_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection, GraphComputingError>;
    fn select_vertices_connected_to_vertex_by_index(
        &self,
        edge_type: EdgeTypeIndex,
        to_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection, GraphComputingError>;

    fn select_vertices_connected_from_vertex_by_key(
        &self,
        edge_type: EdgeType,
        from_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection, GraphComputingError>;
    fn select_vertices_connected_from_vertex_by_index(
        &self,
        edge_type: EdgeTypeIndex,
        from_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection, GraphComputingError>;
}

impl SelectVertex for Graph {
    fn select_vertex_by_key(
        &self,
        vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection, GraphComputingError> {
        let mut vertex_mask =
            SparseVector::<bool>::new(self.graphblas_context_ref(), &self.vertex_capacity()?)?;
        match self.vertex_key_ref_to_vertex_index_ref(vertex_key) {
            Ok(index) => vertex_mask.set_element(VectorElement::new(index.index(), true))?,
            Err(_) => (),
        };
        VertexSelection::new(self, vertex_mask)
    }
    fn select_vertex_by_index(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<VertexSelection, GraphComputingError> {
        let mut vertex_mask =
            SparseVector::<bool>::new(self.graphblas_context_ref(), &self.vertex_capacity()?)?;
        vertex_mask.set_element(VectorElement::new(vertex_index.index(), true))?;
        VertexSelection::new(self, vertex_mask)
    }

    fn select_vertices_by_key(
        &self,
        vertex_keys: Vec<&VertexKeyRef>,
    ) -> Result<VertexSelection, GraphComputingError> {
        let mut vertex_indices: Vec<ElementIndex> = vec![0; vertex_keys.len()]; // TODO: review id pre-allocation actually improves performance
        let mut mask_values: Vec<bool> = vec![true; vertex_keys.len()];

        // TODO: consider to parallelize
        (0..vertex_keys.len())
            .into_iter()
            .for_each(|vertex_key_index| {
                match self.vertex_key_ref_to_vertex_index_ref(vertex_keys[vertex_key_index]) {
                    Ok(vertex_index) => vertex_indices[vertex_key_index] = vertex_index.index(),
                    Err(_) => mask_values[vertex_key_index] = false,
                }
            });

        let mask_elements = VectorElementList::from_vectors(vertex_indices, mask_values)?;
        let selection_mask = SparseVector::from_element_list(
            self.graphblas_context_ref(),
            &self.vertex_capacity()?,
            &mask_elements,
            &*SECOND_BINARY_OPERATOR,
        )?;
        VertexSelection::new(self, selection_mask)
    }
    fn select_vertices_by_index(
        &self,
        vertex_indices: Vec<VertexIndex>,
    ) -> Result<VertexSelection, GraphComputingError> {
        let mask_values: Vec<bool> = vec![true; vertex_indices.len()];

        // TODO: is this efficient? https://stackoverflow.com/questions/48308759/how-do-i-convert-a-vect-to-a-vecu-without-copying-the-vector
        // Or should the VertexIndex be refactored to a type alias of usize for better performance?
        let element_indices: Vec<ElementIndex> = vertex_indices
            .into_iter()
            .map(|index| index.index())
            .collect();

        let mask_elements = VectorElementList::<bool>::from_vectors(element_indices, mask_values)?;
        let selection_mask = SparseVector::from_element_list(
            self.graphblas_context_ref(),
            &self.vertex_capacity()?,
            &mask_elements,
            &*SECOND_BINARY_OPERATOR,
        )?;
        VertexSelection::new(self, selection_mask)
    }

    fn select_vertices_connected_to_vertex_by_key<'g>(
        &'g self,
        edge_type: EdgeType,
        to_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let edge_selection = self.select_edge_type(edge_type)?;
        edge_selection.select_vertices_connected_to_vertex(to_vertex_key)
    }
    fn select_vertices_connected_to_vertex_by_index<'g>(
        &'g self,
        edge_type: EdgeTypeIndex,
        to_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let edge_selection = self.select_edge_type_by_index(edge_type)?;
        edge_selection.select_vertices_connected_to_vertex_by_index(to_vertex_index)
    }

    fn select_vertices_connected_from_vertex_by_key<'g>(
        &'g self,
        edge_type: EdgeType,
        to_vertex_key: &VertexKeyRef,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let edge_selection = self.select_edge_type(edge_type)?;
        edge_selection.select_vertices_connected_from_vertex(to_vertex_key)
    }
    fn select_vertices_connected_from_vertex_by_index<'g>(
        &'g self,
        edge_type: EdgeTypeIndex,
        to_vertex_index: &VertexIndex,
    ) -> Result<VertexSelection<'g>, GraphComputingError> {
        let edge_selection = self.select_edge_type_by_index(edge_type)?;
        edge_selection.select_vertices_connected_from_vertex_by_index(to_vertex_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::vertex::VertexValue;
    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn select_vertex_by_key() {
        let graph = standard_graph_for_testing();

        let selection = graph.select_vertex_by_key("1").unwrap();
        let selected_values = selection.vertex_values_ref().unwrap();
        assert_eq!(selected_values, vec![&VertexValue::UnsignedInteger8Bit(1)])
    }

    #[test]
    fn select_vertex_by_index() {
        let graph = standard_graph_for_testing();

        let selection = graph.select_vertex_by_key("1").unwrap();
        let indices = selection.vertex_indices_ref().unwrap();

        let selection = graph.select_vertex_by_index(indices[0]).unwrap();
        let selected_values = selection.vertex_values_ref().unwrap();
        assert_eq!(selected_values, vec![&VertexValue::UnsignedInteger8Bit(1)])
    }

    #[test]
    fn select_vertices_by_key() {
        let graph = standard_graph_for_testing();

        let selection = graph.select_vertices_by_key(vec!["1", "2"]).unwrap();
        let selected_values = selection.vertex_values_ref().unwrap();
        assert_eq!(selected_values, vec![&1u8.into(), &2u8.into()])
    }

    #[test]
    fn select_vertices_by_index() {
        let graph = standard_graph_for_testing();

        let selection = graph.select_vertices_by_key(vec!["1", "2"]).unwrap();
        let indices = selection.vertex_indices_ref().unwrap();

        let selection = graph.select_vertices_by_index(indices).unwrap();
        let selected_values = selection.vertex_values_ref().unwrap();
        assert_eq!(selected_values, vec![&1u8.into(), &2u8.into()])
    }

    #[test]
    fn test_select_vertices_connected_to_vertex() {
        let graph = standard_graph_for_testing();

        let selection_vertices_smaller_than_minus_one = graph
            .select_vertices_connected_to_vertex_by_key(String::from("smaller_than"), &"-1")
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

        let selection_vertices_larger_than_1_dot_2 = graph
            .select_vertices_connected_to_vertex_by_key(String::from("larger_than"), &"1.2")
            .unwrap();
        let vertices_larger_than_one_dot_two = selection_vertices_larger_than_1_dot_2
            .vertex_values_ref()
            .unwrap();

        assert_eq!(
            vertices_larger_than_one_dot_two,
            vec!(&VertexValue::UnsignedInteger8Bit(2))
        );
    }
}
