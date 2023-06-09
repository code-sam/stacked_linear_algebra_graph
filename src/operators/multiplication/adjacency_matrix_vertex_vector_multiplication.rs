use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationSemiring;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseVectorMultiplicationSemiringOperator;
use graphblas_sparse_linear_algebra::operators::mask::VectorMask;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrices;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrixByVector;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;

use crate::graph::graph::Graph;

use crate::graph::graph::VertexTypeIndex;
use crate::graph::value_type::SparseVertexVectorForValueType;
use crate::graph::vertex::VertexTypeKeyRef;
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::operators::graphblas_operator_applier::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeIndex,
        value_type::{SparseAdjacencyMatrixForValueType, ValueType},
    },
};

pub trait AdjacencyMatrixVertexVectorMultiplication<
    LeftArgument,
    RightArgument,
    Product,
    EvaluationDomain,
> where
    LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
    RightArgument: ValueType + SparseVertexVectorForValueType<RightArgument>,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    EvaluationDomain: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseVector<RightArgument>: VectorMask,
    SparseVector<Product>: VectorMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
        RightArgument: ValueType + SparseVertexVectorForValueType<RightArgument>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        EvaluationDomain: ValueType,
    >
    AdjacencyMatrixVertexVectorMultiplication<
        LeftArgument,
        RightArgument,
        Product,
        EvaluationDomain,
    > for Graph
where
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseVector<RightArgument>: VectorMask,
    SparseVector<Product>: VectorMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
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
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                options,
            )?)
    }
}

pub trait AdjacencyMatrixMultiplicationMasked<
    LeftArgument,
    RightArgument,
    Product,
    EvaluationDomain,
    Mask,
> where
    LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
    RightArgument: ValueType + SparseVertexVectorForValueType<RightArgument>,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseVector<RightArgument>: VectorMask,
    Product: ValueType + SparseVertexVectorForValueType<Product>,
    SparseVector<Product>: VectorMask,
    EvaluationDomain: ValueType,
    Mask: ValueType + SparseVertexVectorForValueType<Mask>,
    SparseVector<Mask>: VectorMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<
        LeftArgument: ValueType + SparseAdjacencyMatrixForValueType<LeftArgument>,
        RightArgument: ValueType + SparseVertexVectorForValueType<RightArgument>,
        Product: ValueType + SparseVertexVectorForValueType<Product>,
        Mask: ValueType + SparseVertexVectorForValueType<Mask>,
        EvaluationDomain: ValueType,
    >
    AdjacencyMatrixMultiplicationMasked<
        LeftArgument,
        RightArgument,
        Product,
        EvaluationDomain,
        Mask,
    > for Graph
where
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseVector<RightArgument>: VectorMask,
    SparseVector<Product>: VectorMask,
    SparseVector<Mask>: VectorMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
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
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &VertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeIndex,
        mask: &VertexTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(left_argument);

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_index_unchecked(right_argument);

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_index_unchecked(product);

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Semiring<EvaluationDomain>,
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
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let adjacency_matrix_left_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(left_argument)?;

        let vertex_vector_right_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_by_key(right_argument)?;

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_by_key(product)?;

        let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_by_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_vector_multiplication_operator()
            .apply_with_mask(
                LeftArgument::sparse_matrix_ref(adjacency_matrix_left_argument),
                operator,
                RightArgument::sparse_vector_ref(vertex_vector_right_argument),
                accumlator,
                Product::sparse_vector_mut_ref(vertex_vector_product),
                Mask::sparse_vector_ref(vertex_vector_mask),
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };
    use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait};
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadVertexValue;

    #[test]
    fn multiply_vertex_vector_with_adjacency_matrix() {
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

        AdjacencyMatrixVertexVectorMultiplication::<u8, u8, u16, u8>::by_key(
            &mut graph,
            &edge_type_1_key,
            &PlusTimes::<u8>::new(),
            vertex_type_key,
            &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
            vertex_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<u16>::with_key(&graph, vertex_type_key).unwrap()
        // );

        assert_eq!(
            ReadVertexValue::<u16>::vertex_value_by_key(
                &graph,
                vertex_type_key,
                vertex_1.key_ref()
            )
            .unwrap(),
            Some(2)
        );
    }
}
