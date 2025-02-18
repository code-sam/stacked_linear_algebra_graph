use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::select::{SelectFromVector, VectorSelector};

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::operators::in_memory::select::{
    select_from_vertex_vector, select_from_vertex_vector_unchecked,
};
use crate::operators::operator_traits::select::{
    SelectFromVertexVector, SelectFromVertexVectorUnchecked,
};
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain> SelectFromVertexVector<EvaluationDomain> for InMemoryGraphTransaction<'g>
where
    VectorSelector: SelectFromVector<EvaluationDomain>,
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        select_from_vertex_vector::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            selector,
            selector_argument,
            argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain> SelectFromVertexVectorUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
where
    VectorSelector: SelectFromVector<EvaluationDomain>,
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: EvaluationDomain,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        select_from_vertex_vector_unchecked::<EvaluationDomain>(
            &mut self.vertex_store_transaction,
            selector,
            selector_argument,
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
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::graph::Graph;
    use crate::operators::operator_traits::new::{NewVertex, NewVertexType};
    use crate::operators::operator_traits::read::GetVertexValue;

    #[test]
    fn select_from_vertex_vector() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();
        let _vertex_result_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        SelectFromVertexVector::<u8>::by_index(
            &mut graph,
            &IsValueGreaterThan::<u8>::new(),
            1,
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
            None
        );

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_2_index,)
                .unwrap(),
            Some(2)
        );
    }
}
