use graphblas_sparse_linear_algebra::operators::binary_operator::{
    AccumulatorBinaryOperator, BinaryOperator,
};
use graphblas_sparse_linear_algebra::operators::element_wise_addition::ApplyElementWiseMatrixAdditionBinaryOperator;

use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatricesOperator, CreateArgumentsForAdjacencyMatricesOperator,
    GetArgumentsForAdjacencyMatricesOperator,
};
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers, Graph,
};
use crate::graph::indexing::{EdgeTypeIndex, GetEdgeTypeIndex};
use crate::operators::indexing::CheckIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArguments;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait BinaryOperatorElementWiseAdjacencyMatrixAddition<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait BinaryOperatorElementWiseAdjacencyMatrixAdditionUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl BinaryOperator<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetEdgeTypeIndex,
        mask: Option<&EdgeTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArguments,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
