use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationMonoidOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType, vertex::vertex::VertexTypeKeyRef},
};

pub trait MonoidElementWiseAdjacencyMatrixMultiplication<
    LeftArgument,
    RightArgument,
    Product,
    EvaluationDomain,
> where
    LeftArgument: ValueType,
    RightArgument: ValueType,
    Product: ValueType,
    EvaluationDomain: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &VertexTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<LeftArgument, RightArgument, Product, EvaluationDomain: ValueType>
    MonoidElementWiseAdjacencyMatrixMultiplication<
        LeftArgument,
        RightArgument,
        Product,
        EvaluationDomain,
    > for Graph
where
    LeftArgument: ValueType,
    RightArgument: ValueType,
    Product: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }
}

pub trait MonoidElementWiseMaskedAdjacencyMatrixMultiplication<
    LeftArgument,
    RightArgument,
    Product,
    EvaluationDomain,
    Mask,
> where
    LeftArgument: ValueType,
    RightArgument: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    Product: ValueType,
    SparseMatrix<Product>: MatrixMask,
    EvaluationDomain: ValueType,
    Mask: ValueType,
    SparseMatrix<Mask>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        left_argument: &VertexTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &VertexTypeKeyRef,
        mask: &VertexTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<LeftArgument, RightArgument, Product, Mask, EvaluationDomain: ValueType>
    MonoidElementWiseMaskedAdjacencyMatrixMultiplication<
        LeftArgument,
        RightArgument,
        Product,
        EvaluationDomain,
        Mask,
    > for Graph
where
    LeftArgument: ValueType,
    RightArgument: ValueType,
    Product: ValueType,
    Mask: ValueType,
    SparseMatrix<LeftArgument>: MatrixMask,
    SparseMatrix<RightArgument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    SparseMatrix<Mask>: MatrixMask,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn by_key(
        &mut self,
        left_argument: &EdgeTypeKeyRef,
        operator: &impl Monoid<EvaluationDomain>,
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
            .element_wise_matrix_multiplication_monoid_operator()
            .apply(
                adjacency_matrix_left_argument,
                operator,
                adjacency_matrix_right_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::monoid::Plus;

    use super::*;

    use crate::graph::edge::{
        DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys,
    };
    use crate::graph::vertex::vertex_defined_by_key::{
        VertexDefinedByKey, VertexDefinedByKeyTrait,
    };
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadEdgeWeight;

    #[test]
    fn monoid_element_wise_adjacency_matrix_multiplication() {
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
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_1_key).unwrap();
        let _edge_type_2_index =
            AddEdgeType::<u16>::add_new_edge_type(&mut graph, edge_type_2_key).unwrap();
        let _result_edge_type_index =
            AddEdgeType::<u16>::add_new_edge_type(&mut graph, result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        for _i in 0..2 {
            MonoidElementWiseAdjacencyMatrixMultiplication::<u8, u8, u16, u8>::by_key(
                &mut graph,
                &edge_type_1_key,
                &Plus::<u8>::new(),
                &edge_type_1_key,
                &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
                result_type_key,
                &OperatorOptions::new_default(),
            )
            .unwrap();
        }

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
            Some(4)
        );

        MonoidElementWiseAdjacencyMatrixMultiplication::<u8, usize, u16, u8>::by_key(
            &mut graph,
            &edge_type_1_key,
            &Plus::<u8>::new(),
            &edge_type_2_key,
            &Assignment::new(),
            result_type_key,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            ReadEdgeWeight::<u16>::key_defined_edge_weight(
                &graph,
                &DirectedEdgeCoordinateDefinedByKeys::new(
                    result_type_key,
                    vertex_2.key_ref(),
                    vertex_1.key_ref(),
                ),
            )
            .unwrap(),
            None
        );
    }
}
