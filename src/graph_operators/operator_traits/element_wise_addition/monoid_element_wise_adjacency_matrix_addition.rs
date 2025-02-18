use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};


pub trait MonoidElementWiseAdjacencyMatrixAddition<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait MonoidElementWiseAdjacencyMatrixAdditionUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
