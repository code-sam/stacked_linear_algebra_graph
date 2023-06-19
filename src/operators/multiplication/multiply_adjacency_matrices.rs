use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationSemiring;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseVectorMultiplicationSemiringOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrices;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
        mask::MatrixMask,
        options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::Graph;
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        value_type::{
            implement_macro_for_all_native_value_types, SparseAdjacencyMatrixForValueType,
            ValueType,
        },
    },
};

pub trait AdjacencyMatrixMultiplication<LeftArgument, RightArgument, Product, EvaluationDomain>
where
    LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
    RightArgument: ValueType + SparseAdjacencyMatrixForValueType<RightArgument>,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    EvaluationDomain: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
        RightArgument: ValueType + SparseAdjacencyMatrixForValueType<RightArgument>,
        Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
        EvaluationDomain: ValueType,
    > AdjacencyMatrixMultiplication<LeftArgument, RightArgument, Product, EvaluationDomain>
    for Graph
where
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
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

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(right_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
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

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                options,
            )?)
    }
}

pub trait MaskedAdjacencyMatrixMultiplication<
    LeftArgument,
    RightArgument,
    Product,
    EvaluationDomain,
    Mask,
> where
    LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
    RightArgument: ValueType + SparseAdjacencyMatrixForValueType<RightArgument>,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
    SparseMatrix<Product>: MatrixMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
    SparseMatrix<Mask>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        mask: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
        RightArgument: ValueType + SparseAdjacencyMatrixForValueType<RightArgument>,
        Product: ValueType + SparseAdjacencyMatrixForValueType<Product>,
        Mask: ValueType + SparseAdjacencyMatrixForValueType<Mask>,
        EvaluationDomain: ValueType,
    >
    MaskedAdjacencyMatrixMultiplication<
        LeftArgument,
        RightArgument,
        Product,
        EvaluationDomain,
        Mask,
    > for Graph
where
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    SparseMatrix<Mask>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
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

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(right_argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
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

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let adjacency_matrix_right_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(right_argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_matrix_ref(adjacency_matrix_right_argument),
                accumlator,
                Product::sparse_matrix_mut_ref(adjacency_matrix_product),
                Mask::sparse_matrix_ref(adjacency_matrix_mask),
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
        WeightedDirectedEdgeDefinedByKeysTrait,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operators::add_edge::AddEdge;
    use crate::operators::add_vertex::AddVertex;
    use crate::operators::{
        AddEdgeType, AddVertexType, ReadAdjacencyMatrixElementList, ReadEdgeWeight,
    };

    #[test]
    fn multiply_adjacency_matrices() {
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
            2u8,
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

        AdjacencyMatrixMultiplication::<u8, u8, u16, u8>::by_key(
            &mut graph,
            &edge_type_1_key,
            &PlusTimes::<u8>::new(),
            &edge_type_1_key,
            &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        // println!("{:?}", ReadAdjacencyMatrixElementList::<u16>::with_key(&graph, result_type_key).unwrap());

        assert_eq!(
            ReadEdgeWeight::<u16>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_1.key_ref(),
                    vertex_1.key_ref(),
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
