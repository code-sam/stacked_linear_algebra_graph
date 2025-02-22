use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
    options::OperatorOptions,
};

use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::traits::vertex_type::GetVertexVector;
use crate::graph_operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToVertexVector;
use crate::graph_operators::operator_traits::apply_operator::ApplyScalarBinaryOperatorToVertexVectorUnchecked;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::GraphblasOperatorApplierCollection,
        vertex_store::traits::vertex_type::CheckVertexTypeIndex,
    },
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
        apply_scalar_binary_operator_with_vertex_vector_as_left_argument::<EvaluationDomain>(
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
        apply_scalar_binary_operator_with_vertex_vector_as_left_argument_and_by_unchecked_index::<
            EvaluationDomain,
        >(
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

pub(crate) fn apply_scalar_binary_operator_with_vertex_vector_as_left_argument<EvaluationDomain>(
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    vertex_store.try_vertex_type_index_validity(left_argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_scalar_binary_operator_with_vertex_vector_as_left_argument_and_by_unchecked_index::<
        EvaluationDomain,
    >(
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

pub(crate) fn apply_scalar_binary_operator_with_vertex_vector_as_right_argument<EvaluationDomain>(
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    left_argument: EvaluationDomain,
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
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    vertex_store.try_vertex_type_index_validity(right_argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_scalar_binary_operator_with_vertex_vector_as_right_argument_and_by_unchecked_index::<
        EvaluationDomain,
    >(
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

pub(crate) fn apply_scalar_binary_operator_with_vertex_vector_as_left_argument_and_by_unchecked_index<
    EvaluationDomain,
>(
    vertex_store: *mut impl GetVertexVector,
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl BinaryOperator<EvaluationDomain>,
    right_argument: EvaluationDomain,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OperatorOptions,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    let vertex_vector_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
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
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
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

pub(crate) fn apply_scalar_binary_operator_with_vertex_vector_as_right_argument_and_by_unchecked_index<
    EvaluationDomain,
>(
    vertex_store: *mut impl GetVertexVector,
    left_argument: EvaluationDomain,
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
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    let vertex_vector_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(right_argument);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
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
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
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
