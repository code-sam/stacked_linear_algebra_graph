use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyIndexUnaryOperator, IndexUnaryOperatorApplier},
    binary_operator::AccumulatorBinaryOperator,
    index_unary_operator::IndexUnaryOperator,
};

use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatrixOperator, CreateArgumentsForAdjacencyMatrixOperator,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
};
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operators::{
    indexing::CheckIndex, options::OptionsForOperatorWithAdjacencyMatrixArgument,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrix;
use crate::operators::operators::apply_operator::ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked;

impl<EvaluationDomain: ValueType> ApplyIndexUnaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(adjacency_matrix)?;
        self.try_edge_type_index_validity(product)?;
        self.try_optional_edge_type_index_validity(mask)?;

        ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked::<EvaluationDomain>::apply(
            self,
            adjacency_matrix,
            operator,
            argument,
            accumlator,
            product,
            mask,
            options,
        )
    }
}

impl<EvaluationDomain: ValueType>
    ApplyIndexUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain> for Graph
where
    IndexUnaryOperatorApplier: ApplyIndexUnaryOperator<EvaluationDomain>,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let operator_argument = ArgumentsForAdjacencyMatrixOperator::create_unchecked(
            edge_store,
            adjacency_matrix,
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
                    .index_unary_operator_applier()
                    .apply_to_matrix(
                        operator_argument.adjacency_matrix_ref(),
                        operator,
                        argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        operator_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .index_unary_operator_applier()
                    .apply_to_matrix(
                        operator_argument.adjacency_matrix_ref(),
                        operator,
                        argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        operator_argument.options_ref(),
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::index_unary_operator::IsValueGreaterThan;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::operators::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn add_scalar_to_adjacency_matrix() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

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

        ApplyIndexUnaryOperatorToAdjacencyMatrix::<f32>::apply(
            &mut graph,
            &edge_type_1_index,
            &IsValueGreaterThan::<f32>::new(),
            &1f32,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     WeightedAdjacencyMatrixSparseMatrixTrait::<u16>::sparse_matrix_ref(
        //         graph
        //             .edge_store_ref()
        //             .adjacency_matrix_ref_for_key(result_type_key)
        //             .unwrap()
        //     )
        //     .get_element_list()
        //     .unwrap()
        // );

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
            Some(1)
        );
    }
}
