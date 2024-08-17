use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatrixOperator, CreateArgumentsForAdjacencyMatrixOperator,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::GetVertexVector;
use crate::operators::indexing::CheckIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector as VertexSelector;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixColumn;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{binary_operator::AccumulatorBinaryOperator, mask::MatrixMask},
};

use crate::graph::graph::Graph;
use crate::graph::graph::{
    GetEdgeStore, GetGraphblasOperatorApplierCollection, GetGraphblasOperatorAppliers,
    GetVertexStore,
};
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait SelectEdgesWithHeadVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        head_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&impl GetVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait SelectEdgesWithHeadVertexUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        head_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&impl GetVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
