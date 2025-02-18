use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, VersionedEdgeTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait BinaryOperatorElementWiseAdjacencyMatrixMultiplicationVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVersionedEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
