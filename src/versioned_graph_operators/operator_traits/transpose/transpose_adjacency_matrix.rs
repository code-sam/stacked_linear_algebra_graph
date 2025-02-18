use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;

use crate::graph::indexing::EdgeTypeIndex;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait TransposeAdjacencyMatrixVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
