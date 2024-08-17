use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait ApplyUnaryOperatorToVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait ApplyUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
