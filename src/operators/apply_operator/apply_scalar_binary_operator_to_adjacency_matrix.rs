use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
};

use crate::graph::edge_store::{
    operations::get_adjacency_matrix::GetAdjacencyMatrix,
    ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
    CreateArgumentsForAdjacencyMatrixOperator,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{EdgeTypeIndex, Graph},
        value_type::ValueType,
    },
    operators::options::{
        OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
        OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    },
};

pub trait ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_left_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

impl<EvaluationDomain: ValueType> ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
    for Graph
where
    BinaryOperatorApplier: ApplyGraphBlasBinaryOperator<EvaluationDomain>,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::try_create(
                edge_store,
                left_argument,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        operator,
                        right_argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        operator,
                        right_argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
        }
    }

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        // DESIGN NOTE: A GraphBLAS implementation provides the implementation of the operator.
        // The GraphBLAS C API requires passing references to operands, and a mutable reference to the result.
        // This API is not compatible with safe Rust, unless significant performance penalties would be acceptable.
        // For example, an alternative to unsafe access would be to clone the operands.
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::try_create(
                edge_store,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
        }
    }

    fn with_adjacency_matrix_as_left_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &EdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument::create_unchecked(
                edge_store,
                left_argument,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        operator,
                        right_argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_matrix_as_left_argument(
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        operator,
                        right_argument,
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
        }
    }

    fn with_adjacency_matrix_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &EdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError> {
        let edge_store = self.edge_store_mut_ref_unsafe();

        let adjacency_matrix_argument =
            ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument::create_unchecked(
                edge_store,
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
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
            None => {
                let adjacency_matrix_mask = self
                    .graphblas_operator_applier_collection_ref()
                    .entire_matrix_selector();

                Ok(self
                    .graphblas_operator_applier_collection_ref()
                    .binary_operator_applier()
                    .apply_with_matrix_as_right_argument(
                        left_argument,
                        operator,
                        adjacency_matrix_argument.adjacency_matrix_ref(),
                        accumlator,
                        adjacency_matrix_product,
                        adjacency_matrix_mask,
                        adjacency_matrix_argument.options_ref(),
                    )?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinate;
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::GetEdgeWeight;

    #[test]
    fn add_scalar_to_adjacency_matrix() {
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

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_1_index,
            &Plus::<u8>::new(),
            &1,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
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
            Some(2)
        );

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_2_index,
            &Plus::<u8>::new(),
            &u8::MAX,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
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
            Some(2)
        );

        ApplyScalarBinaryOperatorToAdjacencyMatrix::<u8>::with_adjacency_matrix_as_left_argument(
            &mut graph,
            &edge_type_1_index,
            &Plus::<u8>::new(),
            &1u8,
            &Assignment::new(),
            &result_edge_type_index,
            None,
            &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetEdgeWeight::<usize>::edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    result_edge_type_index,
                    vertex_2_index,
                    vertex_1_index,
                ),
            )
            .unwrap(),
            Some(3)
        )
    }
}
