use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseVectorAdditionMonoidOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;
use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::VectorMask, options::OperatorOptions,
    },
};

use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;
use crate::graph::graph::{Graph, VertexTypeIndex};
use crate::graph::vertex::vertex::VertexTypeKeyRef;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait MonoidElementWiseVertexVectorAddition<
    EvaluationDomain,
> where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain>
    MonoidElementWiseVertexVectorAddition<EvaluationDomain>
    for Graph
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
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .element_wise_vector_addition_monoid_operator()
            .apply(
                vertex_vector_left_argument,
                operator,
                vertex_vector_right_argument,
                accumlator,
                vertex_vector_product,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(left_argument);

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .element_wise_vector_addition_monoid_operator()
            .apply(
                vertex_vector_left_argument,
                operator,
                vertex_vector_right_argument,
                accumlator,
                vertex_vector_product,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .element_wise_vector_addition_monoid_operator()
            .apply(
                vertex_vector_left_argument,
                operator,
                vertex_vector_right_argument,
                accumlator,
                vertex_vector_product,
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait MonoidElementWiseMaskedVertexVectorAddition<
    EvaluationDomain,
> where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain>
    MonoidElementWiseMaskedVertexVectorAddition<
        EvaluationDomain
    > for Graph
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
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

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

    fn by_unchecked_index(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(left_argument);

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

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

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

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

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };
    use crate::graph::vertex::vertex_defined_by_key::{
        VertexDefinedByKey, VertexDefinedByKeyTrait,
    };
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadVertexValue;

    #[test]
    fn monoid_element_wise_vertex_vector_addition() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_key = "vertex_type";

        let vertex_1 = VertexDefinedByKey::new(vertex_type_key, "vertex_1", &1u8);
        let vertex_2 = VertexDefinedByKey::new(vertex_type_key, "vertex_2", &2u8);

        let _vertex_type_1_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_type_key).unwrap();
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        MonoidElementWiseMaskedVertexVectorAddition::<u8>::by_key(
            &mut graph,
            vertex_type_key,
            &graphblas_sparse_linear_algebra::operators::monoid::Plus::<u8>::new(),
            vertex_type_key,
            &Assignment::new(),
            vertex_type_key,
            vertex_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            ReadVertexValue::<u16>::vertex_value_by_key(
                &graph,
                vertex_type_key,
                vertex_2.key_ref()
            )
            .unwrap(),
            Some(4)
        );

    }
}
