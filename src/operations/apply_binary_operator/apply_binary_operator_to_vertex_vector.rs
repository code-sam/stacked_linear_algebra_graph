use graphblas_sparse_linear_algebra::{
    collections::{
        sparse_scalar::{SetScalarValue, SparseScalar},
        sparse_vector::SparseVector,
    },
    operators::{
        apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
        binary_operator::{Assignment, Plus},
        options::OperatorOptions,
    },
    value_type::AsBoolean,
};

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::graph::EdgeTypeIndex;
use crate::{
    error::{GraphComputingError, LogicError, LogicErrorType},
    graph::{
        graph::{Graph, GraphTrait, VertexTypeIndex},
        indexer::IndexerTrait,
        value_type::{
            implement_3_type_macro_for_all_native_value_types, implement_3_type_macro_stage_1,
            implement_3_type_macro_stage_2, implement_4_type_macro_for_all_native_value_types,
            implement_4_type_macro_stage_1, implement_4_type_macro_stage_2,
            implement_4_type_macro_stage_3, implement_macro_for_all_native_value_types,
            SparseVertexVectorForValueType, ValueType,
        },
        vertex::VertexTypeKeyRef,
        vertex_store::{
            // type_operations::get_vertex_vector_typed::GetVertexVectorTyped,
            type_operations::get_vertex_vector::GetVertexVector,
            SparseVertexVector,
            VertexStoreTrait,
            VertexVector,
        },
    },
};

pub trait ApplyBinaryOperatorToVertexVector<VertexVector, Product, EvaluationDomain>
where
    VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
{
    fn with_index_defined_vertex_vector_as_first_argument(
        &mut self,
        first_argument: &VertexTypeIndex,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_vertex_vector_as_second_argument(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeIndex,
        product: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_first_argument(
        &mut self,
        first_argument: &VertexTypeKeyRef,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_second_argument(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeKeyRef,
        product: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_apply_binary_operator_to_vertex_vector {
    ($evaluation_domain: ty) => {
        impl<
                AdjacencyMatrixOrVertexVector: ValueType + SparseVertexVectorForValueType<AdjacencyMatrixOrVertexVector>,
                Product: ValueType + SparseVertexVectorForValueType<Product>,
            > ApplyBinaryOperatorToVertexVector<AdjacencyMatrixOrVertexVector, Product, $evaluation_domain>
            for Graph
        {
            fn with_index_defined_vertex_vector_as_first_argument(
                &mut self,
                first_argument: &VertexTypeIndex,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

                Ok(operator.apply_with_vector_as_first_argument(
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                )?)
            }

            fn with_index_defined_vertex_vector_as_second_argument(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &VertexTypeIndex,
                product: &VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(second_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

                Ok(operator.apply_with_vector_as_second_argument(
                    &first_argument,
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                )?)
            }

            fn with_key_defined_vertex_vector_as_first_argument(
                &mut self,
                first_argument: &VertexTypeKeyRef,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &VertexTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

                Ok(operator.apply_with_vector_as_first_argument(
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                )?)
            }

            fn with_key_defined_vertex_vector_as_second_argument(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &VertexTypeKeyRef,
                product: &VertexTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(second_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

                Ok(operator.apply_with_vector_as_second_argument(
                    &first_argument,
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                )?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_apply_binary_operator_to_vertex_vector);

pub trait ApplyBinaryOperatorToVertexVectorWithMask<VertexVector, Product, EvaluationDomain, Mask>
where
    VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
{
    fn with_index_defined_vertex_vector_as_first_argument_and_mask(
        &mut self,
        first_argument: &VertexTypeIndex,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_vertex_vector_as_second_argument_and_mask(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeIndex,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_first_argument_and_mask(
        &mut self,
        first_argument: &VertexTypeKeyRef,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_second_argument_and_mask(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeKeyRef,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_apply_binary_operator_to_vertex_vector_with_mask {
    ($evaluation_domain: ty) => {
        impl<
                VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
                Product: ValueType + SparseVertexVectorForValueType<Product>,
                Mask: ValueType + SparseVertexVectorForValueType<Mask>,
            >
            ApplyBinaryOperatorToVertexVectorWithMask<
                VertexVector,
                Product,
                $evaluation_domain,
                Mask,
            > for Graph
        {
            fn with_index_defined_vertex_vector_as_first_argument_and_mask(
                &mut self,
                first_argument: &VertexTypeIndex,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

                Ok(operator.apply_with_vector_as_first_argument_and_mask(
                    VertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                    Mask::sparse_vector_ref(vertex_vector_mask),
                )?)
            }

            fn with_index_defined_vertex_vector_as_second_argument_and_mask(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &VertexTypeIndex,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(second_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

                Ok(operator.apply_with_vector_as_second_argument_and_mask(
                    &first_argument,
                    VertexVector::sparse_vector_ref(vertex_vector_argument),
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                    Mask::sparse_vector_ref(vertex_vector_mask),
                )?)
            }

            fn with_key_defined_vertex_vector_as_first_argument_and_mask(
                &mut self,
                first_argument: &VertexTypeKeyRef,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &VertexTypeKeyRef,
                mask: &VertexTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

                Ok(operator.apply_with_vector_as_first_argument_and_mask(
                    VertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                    Mask::sparse_vector_ref(vertex_vector_mask),
                )?)
            }

            fn with_key_defined_vertex_vector_as_second_argument_and_mask(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &VertexTypeKeyRef,
                product: &VertexTypeKeyRef,
                mask: &VertexTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let vertex_store = self.vertex_store_mut_ref_unsafe();

                let vertex_vector_argument =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(second_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

                Ok(operator.apply_with_vector_as_second_argument_and_mask(
                    &first_argument,
                    VertexVector::sparse_vector_ref(vertex_vector_argument),
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                    Mask::sparse_vector_ref(vertex_vector_mask),
                )?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(
    implement_apply_binary_operator_to_vertex_vector_with_mask
);
