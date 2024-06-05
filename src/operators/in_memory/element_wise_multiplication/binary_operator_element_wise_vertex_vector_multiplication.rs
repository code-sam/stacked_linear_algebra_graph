use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseVectorMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::graph::Graph;
use crate::graph::graph::{
    GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers, GetVertexStore,
};
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::operators::operators::element_wise_multiplication::{
    BinaryOperatorElementWiseVertexVectorMultiplication,
    BinaryOperatorElementWiseVertexVectorMultiplicationUnchecked,
};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain> BinaryOperatorElementWiseVertexVectorMultiplication<EvaluationDomain>
    for Graph
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(left_argument)?;
        self.try_vertex_type_index_validity(right_argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        BinaryOperatorElementWiseVertexVectorMultiplicationUnchecked::apply(
            self,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
        )
    }
}

impl<EvaluationDomain>
    BinaryOperatorElementWiseVertexVectorMultiplicationUnchecked<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product);

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .element_wise_vector_multiplication_binary_operator()
                    .apply(
                        vertex_vector_left_argument,
                        operator,
                        vertex_vector_right_argument,
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
                    .element_wise_vector_multiplication_binary_operator()
                    .apply(
                        vertex_vector_left_argument,
                        operator,
                        vertex_vector_right_argument,
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
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::operators::operators::add::{AddVertex, AddVertexType};
    use crate::operators::operators::read::GetVertexValue;

    #[test]
    fn binary_operator_element_wise_vertex_vector_multiplication() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let _vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        BinaryOperatorElementWiseVertexVectorMultiplication::<u8>::apply(
            &mut graph,
            &vertex_type_1_index,
            &Plus::<u8>::new(),
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
    }
}
