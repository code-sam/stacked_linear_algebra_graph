use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
        binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
        options::OperatorOptions,
    },
};

use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::{
    edge::EdgeTypeKeyRef, edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix,
    value_type::SparseAdjacencyMatrixForValueType,
};
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{EdgeTypeIndex, Graph},
        value_type::ValueType,
        vertex::VertexTypeKeyRef,
    },
};
use graphblas_sparse_linear_algebra::operators::mask::MatrixMask;

pub trait ApplyScalarBinaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, EvaluationDomain>
where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    EvaluationDomain: ValueType,
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn with_index_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_right_argument(
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
        AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
        Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
        EvaluationDomain: ValueType,
    > ApplyScalarBinaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, EvaluationDomain>
    for Graph
where
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_index_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_index_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_unchecked_index_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_unchecked_index_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(right_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_key_defined_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn with_key_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }
}

pub trait ApplyScalarBinaryOperatorToMaskedAdjacencyMatrix<
    AdjacencyMatrix,
    Product,
    EvaluationDomain,
    Mask,
> where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    SparseMatrix<Product>: MatrixMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
    SparseMatrix<Mask>: MatrixMask,
{
    fn with_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_right_argument_and_mask(
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
        AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
        Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
        Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
        EvaluationDomain: ValueType,
    >
    ApplyScalarBinaryOperatorToMaskedAdjacencyMatrix<
        AdjacencyMatrix,
        Product,
        EvaluationDomain,
        Mask,
    > for Graph
where
    SparseMatrix<AdjacencyMatrix>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    SparseMatrix<Mask>: MatrixMask,
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn with_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn with_unchecked_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn with_unchecked_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(right_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn with_key_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        mask: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_left_argument(
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                operator,
                right_argument,
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn with_key_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        mask: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .binary_operator_applier()
            .apply_with_matrix_as_right_argument(
                left_argument,
                operator,
                AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadEdgeWeight;

    #[test]
    fn add_scalar_to_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_key = "vertex_type";
        let edge_type_1_key = "edge_type_1";
        let edge_type_2_key = "edge_type_2";
        let result_type_key = "result_type";

        let vertex_1 = VertexDefinedByKey::new(vertex_type_key, "vertex_1", &1u8);
        let vertex_2 = VertexDefinedByKey::new(vertex_type_key, "vertex_2", &2u8);

        let edge_vertex1_vertex2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1_key,
                vertex_2.key_ref(),
                vertex_1.key_ref(),
            ),
            25usize,
        );
        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_2_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            3u32,
        );

        let _vertex_type_1_index = graph.add_new_vertex_type(vertex_type_key).unwrap();
        let _vertex_1_index = graph.add_new_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index = graph.add_new_edge_type(edge_type_1_key).unwrap();
        let _edge_type_2_index = graph.add_new_edge_type(edge_type_2_key).unwrap();
        let _result_edge_type_index = graph.add_new_edge_type(result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8, u16, u8>::with_key_defined_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_1_key,
            &Plus::<u8>::new(),
            &1,
            &Assignment::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        ).unwrap();

        assert_eq!(
            ReadEdgeWeight::<u16>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_1.key_ref(),
                    vertex_2.key_ref(),
                ),
            )
            .unwrap(),
            Some(2)
        );

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u32, u16, u8>::with_key_defined_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_2_key,
            &Plus::<u8>::new(),
            &u8::MAX,
            &Assignment::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        ).unwrap();

        assert_eq!(
            ReadEdgeWeight::<u16>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_1.key_ref(),
                    vertex_2.key_ref(),
                ),
            )
            .unwrap(),
            Some(2)
        );

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<usize, usize, u8>::with_key_defined_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_1_key,
            &Plus::<u8>::new(),
            &1u8,
            &Assignment::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        ).unwrap();

        assert_eq!(
            ReadEdgeWeight::<usize>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_2.key_ref(),
                    vertex_1.key_ref(),
                ),
            )
            .unwrap(),
            Some(26)
        )
    }
}
