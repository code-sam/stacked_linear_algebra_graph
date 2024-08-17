use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait ApplyUnaryOperatorToAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait ApplyUnaryOperatorToAdjacencyMatrixUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
