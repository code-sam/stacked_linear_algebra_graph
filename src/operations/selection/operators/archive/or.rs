use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::{
    element_wise_addition::ElementWiseVectorAdditionMonoidOperator, monoid::LogicalOr,
    options::OperatorOptions,
};

use crate::error::GraphComputingError;

use crate::operations::selection::vertex_selection::VertexSelection;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static GRAPHBLAS_VECTOR_OR_OPERATOR: Lazy<ElementWiseVectorAdditionMonoidOperator<bool>> =
    Lazy::new(|| {
        ElementWiseVectorAdditionMonoidOperator::<bool>::new(
            &LogicalOr::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            None,
        )
    });

pub trait OrOperator<RightHandSide = Self> {
    type Output;
    fn or(&self, right_hand_side: &RightHandSide) -> Result<Self::Output, GraphComputingError>;
    fn or_with_mask(
        &self,
        right_hand_side: &RightHandSide,
        mask: &RightHandSide,
    ) -> Result<Self::Output, GraphComputingError>;
}

impl<'g> OrOperator for VertexSelection<'g> {
    type Output = VertexSelection<'g>;

    fn or(&self, right_hand_side: &Self) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_OR_OPERATOR.apply(
            self.vertex_mask_ref(),
            right_hand_side.vertex_mask_ref(),
            resulting_vertex_selection.vertex_mask_mut_ref(),
        )?;
        Ok(resulting_vertex_selection)
    }

    /// The operator applies to all coordinates that the mask selects. Elements in the left-hand-side that the mask does not select remain unchanged.
    // TODO: consider introducing a selection/exclusion mask for improved API clarity
    fn or_with_mask(
        &self,
        right_hand_side: &Self,
        mask: &Self,
    ) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_OR_OPERATOR.apply_with_mask(
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
    fn test_or_operator_for_vertex_selection() {
        let graph = standard_graph_for_testing();

        let larger_than_one_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("larger_than"), &"1")
            .unwrap();
        let equal_to_one_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("equal_to"), &"1_duplicate")
            .unwrap();

        let larger_than_or_equal_to_one_selection = larger_than_one_selection
            .or(&equal_to_one_selection)
            .unwrap();
        let mut larger_than_or_equal_to_one = larger_than_or_equal_to_one_selection
            .vertex_values_ref()
            .unwrap();

        larger_than_or_equal_to_one.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        ();
        larger_than_or_equal_to_one.dedup();

        assert_eq!(larger_than_or_equal_to_one.len(), 4);
        assert!(larger_than_or_equal_to_one.contains(&&VertexValue::UnsignedInteger8Bit(1)));
        assert!(larger_than_or_equal_to_one.contains(&&VertexValue::FloatingPoint32Bit(1.1)));
        assert!(larger_than_or_equal_to_one.contains(&&VertexValue::FloatingPoint32Bit(1.2)));
        assert!(larger_than_or_equal_to_one.contains(&&VertexValue::UnsignedInteger8Bit(2)));
    }

    #[test]
    fn test_or_operator_for_vertex_selection_with_mask() {
        let graph = standard_graph_for_testing();

        let larger_than_one_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("larger_than"), &"1")
            .unwrap();
        let equal_to_one_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("equal_to"), &"1_duplicate")
            .unwrap();
        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let selection_of_integers_equal_to_or_larger_than_one = equal_to_one_selection
            .or_with_mask(&larger_than_one_selection, &integer_selection)
            .unwrap();
        let mut integers_equal_to_or_larger_than_one =
            selection_of_integers_equal_to_or_larger_than_one
                .vertex_values_ref()
                .unwrap();

        integers_equal_to_or_larger_than_one.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        ();
        integers_equal_to_or_larger_than_one.dedup();

        assert_eq!(integers_equal_to_or_larger_than_one.len(), 2);
        assert!(
            integers_equal_to_or_larger_than_one.contains(&&VertexValue::UnsignedInteger8Bit(1))
        );
        assert!(
            integers_equal_to_or_larger_than_one.contains(&&VertexValue::UnsignedInteger8Bit(2))
        );
    }
}
