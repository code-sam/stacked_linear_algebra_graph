use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::versioned_graph::indexing::{GetVersionedVertexTypeIndex, VersionedVertexTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait MonoidElementWiseVertexVectorMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVersionedVertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVersionedVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

pub trait MonoidElementWiseVertexVectorMultiplication<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait MonoidElementWiseVertexVectorMultiplicationUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
