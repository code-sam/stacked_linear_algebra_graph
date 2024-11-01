use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
    options::OperatorOptions,
};

use crate::operators::operators::apply_operator::ApplyScalarBinaryOperatorToVertexVector;
use crate::operators::operators::apply_operator::ApplyScalarBinaryOperatorToVertexVectorUnchecked;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::Graph,
        indexing::{GetVertexTypeIndex, VertexTypeIndex},
        value_type::ValueType,
    },
};
use crate::{
    graph::{
        graph::{
            GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers, GetVertexStore,
        },
        vertex_store::operations::GetVertexVector,
    },
    operators::indexing::CheckIndex,
};

impl<EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToVertexVector<EvaluationDomain>
    for Graph
where
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(left_argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        ApplyScalarBinaryOperatorToVertexVectorUnchecked::with_vertex_vector_as_left_argument_and_by_unchecked_index(
            self,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options)
    }

    fn with_vertex_vector_as_right_argument(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        self.try_vertex_type_index_validity(right_argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        ApplyScalarBinaryOperatorToVertexVectorUnchecked::with_vertex_vector_as_right_argument_and_by_unchecked_index(
            self,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options)
    }
}

impl<EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for Graph
where
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_vertex_vector_as_left_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_vector_as_left_argument(
                        vertex_vector_argument,
                        operator,
                        right_argument,
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
                    .binary_operator_applier()
                    .apply_with_vector_as_left_argument(
                        vertex_vector_argument,
                        operator,
                        right_argument,
                        accumlator,
                        vertex_vector_product,
                        vertex_vector_mask,
                        options,
                    )?)
            }
        }
    }

    fn with_vertex_vector_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_vector_as_right_argument(
                        left_argument,
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
                    .binary_operator_applier()
                    .apply_with_vector_as_right_argument(
                        left_argument,
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
