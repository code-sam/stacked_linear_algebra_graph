use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionMonoidOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::graph::{
    GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers, GraphTrait,
};
use crate::graph::graph::{Graph, VertexTypeIndex};
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait MonoidElementWiseVertexVectorAddition<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain> MonoidElementWiseVertexVectorAddition<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref(right_argument)?;

        let vertex_vector_product = unsafe { &mut *vertex_store }.vertex_vector_mut_ref(product)?;

        match mask {
            Some(mask) => {
                let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref(mask)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .element_wise_vector_addition_monoid_operator()
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
                    .element_wise_vector_addition_monoid_operator()
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

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
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
                    .element_wise_vector_addition_monoid_operator()
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
                    .element_wise_vector_addition_monoid_operator()
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
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;

    use super::*;

    use crate::operators::add::{AddVertex, AddVertexType};
    use crate::operators::read::GetVertexValue;

    #[test]
    fn monoid_element_wise_vertex_vector_addition() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let _edge_vertex1_vertex2_value = 1u8;
        let _edge_vertex2_vertex1_value = 2u8;
        let _edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let _vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        MonoidElementWiseVertexVectorAddition::<u8>::by_index(
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
    }
}
