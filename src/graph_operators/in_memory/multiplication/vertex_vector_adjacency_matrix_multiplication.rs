use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyVectorByMatrix;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::traits::traits::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::{
    ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
};
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
    GetVertexStore, Graph, GraphblasOperatorApplierCollection,
};

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::traits::vertex_type::{CheckVertexTypeIndex, GetVertexVector};
use crate::graph_operators::operator_traits::indexing::CheckIndex;
use crate::graph_operators::operator_traits::multiplication::{
    VertexVectorAdjacencyMatrixMultiplication, VertexVectorAdjacencyMatrixMultiplicationUnchecked,
};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixAsRightArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain> VertexVectorAdjacencyMatrixMultiplication<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        apply_vertex_vector_adjacency_matrix_multiplication::<EvaluationDomain>(
            &mut self.public_edge_store,
            &mut self.public_vertex_store,
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

impl<EvaluationDomain> VertexVectorAdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
    for Graph
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        apply_vertex_vector_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            &mut self.public_vertex_store,
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

pub(crate) fn apply_vertex_vector_adjacency_matrix_multiplication<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    vertex_store.try_vertex_type_index_validity(left_argument)?;
    edge_store.try_edge_type_index_validity(right_argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_vertex_vector_adjacency_matrix_multiplication_unchecked::<EvaluationDomain>(
        edge_store,
        vertex_store,
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

pub(crate) fn apply_vertex_vector_adjacency_matrix_multiplication_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    vertex_store: *mut impl GetVertexVector,
    left_argument: &impl GetVertexTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetEdgeTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let vertex_vector_left_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

    let adjacency_matrix_argument =
        ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::create_unchecked(
            edge_store,
            right_argument,
            options,
        );

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .vector_matrix_multiplication_operator()
                .apply(
                    vertex_vector_left_argument,
                    operator,
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    accumlator,
                    vertex_vector_product,
                    vertex_vector_mask,
                    options,
                )?)
        }
        None => {
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
                .vector_matrix_multiplication_operator()
                .apply(
                    vertex_vector_left_argument,
                    operator,
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    accumlator,
                    vertex_vector_product,
                    vertex_vector_mask,
                    options,
                )?)
        }
    }
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn multiply_vertex_vector_with_adjacency_matrix() {
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
        let _result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

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

        VertexVectorAdjacencyMatrixMultiplication::<u8>::by_index(
            &mut graph,
            &vertex_type_1_index,
            &PlusTimes::<u8>::new(),
            &edge_type_1_index,
            &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
            &vertex_type_1_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsRightArgument::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<u16>::with_key(&graph, vertex_type_key).unwrap()
        // );

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_1_index)
                .unwrap(),
            Some(5)
        );
    }
}
