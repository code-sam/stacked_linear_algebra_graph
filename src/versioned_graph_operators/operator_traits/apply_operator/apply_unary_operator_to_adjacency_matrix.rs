use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, unary_operator::UnaryOperator,
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, VersionedEdgeTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait ApplyUnaryOperatorToVersionedAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        operator: &impl UnaryOperator<EvaluationDomain>,
        argument: &impl GetVersionedEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
