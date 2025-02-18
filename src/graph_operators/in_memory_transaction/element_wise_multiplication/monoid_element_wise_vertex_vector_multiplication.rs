use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph_operators::in_memory::element_wise_multiplication::{
    apply_monoid_element_wise_vertex_vector_multiplication,
    apply_monoid_element_wise_vertex_vector_multiplication_unchecked,
};
use crate::graph_operators::operator_traits::element_wise_multiplication::{
    MonoidElementWiseVertexVectorMultiplication,
    MonoidElementWiseVertexVectorMultiplicationUnchecked,
};
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain> MonoidElementWiseVertexVectorMultiplication<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_monoid_element_wise_vertex_vector_multiplication::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain> MonoidElementWiseVertexVectorMultiplicationUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_monoid_element_wise_vertex_vector_multiplication_unchecked::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;

    use super::*;

    use crate::graph::graph::Graph;
    use crate::graph_operators::operator_traits::new::{NewVertex, NewVertexType};
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn monoid_element_wise_vertex_vector_multiplication() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let _vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        MonoidElementWiseVertexVectorMultiplication::<u8>::apply(
            &mut graph,
            &vertex_type_1_index,
            &graphblas_sparse_linear_algebra::operators::monoid::Plus::<u8>::new(),
            &vertex_type_1_index,
            &Assignment::new(),
            &vertex_type_1_index,
            Some(&vertex_type_1_index),
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_2_index)
                .unwrap(),
            Some(4)
        );

        MonoidElementWiseVertexVectorMultiplication::<u8>::apply(
            &mut graph,
            &vertex_type_1_index,
            &graphblas_sparse_linear_algebra::operators::monoid::Plus::<u8>::new(),
            &vertex_type_1_index,
            &Assignment::new(),
            &vertex_type_1_index,
            Some(&vertex_type_1_index),
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<u32>::vertex_value(&graph, &vertex_type_1_index, &vertex_2_index)
                .unwrap(),
            Some(8)
        );
    }
}
