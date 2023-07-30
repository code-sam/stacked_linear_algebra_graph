use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    operators::{
        apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
        binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
        mask::VectorMask,
        options::OperatorOptions,
    },
};

use crate::graph::graph::VertexTypeIndex;
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::Graph,
        value_type::{SparseVertexVectorForValueType, ValueType},
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::VertexStoreTrait,
    },
};

pub trait ApplyScalarBinaryOperatorToVertexVector<VertexVector, Product, EvaluationDomain>
where
    VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
    SparseVector<VertexVector>: VectorMask,
    SparseVector<Product>: VectorMask,
{
    fn with_index_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        EvaluationDomain: ValueType,
    > ApplyScalarBinaryOperatorToVertexVector<VertexVector, Product, EvaluationDomain> for Graph
where
    SparseVector<VertexVector>: VectorMask,
    SparseVector<Product>: VectorMask,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_index_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(left_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_index_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_unchecked_index_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(left_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_unchecked_index_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_key_defined_vertex_vector_as_left_argument(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(left_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }

    fn with_key_defined_vertex_vector_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                unsafe { &*vertex_store }.mask_to_select_entire_vertex_vector_ref(),
                options,
            )?)
    }
}

pub trait ApplyScalarBinaryOperatorToMaskedVertexVector<
    VertexVector,
    Product,
    EvaluationDomain,
    Mask,
> where
    VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
    SparseVector<VertexVector>: VectorMask,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    SparseVector<Product>: VectorMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    SparseVector<Mask>: VectorMask,
{
    fn with_index_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        VertexVector: ValueType + SparseVertexVectorForValueType<VertexVector>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        Mask: ValueType + SparseVertexVectorForValueType<Mask>,
        EvaluationDomain: ValueType,
    > ApplyScalarBinaryOperatorToMaskedVertexVector<VertexVector, Product, EvaluationDomain, Mask>
    for Graph
where
    SparseVector<VertexVector>: VectorMask,
    SparseVector<Product>: VectorMask,
    SparseVector<Mask>: VectorMask,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_index_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(left_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_index_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_unchecked_index_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(left_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_unchecked_index_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_key_defined_vertex_vector_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(left_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_left_argument(
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn with_key_defined_vertex_vector_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
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

        let vertex_vector_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_vector_as_right_argument(
                left_argument,
                operator,
                VertexVector::sparse_vector_ref(vertex_vector_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }
}
