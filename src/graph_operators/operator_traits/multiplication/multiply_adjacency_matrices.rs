use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};


pub trait AdjacencyMatrixMultiplication<EvaluationDomain>
where
    EvaluationDomain: ValueType,
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
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait AdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
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
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
