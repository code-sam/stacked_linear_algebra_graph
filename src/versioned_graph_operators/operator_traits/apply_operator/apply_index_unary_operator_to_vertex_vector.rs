use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::index_unary_operator::IndexUnaryOperator;
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;

use crate::error::GraphComputingError;
use crate::graph::indexing::{GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::{
    GetVersionedVertexTypeIndex, VersionedVertexTypeIndex,
};

pub trait ApplyIndexUnaryOperatorToVersionedVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        vertex_vector: &impl GetVersionedVertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVersionedVertexTypeIndex,
        mask: Option<&VersionedVertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}


#[cfg(test)]
mod tests {}
