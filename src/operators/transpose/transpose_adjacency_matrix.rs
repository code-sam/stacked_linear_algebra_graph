use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionBinaryOperator;

use graphblas_sparse_linear_algebra::operators::select::MatrixSelector;
use graphblas_sparse_linear_algebra::operators::select::SelectFromMatrix;
use graphblas_sparse_linear_algebra::operators::transpose::TransposeMatrix;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType},
};

pub trait TransposeAdjacencyMatrix<Argument, Product, EvaluationDomain>
where
    Argument: ValueType,
    Product: ValueType,
    EvaluationDomain: ValueType,
    SparseMatrix<Argument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
{
    fn by_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<Argument: ValueType, Product: ValueType, EvaluationDomain: ValueType>
    TransposeAdjacencyMatrix<Argument, Product, EvaluationDomain> for Graph
where
    Argument: ValueType,
    Product: ValueType,
    SparseMatrix<Argument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        argument: &EdgeTypeIndex,
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply(
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply(
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                options,
            )?)
    }

    fn by_key(
        &mut self,
        argument: &EdgeTypeKeyRef,
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
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply(
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                options,
            )?)
    }
}

pub trait TransposeAdjacencyMatrixMasked<Argument, Product, EvaluationDomain, Mask>
where
    Argument: ValueType,
    SparseMatrix<Argument>: MatrixMask,
    Product: ValueType,
    SparseMatrix<Product>: MatrixMask,
    EvaluationDomain: ValueType,
    Mask: ValueType,
    SparseMatrix<Mask>: MatrixMask,
{
    fn by_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_key(
        &mut self,
        argument: &EdgeTypeKeyRef,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeKeyRef,
        mask: &EdgeTypeKeyRef,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<Argument, Product, EvaluationDomain: ValueType, Mask>
    TransposeAdjacencyMatrixMasked<Argument, Product, EvaluationDomain, Mask> for Graph
where
    Argument: ValueType,
    Product: ValueType,
    Mask: ValueType,
    SparseMatrix<Argument>: MatrixMask,
    SparseMatrix<Product>: MatrixMask,
    SparseMatrix<Mask>: MatrixMask,
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        argument: &EdgeTypeIndex,
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref_for_index(product)?;

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply_with_mask(
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_for_index_unchecked(argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_index_unchecked(product);

        let adjacency_matrix_mask =
            unsafe { &*edge_store }.try_adjacency_matrix_ref_for_index(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply_with_mask(
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn by_key(
        &mut self,
        argument: &EdgeTypeKeyRef,
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
            unsafe { &*edge_store }.adjacency_matrix_ref_for_key(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_for_key(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.adjacency_matrix_ref_for_key(mask)?;

        Ok(self
            .graphblas_operator_applier_collection_ref()
            .matrix_transposer()
            .apply_with_mask(
                adjacency_matrix_argument,
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
    fn transpose_adjacency_matrix() {
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

        let _vertex_type_1_index =
            AddVertexType::<u8>::add_new_vertex_type(&mut graph, vertex_type_key).unwrap();
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_1_key).unwrap();
        let _edge_type_2_index =
            AddEdgeType::<u16>::add_new_edge_type(&mut graph, edge_type_2_key).unwrap();
        let _result_edge_type_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, result_type_key).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        TransposeAdjacencyMatrix::<u8, u16, u8>::by_key(
            &mut graph,
            &edge_type_1_key,
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
            Some(1)
        );

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
    }
}
