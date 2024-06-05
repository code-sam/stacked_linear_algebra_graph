use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::multiplication::MultiplyVectorByMatrix;
use graphblas_sparse_linear_algebra::operators::semiring::Semiring;

use crate::graph::edge_store::{
    ArgumentsForOperatorWithAdjacencyMatrixAsSecondArgument,
    CreateArgumentsForOperatorWithAdjacencyMatrixAsRightArgument,
    GetArgumentForOperatorWithAdjacencyMatrixAsSecondArgument,
};
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
    GetVertexStore, Graph,
};

use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixAsRightArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait VertexVectorAdjacencyMatrixMultiplication<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn by_index(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait VertexVectorAdjacencyMatrixMultiplicationUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        left_argument: &impl GetVertexTypeIndex,
        operator: &impl Semiring<EvaluationDomain>,
        right_argument: &impl GetEdgeTypeIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixAsRightArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
