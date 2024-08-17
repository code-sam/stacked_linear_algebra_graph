use graphblas_sparse_linear_algebra::operators::{
    binary_operator::AccumulatorBinaryOperator,
    index_unary_operator::IndexUnaryOperator,
    options::OperatorOptions,
};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{GetVertexTypeIndex, VertexTypeIndex},
        value_type::ValueType,
    },
};

pub trait ApplyIndexUnaryOperatorToVertexVector<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        vertex_vector: &impl GetVertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait ApplyIndexUnaryOperatorToVertexVectorUnchecked<EvaluationDomain>
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        vertex_vector: &impl GetVertexTypeIndex,
        operator: &impl IndexUnaryOperator<EvaluationDomain>,
        argument: &EvaluationDomain,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        product: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OperatorOptions,
    ) -> Result<(), GraphComputingError>;
}

#[cfg(test)]
mod tests {}
