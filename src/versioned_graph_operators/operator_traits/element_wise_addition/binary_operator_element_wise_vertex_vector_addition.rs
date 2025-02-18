use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::versioned_graph::indexing::{GetVersionedVertexTypeIndex, VersionedVertexTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait BinaryOperatorElementWiseVertexVectorAdditionVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVersionedVertexTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVersionedVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
