use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::{
    apply::ApplyUnaryOperator as ApplyGraphBlasUnaryOperator,
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::graph::{GetGraphblasOperatorAppliers, GetVertexStore};
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToVertexVector;
use crate::operators::operators::apply_operator::ApplyUnaryOperatorToVertexVectorUnchecked;
use crate::operators::operators::indexing::CheckIndex;
use crate::{
    error::GraphComputingError,
    graph::{graph::Graph, value_type::ValueType},
};

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVector<EvaluationDomain> for Graph {
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        ApplyUnaryOperatorToVertexVectorUnchecked::apply(
            self, operator, argument, accumlator, product, mask, options,
        )
    }
}

impl<EvaluationDomain: ValueType> ApplyUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for Graph
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
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .unary_operator_applier()
                    .apply_to_vector(
                        operator,
                        vertex_vector_argument,
                        accumlator,
                        vertex_vector_product,
                        vertex_vector_mask,
                        options,
                    )?)
            }
            None => {
                let vertex_vector_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_vector_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .unary_operator_applier()
                    .apply_to_vector(
                        operator,
                        vertex_vector_argument,
                        accumlator,
                        vertex_vector_product,
                        vertex_vector_mask,
                        options,
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::unary_operator::ColumnIndex;

    use super::*;

    use crate::graph::indexing::{GetIndex, GetVertexIndexIndex};
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
