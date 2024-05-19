use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrices;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;

use crate::graph::edge_store::ArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::CreateArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::GetArgumentsForAdjacencyMatricesOperator;
use crate::graph::graph::GetEdgeStore;
use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::indexing::CheckIndex;
use crate::operators::operators::multiplication::AdjacencyMatrixMultiplication;
use crate::operators::operators::multiplication::AdjacencyMatrixMultiplicationUnchecked;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::{error::GraphComputingError, graph::value_type::ValueType};


impl<EvaluationDomain: ValueType> AdjacencyMatrixMultiplication<EvaluationDomain> for Graph {
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(left_argument)?;
        self.try_edge_type_index_validity(right_argument)?;
        self.try_edge_type_index_validity(product)?;
        self.try_optional_edge_type_index_validity(mask)?;

        AdjacencyMatrixMultiplicationUnchecked::apply(
            self,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
        )
    }
}

impl<EvaluationDomain: ValueType> AdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
    for Graph
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_arguments = ArgumentsForAdjacencyMatricesOperator::create_unchecked(
            edge_store,
            left_argument,
            right_argument,
            options,
        );

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product);

        match mask {
            Some(mask) => {
                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_multiplication_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        options,
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .matrix_multiplication_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        options,
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::operators::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn multiply_adjacency_matrices() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u16>::apply(&mut graph).unwrap();
        let result_edge_type_index = AddEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        AdjacencyMatrixMultiplication::<u8>::apply(
            &mut graph,
            &edge_type_1_index,
            &PlusTimes::<u8>::new(),
            &edge_type_1_index,
            &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArguments::new_default(),
        )
        .unwrap();

        // println!("{:?}", ReadAdjacencyMatrixElementList::<u16>::with_key(&graph, result_type_key).unwrap());

        assert_eq!(
            GetEdgeWeight::<u16>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_1_index,
                    vertex_1_index,
                ),
            )
            .unwrap(),
            Some(2)
        );
    }
}
