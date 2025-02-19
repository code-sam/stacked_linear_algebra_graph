use crate::graph::indexing::{
    GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex,
};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use crate::versioned_graph::indexing::{
    GetVersionedEdgeTypeIndex, GetVersionedVertexIndexIndex, GetVersionedVertexTypeIndex,
    VersionedVertexTypeIndex,
};
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::SparseMatrix,
    operators::{binary_operator::AccumulatorBinaryOperator, mask::MatrixMask},
};

use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait SelectEdgesWithHeadVertexVersioned<EvaluationDomain>
where
    EvaluationDomain: ValueType,
    SparseMatrix<EvaluationDomain>: MatrixMask,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetVersionedEdgeTypeIndex,
        head_vertex: &impl GetVersionedVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
