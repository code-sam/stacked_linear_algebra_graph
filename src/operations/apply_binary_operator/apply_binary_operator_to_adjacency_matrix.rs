use graphblas_sparse_linear_algebra::{
    collections::{
        sparse_scalar::{SetScalarValue, SparseScalar},
        sparse_vector::SparseVector,
    },
    operators::{
        apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
        binary_operator::{AccumulatorBinaryOperator, Assignment, BinaryOperator, Plus},
        options::OperatorOptions,
    },
    value_type::AsBoolean,
};

use crate::graph::graph::EdgeTypeIndex;
use crate::graph::{
    edge::EdgeTypeKeyRef, edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix,
    value_type::SparseAdjacencyMatrixForValueType,
};
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

pub trait ApplyBinaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, EvaluationDomain>
where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    EvaluationDomain: ValueType,
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
        right_argument: &EdgeTypeKeyRef,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_apply_binary_operator_to_adjacency_matrix {
    ($evaluation_domain: ty) => {
        impl<
                AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
                Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
            > ApplyBinaryOperatorToAdjacencyMatrix<AdjacencyMatrix, Product, $evaluation_domain>
            for Graph
        {
            fn with_index_defined_adjacency_matrix_as_left_argument(
                &mut self,
                left_argument: &EdgeTypeIndex,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &EdgeTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        operator,
                        &right_argument,
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }

            fn with_index_defined_adjacency_matrix_as_right_argument(
                &mut self,
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &EdgeTypeIndex,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &EdgeTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(right_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }

            fn with_unchecked_index_defined_adjacency_matrix_as_left_argument(
                &mut self,
                left_argument: &EdgeTypeIndex,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &EdgeTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

                let adjacency_matrix_product = unsafe { &mut *edge_store }
                    .adjacency_matrix_mut_ref_for_index_unchecked(product);

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        operator,
                        &right_argument,
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }

            fn with_unchecked_index_defined_adjacency_matrix_as_right_argument(
                &mut self,
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &EdgeTypeIndex,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &EdgeTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument = unsafe { &*edge_store }
                    .adjacency_matrix_ref_for_index_unchecked(right_argument);

                let adjacency_matrix_product = unsafe { &mut *edge_store }
                    .adjacency_matrix_mut_ref_for_index_unchecked(product);

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }

            fn with_key_defined_adjacency_matrix_as_left_argument(
                &mut self,
                left_argument: &VertexTypeKeyRef,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeKeyRef,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        operator,
                        right_argument,
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }

            fn with_key_defined_adjacency_matrix_as_right_argument(
                &mut self,
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &VertexTypeKeyRef,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeKeyRef,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(right_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                        accumlator,
                        Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                        options,
                    )?)
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_apply_binary_operator_to_adjacency_matrix);

pub trait ApplyBinaryOperatorToAdjacencyMatrixWithMask<
    AdjacencyMatrix,
    Product,
    EvaluationDomain,
    Mask,
> where
    AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
{
    fn with_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_left_argument_and_mask(
        &mut self,
        left_argument: &VertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn with_unchecked_index_defined_adjacency_matrix_as_right_argument_and_mask(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
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

macro_rules! implement_apply_binary_operator_to_adjacency_matrix_with_mask {
    ($evaluation_domain: ty) => {
        impl<
                AdjacencyMatrix: ValueType + SparseAdjacencyMatrixForValueType<AdjacencyMatrix>,
                Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
                Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
            >
            ApplyBinaryOperatorToAdjacencyMatrixWithMask<
                AdjacencyMatrix,
                Product,
                $evaluation_domain,
                Mask,
            > for Graph
        {
            fn with_index_defined_adjacency_matrix_as_left_argument_and_mask(
                &mut self,
                left_argument: &VertexTypeIndex,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument_and_mask(
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
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &VertexTypeIndex,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument_and_mask(
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
                left_argument: &VertexTypeIndex,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

                let adjacency_matrix_product = unsafe { &mut *edge_store }
                    .adjacency_matrix_mut_ref_for_index_unchecked(product);

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(mask);

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument_and_mask(
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
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &VertexTypeIndex,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
                product: &VertexTypeIndex,
                mask: &VertexTypeIndex,
                options: &OperatorOptions,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument = unsafe { &*edge_store }
                    .adjacency_matrix_ref_for_index_unchecked(right_argument);

                let adjacency_matrix_product = unsafe { &mut *edge_store }
                    .adjacency_matrix_mut_ref_for_index_unchecked(product);

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(mask);

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument_and_mask(
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
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &$evaluation_domain,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
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

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument_and_mask(
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
                left_argument: &$evaluation_domain,
                operator: &impl BinaryOperator<$evaluation_domain>,
                right_argument: &EdgeTypeKeyRef,
                accumlator: &impl AccumulatorBinaryOperator<$evaluation_domain>,
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

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

                Ok(self
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument_and_mask(
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
    };
}
implement_macro_for_all_native_value_types!(
    implement_apply_binary_operator_to_adjacency_matrix_with_mask
);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
        WeightedDirectedEdgeDefinedByKeysTrait,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::{AddEdgeType, AddVertexType, ReadEdgeWeight};

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

        let vertex_type_1_index = graph.add_new_vertex_type(vertex_type_key).unwrap();
        let vertex_1_index = graph.add_new_vertex(vertex_1.clone()).unwrap();
        let vertex_2_index = graph.add_new_vertex(vertex_2.clone()).unwrap();

        let edge_type_1_index = graph.add_new_edge_type(edge_type_1_key).unwrap();
        let edge_type_2_index = graph.add_new_edge_type(edge_type_2_key).unwrap();
        let result_edge_type_index = graph.add_new_edge_type(result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        ApplyBinaryOperatorToAdjacencyMatrix::<u8, u16, u8>::with_key_defined_adjacency_matrix_as_left_argument(
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

        ApplyBinaryOperatorToAdjacencyMatrix::<u32, u16, u8>::with_key_defined_adjacency_matrix_as_left_argument(
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

        ApplyBinaryOperatorToAdjacencyMatrix::<usize, usize, u8>::with_key_defined_adjacency_matrix_as_left_argument(
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
