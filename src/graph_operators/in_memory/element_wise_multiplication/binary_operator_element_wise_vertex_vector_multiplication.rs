use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseVectorMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::{Graph, GraphblasOperatorApplierCollection};
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_type::{CheckVertexTypeIndex, GetVertexVector};
use crate::graph_operators::operator_traits::element_wise_multiplication::{
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
        apply_binary_operator_element_wise_vertex_vector_multiplication::<EvaluationDomain>(
            &mut self.public_vertex_store,
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
        apply_binary_operator_element_wise_vertex_vector_multiplication_unchecked::<EvaluationDomain>(
            &mut self.public_vertex_store,
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

pub(crate) fn apply_binary_operator_element_wise_vertex_vector_multiplication<EvaluationDomain>(
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    vertex_store.try_vertex_type_index_validity(left_argument)?;
    vertex_store.try_vertex_type_index_validity(right_argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_binary_operator_element_wise_vertex_vector_multiplication_unchecked::<EvaluationDomain>(
        vertex_store,
        left_argument,
        operator,
        right_argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_binary_operator_element_wise_vertex_vector_multiplication_unchecked<
    EvaluationDomain,
>(
    vertex_store: *mut impl GetVertexVector,
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let vertex_vector_left_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

    let vertex_vector_right_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(right_argument);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
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
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
                .element_wise_vector_addition_binary_operator()
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

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph_operators::operator_traits::new::{NewVertex, NewVertexType};
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn binary_operator_element_wise_vertex_vector_multiplication() {
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
