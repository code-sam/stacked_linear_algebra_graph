use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyMatrixByVector;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::ArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::edge_store::CreateArgumentsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::edge_store::GetArgumentForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::graph::graph::GetEdgeStore;
use crate::graph::graph::GetGraphblasOperatorApplierCollection;
use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::GetVertexStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixAsLeftArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait AdjacencyMatrixVertexVectorMultiplication<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait AdjacencyMatrixVertexVectorMultiplicationUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetEdgeTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetVertexTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsLeftArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
