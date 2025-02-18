use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::versioned_graph::indexing::{GetVersionedVertexTypeIndex, VersionedVertexTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait SemiringElementWiseVertexVectorMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &impl GetVersionedVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVersionedVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
