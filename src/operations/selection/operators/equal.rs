use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::{
    element_wise_addition::ElementWiseVectorAdditionMonoidOperator, monoid::Equal,
    options::OperatorOptions,
};

use crate::error::GraphComputingError;
use crate::operations::selection::vertex_selection::VertexSelection;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS_WITH_PRE_CLEARED_OUTPUT: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new(true, false, false, false, false));

static GRAPHBLAS_VECTOR_EQUAL_OPERATOR: Lazy<ElementWiseVectorAdditionMonoidOperator<bool>> =
    Lazy::new(|| {
        ElementWiseVectorAdditionMonoidOperator::<bool>::new(
            &Equal::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS_WITH_PRE_CLEARED_OUTPUT,
            None,
        )
    });

pub trait EqualOperator<RightHandSide = Self> {
    type Output;
    fn equal(&self, right_hand_side: &RightHandSide) -> Result<Self::Output, GraphComputingError>;
    fn equal_with_mask(
        &self,
        right_hand_side: &RightHandSide,
        mask: &RightHandSide,
    ) -> Result<Self::Output, GraphComputingError>;
}

impl<'g> EqualOperator for VertexSelection<'g> {
    type Output = VertexSelection<'g>;

    // Without the use of a full matrix, this method may result in non-intuitive behaviour for the user:
    // (empty == empty) => empty
    // (empty == false) => false
    // (true == empty) => true
    // (false == false) => true
    // (true == true) => true
    // (false == true) => false
    // Therefore, the current implementation uses full matrices.

    /// Requires conversion to a full vector, possibly causing high memory usage
    fn equal(&self, right_hand_side: &Self) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_EQUAL_OPERATOR.apply(
            &self.to_full_vertex_mask()?,
            &right_hand_side.to_full_vertex_mask()?,
            resulting_vertex_selection.vertex_mask_mut_ref(),
        )?;
        Ok(resulting_vertex_selection)
    }

    /// The operator applies to all coordinates that the mask selects. Elements in the left-hand-side that the mask does not select remain unchanged.
    /// Requires conversion to a full vector, causing high memory usage. A smaller mask selecting fewer elements will reduce memory usage.
    fn equal_with_mask(
        &self,
        right_hand_side: &Self,
        mask: &Self,
    ) -> Result<Self, GraphComputingError> {
        // TODO: Size checking
        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_EQUAL_OPERATOR.apply_with_mask(
            mask.vertex_mask_ref(),
            &self.to_full_vertex_mask_with_mask(mask.vertex_mask_ref())?,
            &right_hand_side.to_full_vertex_mask_with_mask(mask.vertex_mask_ref())?,
            // &mut resulting_vertex_selection,
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
    fn test_equal_operator_for_vertex_selection() {
        let graph = standard_graph_for_testing();

        let positive_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"positive")
            .unwrap();
        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let positive_and_integer_or_neiter_selection =
            positive_selection.equal(&integer_selection).unwrap();

        let mut positive_and_integer_or_neiter = positive_and_integer_or_neiter_selection
            .vertex_values_ref()
            .unwrap();

        positive_and_integer_or_neiter.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        ();
        positive_and_integer_or_neiter.dedup();

        assert_eq!(positive_and_integer_or_neiter.len(), 10);
        assert!(positive_and_integer_or_neiter.contains(&&VertexValue::FloatingPoint32Bit(-1.1)));
        assert!(positive_and_integer_or_neiter.contains(&&VertexValue::UnsignedInteger8Bit(1)));
        assert!(positive_and_integer_or_neiter.contains(&&VertexValue::UnsignedInteger8Bit(2)));

        assert!(
            positive_and_integer_or_neiter.contains(&&VertexValue::String(String::from("string")))
        );
        assert!(
            positive_and_integer_or_neiter.contains(&&VertexValue::String(String::from("integer")))
        );
        assert!(positive_and_integer_or_neiter
            .contains(&&VertexValue::String(String::from("natural_number"))));
        assert!(positive_and_integer_or_neiter
            .contains(&&VertexValue::String(String::from("real_number"))));
        assert!(positive_and_integer_or_neiter
            .contains(&&VertexValue::String(String::from("negative"))));
        assert!(positive_and_integer_or_neiter
            .contains(&&VertexValue::String(String::from("positive"))));
        assert!(positive_and_integer_or_neiter
            .contains(&&VertexValue::String(String::from("not_a_number"))));
    }

    #[test]
    fn test_equal_operator_for_vertex_selection_with_mask() {
        let graph = standard_graph_for_testing();

        let positive_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("sign"), &"positive")
            .unwrap();
        let larger_than_one_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("larger_than"), &"1_duplicate")
            .unwrap();
        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let resulting_selection = positive_selection
            .equal_with_mask(&larger_than_one_selection, &integer_selection)
            .unwrap();
        let mut resulting_values = resulting_selection.vertex_values_ref().unwrap();

        resulting_values.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        ();
        resulting_values.dedup();

        assert_eq!(resulting_values.len(), 3);
        assert!(resulting_values.contains(&&VertexValue::Integer8Bit(-1)));
        assert!(resulting_values.contains(&&VertexValue::UnsignedInteger8Bit(0)));
        assert!(resulting_values.contains(&&VertexValue::UnsignedInteger8Bit(2)));
    }
}
