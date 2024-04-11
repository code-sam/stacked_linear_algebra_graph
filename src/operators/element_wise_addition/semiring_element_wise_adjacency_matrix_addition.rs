use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionSemiring;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::ArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::CreateArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::GetArgumentsForAdjacencyMatricesOperator;
use crate::graph::graph::GetEdgeStore;
use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::{
    error::GraphComputingError,
    graph::{edge::EdgeTypeIndex, value_type::ValueType},
};

pub trait SemiringElementWiseAdjacencyMatrixAddition<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain: ValueType> SemiringElementWiseAdjacencyMatrixAddition<EvaluationDomain>
    for Graph
{
    fn by_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_arguments = ArgumentsForAdjacencyMatricesOperator::try_create(
            edge_store,
            left_argument,
            right_argument,
            options,
        )?;

        let adjacency_matrix_product =
            unsafe { &mut *edge_store }.try_adjacency_matrix_mut_ref(product)?;

        match mask {
            Some(mask) => {
                let adjacency_matrix_mask =
                    unsafe { &*edge_store }.try_adjacency_matrix_ref(mask)?;

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .element_wise_matrix_addition_semiring_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_arguments.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .element_wise_matrix_addition_semiring_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_arguments.options_ref(),
                    )?)
            }
        }
    }

    fn by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
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
                    .element_wise_matrix_addition_semiring_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_arguments.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .element_wise_matrix_addition_semiring_operator()
                    .apply(
                        adjacency_matrix_arguments.left_adjacency_matrix_ref(),
                        operator,
                        adjacency_matrix_arguments.right_adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_arguments.options_ref(),
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn semiring_element_wise_adjacency_matrix_addition() {
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

        for _i in 0..2 {
            SemiringElementWiseAdjacencyMatrixAddition::<u8>::by_index(
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
        }

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
            Some(4)
        );

        // println!("{}", graph.edge_store_ref().try_adjacency_matrix_ref_for_index(&_edge_type_1_index).unwrap());
        // println!("{}", graph.edge_store_ref().try_adjacency_matrix_ref_for_index(&_edge_type_2_index).unwrap());

        SemiringElementWiseAdjacencyMatrixAddition::<u8>::by_index(
            &mut graph,
            &edge_type_1_index,
            &PlusTimes::<u8>::new(),
            &edge_type_2_index,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixArguments::new_default(),
        )
        .unwrap();

        // println!("{}", graph.edge_store_ref().try_adjacency_matrix_ref_for_index(&_result_edge_type_index).unwrap());
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
