use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::graph::value_type::ValueType;
use crate::operators::options::{
    OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
};
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, VersionedEdgeTypeIndex};

pub trait ApplyScalarBinaryOperatorToVersionedAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &impl GetVersionedEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

pub trait ApplyScalarBinaryOperatorToAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn with_adjacency_matrix_as_left_argument(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument(
        &mut self,
        left_argument: EvaluationDomain,
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
        right_argument: EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;

    fn with_adjacency_matrix_as_right_argument_and_by_unchecked_index(
        &mut self,
        left_argument: EvaluationDomain,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
