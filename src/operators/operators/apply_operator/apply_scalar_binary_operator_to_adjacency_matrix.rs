use graphblas_sparse_linear_algebra::operators::{
    apply::{ApplyBinaryOperator as ApplyGraphBlasBinaryOperator, BinaryOperatorApplier},
    binary_operator::{AccumulatorBinaryOperator, BinaryOperator},
};

use crate::graph::{
    edge_store::{
        operations::get_adjacency_matrix::GetAdjacencyMatrix,
        ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
        ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
        CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument,
        CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
        GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument,
        GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
    },
    indexing::{EdgeTypeIndex, GetEdgeTypeIndex},
};
use crate::{
    error::GraphComputingError,
    graph::{graph::Graph, value_type::ValueType},
    operators::options::{
        OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
        OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    },
};
use crate::{
    graph::graph::{
        GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
    },
    operators::indexing::CheckIndex,
};

pub trait ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait ApplyScalarBinaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn with_adjacency_matrix_as_left_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: &EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {
    
}
