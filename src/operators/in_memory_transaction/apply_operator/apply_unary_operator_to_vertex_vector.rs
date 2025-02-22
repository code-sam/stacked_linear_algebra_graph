use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory::apply_operator::{
    apply_unary_operator_to_vertex_vector, apply_unary_operator_to_vertex_vector_unchecked,
};
use crate::operators::in_memory_transaction::transaction::InMemoryGraphTransaction;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToVertexVector;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToVertexVectorUnchecked;

impl<'g, EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVector<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_vertex_vector::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        apply_unary_operator_to_vertex_vector_unchecked::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            operator,
            argument,
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
    use graphblas_sparse_linear_algebra::operators::unary_operator::ColumnIndex;

    use super::*;

    use crate::graph::graph::Graph;
    use crate::graph::indexing::GetIndex;
    use crate::operators::operators::new::{NewVertex, NewVertexType};
    use crate::operators::operators::read::GetVertexValue;

    #[test]
    fn add_scalar_to_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();

        ApplyUnaryOperatorToVertexVector::<i32>::apply(
            &mut graph,
            &ColumnIndex::<i32>::new(),
            &vertex_type_1_index,
            &Assignment::new(),
            &vertex_type_1_index,
            None,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_1_index,)
                .unwrap(),
            Some(vertex_1_index.index() as u16)
        );
    }
}
