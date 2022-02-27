use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::{
    apply::{UnaryOperatorApplier, UnaryOperatorApplierTrait},
    options::OperatorOptions,
    unary_operator::LogicalNegation,
};

use crate::error::GraphComputingError;
use crate::operations::selection::vertex_selection::VertexSelection;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS_WITH_PRE_CLEARED_OUTPUT: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new(true, false, false, false, false));

static GRAPHBLAS_VECTOR_LOGICAL_NEGATION_OPERATOR: Lazy<UnaryOperatorApplier<bool>> =
    Lazy::new(|| {
        UnaryOperatorApplier::<bool>::new(
            &LogicalNegation::<bool>::new(),
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS_WITH_PRE_CLEARED_OUTPUT,
            None,
        )
    });

pub trait LogicalNegationOperator<RightHandSide = Self> {
    type Output;
    fn not(&self) -> Result<Self::Output, GraphComputingError>;
    fn not_with_mask(&self, mask: &RightHandSide) -> Result<Self::Output, GraphComputingError>;
}

impl<'g> LogicalNegationOperator for VertexSelection<'g> {
    type Output = VertexSelection<'g>;

    fn not(&self) -> Result<Self, GraphComputingError> {
        // TODO: Size checking

        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_LOGICAL_NEGATION_OPERATOR.apply_to_vector(
            &self.to_full_vertex_mask()?,
            resulting_vertex_selection.vertex_mask_mut_ref(),
        )?;
        Ok(resulting_vertex_selection)
    }

    fn not_with_mask(&self, mask: &Self) -> Result<Self, GraphComputingError> {
        // TODO: Size checking
        let mask_vector = mask.vertex_mask_ref();
        let mut resulting_vertex_selection = self.clone();
        GRAPHBLAS_VECTOR_LOGICAL_NEGATION_OPERATOR.apply_to_vector_with_mask(
            &self.to_full_vertex_mask_with_mask(mask_vector)?,
            resulting_vertex_selection.vertex_mask_mut_ref(),
            mask_vector,
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
    fn test_not_operator_for_vertex_selection() {
        let graph = standard_graph_for_testing();

        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();

        let no_integer_selection = integer_selection.not().unwrap();
        let mut no_integer = no_integer_selection.vertex_values_ref().unwrap();

        no_integer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        no_integer.dedup();

        println!("{:?}", no_integer);
        assert_eq!(no_integer.len(), 10);
        assert!(no_integer.contains(&&VertexValue::String(String::from("integer"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("natural_number"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("negative"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("not_a_number"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("real_number"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("string"))));
        assert!(no_integer.contains(&&VertexValue::String(String::from("positive"))));
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(-1.1)));
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(1.1)));
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(1.2)));
    }

    #[test]
    fn test_not_operator_for_vertex_selection_with_mask() {
        let graph = standard_graph_for_testing();

        let integer_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"integer")
            .unwrap();
        let number_selection = graph
            .select_vertices_connected_to_vertex_by_key(String::from("is_a"), &"real_number")
            .unwrap();

        let no_integer_selection = integer_selection.not_with_mask(&number_selection).unwrap();
        let mut no_integer = no_integer_selection.vertex_values_ref().unwrap();

        no_integer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        no_integer.dedup();

        println!("{:?}", no_integer);
        assert_eq!(no_integer.len(), 3);
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(-1.1)));
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(1.1)));
        assert!(no_integer.contains(&&VertexValue::FloatingPoint32Bit(1.2)));
    }
}
