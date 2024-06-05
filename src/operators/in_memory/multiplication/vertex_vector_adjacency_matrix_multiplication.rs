use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyVectorByMatrix;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::{
    ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
};
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
    GetVertexStore, Graph,
};

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::operators::operators::multiplication::{
    VertexVectorAdjacencyMatrixMultiplication, VertexVectorAdjacencyMatrixMultiplicationUnchecked,
};
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixAsRightArgument;
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
        self.try_vertex_type_index_validity(left_argument)?;
        self.try_edge_type_index_validity(right_argument)?;
        self.try_vertex_type_index_validity(product)?;
        self.try_optional_vertex_type_index_validity(mask)?;

        VertexVectorAdjacencyMatrixMultiplicationUnchecked::apply(
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
        let edge_store = self.edge_store_mut_ref_unsafe();
        let vertex_store = self.vertex_store_mut_ref_unsafe();

        let vertex_vector_left_argument =
            unsafe { &*vertex_store }.vertex_vector_ref_unchecked(left_argument);

        let adjacency_matrix_argument =
            ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::create_unchecked(
                edge_store,
                right_argument,
                options,
            );

        let vertex_vector_product =
            unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(product);

        match mask {
            Some(mask) => {
                let vertex_vector_mask =
                    unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

                Ok(self
                    .graphblas_operator_applier_collection_ref()
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
                let vertex_vector_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_vector_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
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
}

#[cfg(test)]
mod tests {

    use graphblas_sparse_linear_algebra::operators::semiring::PlusTimes;

    use super::*;

    use crate::operators::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::operators::read::GetVertexValue;

    #[test]
    fn multiply_vertex_vector_with_adjacency_matrix() {
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
        let _result_edge_type_index = AddEdgeType::<f32>::apply(&mut graph).unwrap();

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
