use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionSemiring;
use graphblas_sparse_linear_algebra::operators::element_wise_multiplication::ApplyElementWiseMatrixMultiplicationSemiring;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::ArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::CreateArgumentsForAdjacencyMatricesOperator;
use crate::graph::edge_store::GetArgumentsForAdjacencyMatricesOperator;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollection;
use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::operator_traits::element_wise_multiplication::SemiringElementWiseAdjacencyMatrixMultiplication;
use crate::operators::operator_traits::element_wise_multiplication::SemiringElementWiseAdjacencyMatrixMultiplicationUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain: ValueType> SemiringElementWiseAdjacencyMatrixMultiplication<EvaluationDomain>
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
        apply_semiring_element_wise_adjacency_matrix_multiplication::<EvaluationDomain>(
            &mut self.public_edge_store,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<EvaluationDomain: ValueType>
    SemiringElementWiseAdjacencyMatrixMultiplicationUnchecked<EvaluationDomain> for Graph
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
        apply_semiring_element_wise_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn apply_semiring_element_wise_adjacency_matrix_multiplication<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    edge_store.try_edge_type_index_validity(left_argument)?;
    edge_store.try_edge_type_index_validity(right_argument)?;
    edge_store.try_edge_type_index_validity(product)?;
    edge_store.try_optional_edge_type_index_validity(mask)?;

    apply_semiring_element_wise_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
        edge_store,
        left_argument,
        operator,
        right_argument,
        accumlator,
        product,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn apply_semiring_element_wise_adjacency_matrix_multiplication_unchecked<
    EvaluationDomain,
>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetEdgeTypeIndex,
    mask: Option<&EdgeTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let adjacency_matrix_arguments = ArgumentsForAdjacencyMatricesOperator::create_unchecked(
        edge_store,
        left_argument,
        right_argument,
        options,
    );

    let adjacency_matrix_product =
        unsafe { &mut *edge_store }.adjacency_matrix_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let adjacency_matrix_mask =
                unsafe { &*edge_store }.adjacency_matrix_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .element_wise_matrix_multiplication_semiring_operator()
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
            let adjacency_matrix_mask =
                graphblas_operator_applier_collection.entire_matrix_selector();

            Ok(graphblas_operator_applier_collection
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

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::operators::operator_traits::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operator_traits::read::{GetEdgeWeight, GetSparseAdjacencyMatrix};

    #[test]
    fn semiring_element_wise_adjacency_matrix_multiplication() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u16>::apply(&mut graph).unwrap();
        let result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

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

        for _i in 0..2 {
            SemiringElementWiseAdjacencyMatrixMultiplication::<u8>::apply(
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

        SemiringElementWiseAdjacencyMatrixMultiplication::<u8>::apply(
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
