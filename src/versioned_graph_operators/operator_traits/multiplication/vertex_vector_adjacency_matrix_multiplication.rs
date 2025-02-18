use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixAsRightArgument;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexTypeIndex, VersionedVertexTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait VertexVectorAdjacencyMatrixMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &impl GetVersionedVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
