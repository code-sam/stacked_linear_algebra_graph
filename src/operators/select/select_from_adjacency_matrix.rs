use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::select::MatrixSelector;
use graphblas_sparse_linear_algebra::operators::select::SelectFromMatrix;
use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, options::OperatorOptions,
};

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollectionTrait;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType},
};

pub trait SelectFromAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain: ValueType> SelectFromAdjacencyMatrix<EvaluationDomain> for Graph
where
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
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
            .matrix_selector()
            .apply(
                selector,
                selector_argument,
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
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
            .matrix_selector()
            .apply(
                selector,
                selector_argument,
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                unsafe { &*edge_store }.mask_to_select_entire_adjacency_matrix_ref(),
                options,
            )?)
    }
}

pub trait SelectFromMaskedAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
        argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: &EdgeTypeIndex,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain: ValueType> SelectFromMaskedAdjacencyMatrix<EvaluationDomain> for Graph
where
    MatrixSelector: SelectFromMatrix<EvaluationDomain>,
{
    fn by_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
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
            .matrix_selector()
            .apply(
                selector,
                selector_argument,
                adjacency_matrix_argument,
                accumlator,
                adjacency_matrix_product,
                adjacency_matrix_mask,
                options,
            )?)
    }

    fn by_unchecked_index(
        &mut self,
        selector: &impl IndexUnaryOperator<EvaluationDomain>,
        selector_argument: &EvaluationDomain,
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
            .matrix_selector()
            .apply(
                selector,
                selector_argument,
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
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::graph::graph::GraphTrait;
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn select_from_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u16>::apply(&mut graph).unwrap();
        let result_edge_type_index = AddEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        SelectFromAdjacencyMatrix::by_index(
            &mut graph,
            &IsValueGreaterThan::<u8>::new(),
            &1,
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
                    vertex_1_index,
                    vertex_2_index,
                ),
            )
            .unwrap(),
            None
        );

        for adjacency_matrix in graph.edge_store_ref().adjacency_matrices_ref().into_iter() {
            println!("{}", adjacency_matrix);
        }
        
        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_2_index,
                    vertex_1_index,
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
