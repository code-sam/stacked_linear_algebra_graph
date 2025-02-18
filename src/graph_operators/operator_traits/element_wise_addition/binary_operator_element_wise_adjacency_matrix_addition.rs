use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, VersionedEdgeTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};


pub trait BinaryOperatorElementWiseAdjacencyMatrixAddition<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait BinaryOperatorElementWiseAdjacencyMatrixAdditionUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
