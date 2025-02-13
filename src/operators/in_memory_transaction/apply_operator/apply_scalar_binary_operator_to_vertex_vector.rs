use graphblas_sparse_linear_algebra::operators::apply::{
    ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier,
};
use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory::apply_operator::{
    apply_scalar_binary_operator_with_vertex_vector_as_left_argument,
    apply_scalar_binary_operator_with_vertex_vector_as_left_argument_and_by_unchecked_index,
    apply_scalar_binary_operator_with_vertex_vector_as_right_argument,
    apply_scalar_binary_operator_with_vertex_vector_as_right_argument_and_by_unchecked_index,
};
use crate::operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToVertexVector;
use crate::operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToVertexVectorUnchecked;
use crate::operators::transaction::in_memory::InMemoryGraphTransaction;

impl<'g, EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToVertexVector<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
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
        apply_scalar_binary_operator_with_vertex_vector_as_left_argument::<EvaluationDomain>(
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
        apply_scalar_binary_operator_with_vertex_vector_as_right_argument::<EvaluationDomain>(
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

impl<'g, EvaluationDomain: ValueType>
    ApplyScalarBinaryOperatorToVertexVectorUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
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
        apply_scalar_binary_operator_with_vertex_vector_as_left_argument_and_by_unchecked_index::<
            EvaluationDomain,
        >(
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
        apply_scalar_binary_operator_with_vertex_vector_as_right_argument_and_by_unchecked_index::<
            EvaluationDomain,
        >(
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
