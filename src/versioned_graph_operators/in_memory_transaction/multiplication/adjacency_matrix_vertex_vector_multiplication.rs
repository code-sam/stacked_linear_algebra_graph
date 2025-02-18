use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::indexing::VertexTypeIndex;
use crate::operators::in_memory::multiplication::apply_adjacency_matrix_vertex_vector_multiplication;
use crate::operators::in_memory::multiplication::apply_adjacency_matrix_vertex_vector_multiplication_unchecked;
use crate::operators::operator_traits::multiplication::AdjacencyMatrixVertexVectorMultiplication;
use crate::operators::operator_traits::multiplication::AdjacencyMatrixVertexVectorMultiplicationUnchecked;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::transaction::in_memory::InMemoryGraphTransaction;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<'g, EvaluationDomain: ValueType> AdjacencyMatrixVertexVectorMultiplication<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
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
            &mut self.edge_store_transaction,
            &mut self.vertex_store_transaction,
            left_argument,
            operator,
            right_argument,
            accumlator,
            product,
            mask,
            options,
            self.graphblas_operator_applier_collection,
        )
    }
}

impl<'g, EvaluationDomain: ValueType>
    AdjacencyMatrixVertexVectorMultiplicationUnchecked<EvaluationDomain>
    for InMemoryGraphTransaction<'g>
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
            &mut self.edge_store_transaction,
            &mut self.vertex_store_transaction,
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

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::graph::graph::Graph;
    use crate::operators::operator_traits::new::{NewEdge, NewEdgeType, NewVertex, NewVertexType};
    use crate::operators::operator_traits::read::GetVertexValue;

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
