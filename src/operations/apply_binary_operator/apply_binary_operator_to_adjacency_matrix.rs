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
    fn with_index_defined_adjacency_matrix_as_first_argument(
        &mut self,
        first_argument: &EdgeTypeIndex,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_adjacency_matrix_as_second_argument(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EdgeTypeIndex,
        product: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_first_argument(
        &mut self,
        first_argument: &EdgeTypeKeyRef,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_second_argument(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EdgeTypeKeyRef,
        product: &EdgeTypeKeyRef,
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
            fn with_index_defined_adjacency_matrix_as_first_argument(
                &mut self,
                first_argument: &EdgeTypeIndex,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &EdgeTypeIndex,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(first_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                Ok(operator.apply_with_matrix_as_first_argument(
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    &second_argument,
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                )?)
            }

            fn with_index_defined_adjacency_matrix_as_second_argument(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &EdgeTypeIndex,
                product: &EdgeTypeIndex,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(second_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                Ok(operator.apply_with_matrix_as_second_argument(
                    &first_argument,
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                )?)
            }

            fn with_key_defined_adjacency_matrix_as_first_argument(
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
                let edge_store = self.edge_store_mut_ref_unsafe();

                // TODO:: as an alternative to unsafe{}, cloning will work. But this is expensive.
                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(first_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                Ok(operator.apply_with_matrix_as_first_argument(
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    &second_argument,
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                )?)
            }

            fn with_key_defined_adjacency_matrix_as_second_argument(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &VertexTypeKeyRef,
                product: &VertexTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(second_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                Ok(operator.apply_with_matrix_as_second_argument(
                    &first_argument,
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
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
    fn with_index_defined_adjacency_matrix_as_first_argument_and_mask(
        &mut self,
        first_argument: &VertexTypeIndex,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_index_defined_adjacency_matrix_as_second_argument_and_mask(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeIndex,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_first_argument_and_mask(
        &mut self,
        first_argument: &VertexTypeKeyRef,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &EvaluationDomain,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn with_key_defined_adjacency_matrix_as_second_argument_and_mask(
        &mut self,
        first_argument: &EvaluationDomain,
        operator: &BinaryOperatorApplier<EvaluationDomain>,
        second_argument: &VertexTypeKeyRef,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
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
            fn with_index_defined_adjacency_matrix_as_first_argument_and_mask(
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
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(first_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

                Ok(operator.apply_with_matrix_as_first_argument_and_mask(
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    &second_argument,
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                    Mask::sparse_matrix_ref(adjacency_matrix_mask),
                )?)
            }

            fn with_index_defined_adjacency_matrix_as_second_argument_and_mask(
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
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(second_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

                Ok(operator.apply_with_matrix_as_second_argument_and_mask(
                    &first_argument,
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                    Mask::sparse_matrix_ref(adjacency_matrix_mask),
                )?)
            }

            fn with_key_defined_adjacency_matrix_as_first_argument_and_mask(
                &mut self,
                first_argument: &EdgeTypeKeyRef,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &$evaluation_domain,
                product: &EdgeTypeKeyRef,
                mask: &EdgeTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(first_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

                Ok(operator.apply_with_matrix_as_first_argument_and_mask(
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    &second_argument,
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                    Mask::sparse_matrix_ref(adjacency_matrix_mask),
                )?)
            }

            fn with_key_defined_adjacency_matrix_as_second_argument_and_mask(
                &mut self,
                first_argument: &$evaluation_domain,
                operator: &BinaryOperatorApplier<$evaluation_domain>,
                second_argument: &EdgeTypeKeyRef,
                product: &EdgeTypeKeyRef,
                mask: &EdgeTypeKeyRef,
            ) -> Result<(), GraphComputingError> {
                // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
                // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
                // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
                // For example, an alternative to unsafe access would be to clone the operands.
                let edge_store = self.edge_store_mut_ref_unsafe();

                let adjacency_matrix_argument =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(second_argument)?;

                let adjacency_matrix_product =
                    unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

                Ok(operator.apply_with_matrix_as_second_argument_and_mask(
                    &first_argument,
                    AdjacencyMatrix::sparse_matrix_ref(adjacency_matrix_argument),
                    Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                    Mask::sparse_matrix_ref(adjacency_matrix_mask),
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
            2u16,
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

        let plus_binaray_operator = BinaryOperatorApplier::new(
            &Plus::<u8>::new(),
            &OperatorOptions::new_default(),
            &Assignment::new(),
        );

        ApplyBinaryOperatorToAdjacencyMatrix::<u8, u16, u8>::with_key_defined_adjacency_matrix_as_first_argument(
            &mut graph,
            &edge_type_1_key,
            &plus_binaray_operator,
            &1,
            result_type_key,
        ).unwrap();

        assert_eq!(ReadEdgeWeight::<u16>::key_defined_edge_weight(
            &graph,
            &DirectedEdgeCoordinateDefinedByKeys::new(
                result_type_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
        ).unwrap(), Some(2));

        ApplyBinaryOperatorToAdjacencyMatrix::<u32, u16, u8>::with_key_defined_adjacency_matrix_as_first_argument(
            &mut graph,
            &edge_type_2_key,
            &plus_binaray_operator,
            &u8::MAX,
            result_type_key,
        ).unwrap();

        assert_eq!(ReadEdgeWeight::<u16>::key_defined_edge_weight(
            &graph,
            &DirectedEdgeCoordinateDefinedByKeys::new(
                result_type_key,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
        ).unwrap(), Some(2))
    }
}
