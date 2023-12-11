use graphblas_sparse_linear_algebra::operators::select::MatrixSelector;
use graphblas_sparse_linear_algebra::operators::select::SelectFromMatrix;
use graphblas_sparse_linear_algebra::operators::transpose::TransposeMatrix;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{
        binary_operator::AccumulatorBinaryOperator, mask::MatrixMask, options::OperatorOptions,
    },
};

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
    fn apply(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn apply_unchecked(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
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
    fn apply(
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref(product)?;

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

    fn apply_unchecked(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product);

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
    fn apply_with_mask(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn apply_with_mask_unchecked(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
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
    fn apply_with_mask(
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
            unsafe { &*edge_store }.try_adjacency_matrix_ref(argument)?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref(product)?;

        let adjacency_matrix_mask = unsafe { &*edge_store }.try_adjacency_matrix_ref(mask)?;

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

    fn apply_with_mask_unchecked(
        &mut self,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(argument);

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product);

        let adjacency_matrix_mask = unsafe { &*edge_store }.try_adjacency_matrix_ref(mask)?;

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

    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn transpose_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_index = AddVertexType::<u8>::apply(&mut graph).unwrap();
        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let result_edge_type_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let vertex_index_1 = graph
            .add_vertex(&vertex_type_index, vertex_value_1)
            .unwrap();
        let vertex_index_2 = graph
            .add_vertex(&vertex_type_index, vertex_value_2)
            .unwrap();

        let edge_vertex1_vertex2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_1_index, vertex_index_1, vertex_index_2),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_1_index, vertex_index_2, vertex_index_1),
            2u8,
        );
        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(edge_type_2_index, vertex_index_1, vertex_index_2),
            1u8,
        );

        graph
            .add_edge_from_edge(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_edge_from_edge(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_edge_from_edge(edge_vertex1_vertex2_type_2.clone())
            .unwrap();

        TransposeAdjacencyMatrix::<u8, u16, u8>::apply(
            &mut graph,
            &edge_type_1_index,
            &Assignment::new(),
            &result_edge_type_index,
            &OperatorOptions::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_index_2,
                    vertex_index_1,
                ),
            )
            .unwrap(),
            Some(1)
        );

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_index_1,
                    vertex_index_2,
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
