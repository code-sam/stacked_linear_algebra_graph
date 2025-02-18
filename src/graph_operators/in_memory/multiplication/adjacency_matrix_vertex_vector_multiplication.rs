use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrixByVector;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::edge_store::CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::edge_store::GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::Graph;
use crate::graph::graph::GraphblasOperatorApplierCollection;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::CheckVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph_operators::operator_traits::multiplication::AdjacencyMatrixVertexVectorMultiplication;
use crate::graph_operators::operator_traits::multiplication::AdjacencyMatrixVertexVectorMultiplicationUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain: ValueType> AdjacencyMatrixVertexVectorMultiplication<EvaluationDomain>
    for Graph
{
    /// NOTE: relatively slow because graph holds adjacency matrix by row. Where possible, consider using vector * tranpose(matrix) through VertexVectorAdjacencyMatrixMultiplication instead.
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        apply_adjacency_matrix_vertex_vector_multiplication::<EvaluationDomain>(
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

impl<EvaluationDomain: ValueType>
    AdjacencyMatrixVertexVectorMultiplicationUnchecked<EvaluationDomain> for Graph
{
    /// NOTE: relatively slow because graph holds adjacency matrix by row. Where possible, consider using vector * tranpose(matrix) through VertexVectorAdjacencyMatrixMultiplication instead.
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        apply_adjacency_matrix_vertex_vector_multiplication_unchecked::<EvaluationDomain>(
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

/// NOTE: relatively slow because graph holds adjacency matrix by row. Where possible, consider using vector * tranpose(matrix) through VertexVectorAdjacencyMatrixMultiplication instead.
pub(crate) fn apply_adjacency_matrix_vertex_vector_multiplication<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex),
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    edge_store.try_edge_type_index_validity(left_argument)?;
    vertex_store.try_vertex_type_index_validity(right_argument)?;
    vertex_store.try_vertex_type_index_validity(product)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    apply_adjacency_matrix_vertex_vector_multiplication_unchecked::<EvaluationDomain>(
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

/// NOTE: relatively slow because graph holds adjacency matrix by row. Where possible, consider using vector * tranpose(matrix) through VertexVectorAdjacencyMatrixMultiplication instead.
pub(crate) fn apply_adjacency_matrix_vertex_vector_multiplication_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    vertex_store: *mut impl GetVertexVector,
    left_argument: &impl GetEdgeTypeIndex,
    operator: &impl Semiring<EvaluationDomain>,
    right_argument: &impl GetVertexTypeIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    product: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let adjacency_matrix_argument =
        ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::create_unchecked(
            edge_store,
            left_argument,
            options,
        );

    let vertex_vector_right_argument =
        unsafe { &*vertex_store }.vertex_vector_ref_unchecked(right_argument);

    let vertex_vector_product =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .matrix_vector_multiplication_operator()
                .apply(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    operator,
                    vertex_vector_right_argument,
                    accumlator,
                    vertex_vector_product,
                    vertex_vector_mask,
                    adjacency_matrix_argument.options_ref(),
                )?)
        }
        None => {
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
                .matrix_vector_multiplication_operator()
                .apply(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    operator,
                    vertex_vector_right_argument,
                    accumlator,
                    vertex_vector_product,
                    vertex_vector_mask,
                    adjacency_matrix_argument.options_ref(),
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

        AdjacencyMatrixVertexVectorMultiplication::<u8>::apply(
            &mut graph,
            &edge_type_1_index,
            &PlusTimes::<u8>::new(),
            &vertex_type_1_index,
            &graphblas_sparse_linear_algebra::operators::binary_operator::Plus::<u8>::new(),
            &vertex_type_1_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
        )
        .unwrap();

        // println!("{:?}", ReadAdjacencyMatrixElementList::<u16>::with_key(&graph, edge_type_1_key).unwrap());

        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<u16>::with_key(&graph, vertex_type_key).unwrap()
        // );

        assert_eq!(
            GetVertexValue::<u16>::vertex_value(&graph, &vertex_type_1_index, &vertex_1_index)
                .unwrap(),
            Some(3)
        );
    }
}
