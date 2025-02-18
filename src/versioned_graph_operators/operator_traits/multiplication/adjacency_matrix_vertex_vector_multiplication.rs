use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::indexing::VertexTypeIndex;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::GetVersionedVertexTypeIndex;
use crate::versioned_graph::indexing::VersionedVertexTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait AdjacencyMatrixVertexVectorMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVersionedEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVersionedVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
