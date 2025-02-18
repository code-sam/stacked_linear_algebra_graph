use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::monoid::Monoid;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait MonoidElementWiseAdjacencyMatrixMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &impl GetVersionedEdgeTypeIndex,
        operator: &impl Monoid<EvaluationDomain>,
        right_argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
