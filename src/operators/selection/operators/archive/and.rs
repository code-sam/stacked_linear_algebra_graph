use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::{
    element_wise_multiplication::ElementWiseVectorMultiplicationMonoidOperator, monoid::LogicalAnd,
    options::OperatorOptions,
};

use crate::error::GraphComputingError;
use crate::operations::selection::vertex_selection::VertexSelection;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_VECTOR_AND_OPERATOR: Lazy<ElementWiseVectorMultiplicationMonoidOperator<bool>> =
    Lazy::new(|| {
        ElementWiseVectorMultiplicationMonoidOperator::<bool>::new(
            &LogicalAnd::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            None,
        )
    });

pub trait AndOperator<RightHandSide = Self> {
    type Output;
    fn and(&self, right_hand_side: &RightHandSide) -> Result<Self::Output, GraphComputingError>;
    // TODO: consider introducing a selection/exclusion mask for improved API clarity
    fn and_with_mask(
        &self,
        right_hand_side: &RightHandSide,
        mask: &RightHandSide,
    ) -> Result<Self::Output, GraphComputingError>;
}

impl<'g> AndOperator for VertexSelection<'g> {
    type Output = VertexSelection<'g>;

    fn and(&self, right_hand_side: &Self) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_AND_OPERATOR.apply(
            self.vertex_mask_ref(),
            right_hand_side.vertex_mask_ref(),
            resulting_vertex_selection.vertex_mask_mut_ref(),
        )?;
        Ok(resulting_vertex_selection)
    }

    /// The operator applies to all coordinates that the mask selects. Elements in the left-hand-side that the mask does not select remain unchanged.
    fn and_with_mask(
        &self,
        right_hand_side: &Self,
        mask: &Self,
    ) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_AND_OPERATOR.apply_with_mask(
            mask.vertex_mask_ref(),
            self.vertex_mask_ref(),
            right_hand_side.vertex_mask_ref(),
            resulting_vertex_selection.vertex_mask_mut_ref(),
        )?;
        Ok(resulting_vertex_selection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::vertex::VertexValue;
    use crate::operations::select_vertex::SelectVertex;

    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn test_and_operator_for_vertex_selection() {
        let graph = standard_graph_for_testing();

        let negative_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"negative")
            .unwrap();
        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let negative_integer_selection = negative_selection.and(&integer_selection).unwrap();
        let negative_integers = negative_integer_selection.vertex_values_ref().unwrap();

        assert_eq!(negative_integers, vec!(&VertexValue::Integer8Bit(-1)));
    }

    #[test]
    fn test_and_operator_for_vertex_selection_with_mask() {
        let graph = standard_graph_for_testing();

        let negative_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"negative")
            .unwrap();
        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();
        let real_number_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"real_number")
            .unwrap();

        let selection_of_real_numbers_without_positive_integers = real_number_selection
            .and_with_mask(&negative_selection, &integer_selection)
            .unwrap();
        let real_numbers_without_positive_integers =
            selection_of_real_numbers_without_positive_integers
                .vertex_values_ref()
                .unwrap();

        assert_eq!(real_numbers_without_positive_integers.len(), 4);
        assert!(real_numbers_without_positive_integers.contains(&&VertexValue::Integer8Bit(-1)));
        assert!(real_numbers_without_positive_integers
            .contains(&&VertexValue::FloatingPoint32Bit(-1.1)));
        assert!(
            real_numbers_without_positive_integers.contains(&&VertexValue::FloatingPoint32Bit(1.1))
        );
        assert!(
            real_numbers_without_positive_integers.contains(&&VertexValue::FloatingPoint32Bit(1.2))
        );
    }
}
