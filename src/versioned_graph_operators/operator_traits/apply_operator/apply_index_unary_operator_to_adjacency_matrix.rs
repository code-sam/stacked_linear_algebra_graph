use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator, index_unary_operator::IndexUnaryOperator,
};

use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::versioned_graph::indexing::{GetVersionedEdgeTypeIndex, VersionedEdgeTypeIndex};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait ApplyIndexUnaryOperatorToVersionedAdjacencyMatrix<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetVersionedEdgeTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedEdgeTypeIndex,
        mask: Option<&VersionedEdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
