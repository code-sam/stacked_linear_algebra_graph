use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{binary_operator::AccumulatorBinaryOperator, mask::MatrixMask},
};

use crate::{graph::indexing::{
    GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex,
}, versioned_graph::indexing::{GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex, VersionedVertexTypeIndex}};
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait SelectEdgesWithTailVertexVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetVersionedEdgeTypeIndex,
        tail_vertex: &impl GetVersionedVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

pub trait SelectEdgesWithTailVertex<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        tail_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait SelectEdgesWithTailVertexUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        tail_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
