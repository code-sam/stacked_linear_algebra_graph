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

use crate::graph::vertex_store::SparseVertexVectorDynamicDispatch;
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
            SparseVertexVectorForValueType, ValueType, ValueTypeIndex,
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

pub trait ApplyBinaryOperator<AdjacencyMatrixOrVertexVector, Product, EvaluationDomain>
where
    AdjacencyMatrixOrVertexVector:
        ValueType + SparseVertexVectorForValueType<AdjacencyMatrixOrVertexVector>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
{
    // fn apply_binary_operator_with_key_defined_vertex_vector_as_first_argument(
    //     &mut self,
    //     first_argument: &VertexTypeKeyRef,
    //     operator: &BinaryOperatorApplier<FirstArgument, SecondArgument, Product, EvaluationDomain>,
    //     second_argument: &SecondArgument,
    //     product: &VertexTypeKeyRef,
    // ) -> Result<(), GraphComputingError>;

    fn with_index_defined_vertex_vector_as_first_argument(
        &mut self,
        first_argument: &VertexTypeIndex,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    // fn apply_with_vector_as_second_argument(
    //     &self,
    //     first_argument: &EvaluationDomain,
    //     second_argument: &SparseVector<SecondArgument>,
    //     product: &mut SparseVector<Product>,
    // ) -> Result<(), SparseLinearAlgebraError>;

    // fn apply_with_matrix_as_first_argument(
    //     &self,
    //     first_argument: &SparseMatrix<FirstArgument>,
    //     second_argument: &EvaluationDomain,
    //     product: &mut SparseMatrix<Product>,
    // ) -> Result<(), SparseLinearAlgebraError>;

    // fn apply_with_matrix_as_second_argument(
    //     &self,
    //     first_argument: &EvaluationDomain,
    //     second_argument: &SparseMatrix<SecondArgument>,
    //     product: &mut SparseMatrix<Product>,
    // ) -> Result<(), SparseLinearAlgebraError>;
}

macro_rules! implement_apply_binary_operator {
    ($evaluation_domain: ty) => {
        impl<
                AdjacencyMatrixOrVertexVector: ValueType + SparseVertexVectorForValueType<AdjacencyMatrixOrVertexVector>,
                Product: ValueType + SparseVertexVectorForValueType<Product>,
            > ApplyBinaryOperator<AdjacencyMatrixOrVertexVector, Product, $evaluation_domain>
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
                    unsafe { &*vertex_store }.vertex_vector_by_index_ref(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_by_index_mut_ref(product)?;

                Ok(operator.apply_with_vector_as_first_argument(
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product), // SparseVertexVectorDynamicDispatch::<Product>::sparse_vector_mut_ref(vertex_vector_product),
                )?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_apply_binary_operator);

pub trait ApplyBinaryOperatorWithMask<
    AdjacencyMatrixOrVertexVector,
    Product,
    EvaluationDomain,
    Mask,
> where
    AdjacencyMatrixOrVertexVector: ValueType,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType + SparseVertexVectorForValueType<EvaluationDomain>,
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

    // fn apply_with_vector_as_second_argument_and_mask<MaskValueType: ValueType + AsBoolean>(
    //     &self,
    //     first_argument: &EvaluationDomain,
    //     second_argument: &SparseVector<SecondArgument>,
    //     product: &mut SparseVector<Product>,
    //     mask: &SparseVector<MaskValueType>,
    // ) -> Result<(), SparseLinearAlgebraError>;

    // fn apply_with_matrix_as_first_argument_and_mask<MaskValueType: ValueType + AsBoolean>(
    //     &self,
    //     first_argument: &SparseMatrix<FirstArgument>,
    //     second_argument: &EvaluationDomain,
    //     product: &mut SparseMatrix<Product>,
    //     mask: &SparseMatrix<MaskValueType>,
    // ) -> Result<(), SparseLinearAlgebraError>;

    // fn apply_with_matrix_as_second_argument_and_mask<MaskValueType: ValueType + AsBoolean>(
    //     &self,
    //     first_argument: &EvaluationDomain,
    //     second_argument: &SparseMatrix<SecondArgument>,
    //     product: &mut SparseMatrix<Product>,
    //     mask: &SparseMatrix<MaskValueType>,
    // ) -> Result<(), SparseLinearAlgebraError>;
}

macro_rules! implement_apply_binary_operator_with_mask {
    ($evaluation_domain: ty) => {
        impl<
        AdjacencyMatrixOrVertexVector: ValueType + SparseVertexVectorForValueType<AdjacencyMatrixOrVertexVector>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        Mask: ValueType + SparseVertexVectorForValueType<Mask>,
        > ApplyBinaryOperatorWithMask<AdjacencyMatrixOrVertexVector, Product, $evaluation_domain, Mask>
            for Graph
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
                    unsafe { &*vertex_store }.vertex_vector_by_index_ref(first_argument)?;

                let vertex_vector_product =
                    unsafe { &mut *vertex_store }.vertex_vector_by_index_mut_ref(product)?;

                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_by_index_ref(mask)?;

                Ok(operator.apply_with_vector_as_first_argument_and_mask(
                    AdjacencyMatrixOrVertexVector::sparse_vector_ref(vertex_vector_argument),
                    &second_argument,
                    Product::sparse_vector_mut_ref(vertex_vector_product),
                    Mask::sparse_vector_ref(vertex_vector_mask),
                )?)

            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_apply_binary_operator_with_mask);
