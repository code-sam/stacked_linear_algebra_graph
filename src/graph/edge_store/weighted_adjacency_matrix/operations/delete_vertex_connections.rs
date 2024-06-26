use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::insert::InsertVectorIntoColumn;
use graphblas_sparse_linear_algebra::operators::insert::InsertVectorIntoRow;
use graphblas_sparse_linear_algebra::operators::insert::{
    InsertVectorIntoColumnOperator, InsertVectorIntoRowOperator,
};
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::operators::options::OptionsForOperatorWithAdjacencyMatrixArgument;

use super::GetMatrixSize;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OptionsForOperatorWithAdjacencyMatrixArgument> =
    Lazy::new(|| OptionsForOperatorWithAdjacencyMatrixArgument::new_default());

static INSERT_VECTOR_INTO_COLUMN_OPERATOR: Lazy<InsertVectorIntoColumnOperator> =
    Lazy::new(|| InsertVectorIntoColumnOperator::new());

static INSERT_VECTOR_INTO_ROW_OPERATOR: Lazy<InsertVectorIntoRowOperator> =
    Lazy::new(|| InsertVectorIntoRowOperator::new());

// TODO: this doesn't work because Lazy generates a one-off type that doesn't implement AccumulatorBinaryOperator.
// static BOOLEAN_ASSIGNMENT_OPERATOR: Lazy<Assignment<bool>> = Lazy::new(|| Assignment::<bool>::new());

static OPERATOR_CACHE: Lazy<OperatorCache> = Lazy::new(|| OperatorCache::new());

pub(crate) trait DeleteVertexConnections {
    fn delete_vertex_connections_unchecked(
        &mut self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexConnections for WeightedAdjacencyMatrix {
    fn delete_vertex_connections_unchecked(
        &mut self,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        // TODO: does the value type mismatch actually cause a performance penalty? Since the vector is empty, it may not.
        // TODO: is there a benefit to caching an empty vector (and matrix) in the edge/vertex store? The cached vector/matrix
        // could be selected by the ValueTypeIdentifier.
        // TODO: a vector/matrix of the correct size and value type could be generated by the ValueTypeIdentifier,
        // this would cost a match statement.
        let empty_column =
            SparseVector::<bool>::new(&self.graphblas_context_ref(), &self.vertex_capacity()?)?;

        // TODO: is inserting an empty vector the fastest way to delete a row/column?
        INSERT_VECTOR_INTO_COLUMN_OPERATOR.apply(
            self,
            &ElementIndexSelector::All,
            vertex_index.index_ref(),
            &empty_column,
            &OPERATOR_CACHE.boolean_assignment,
            &SelectEntireVector::new(self.graphblas_context_ref()), // TODO: could the mask be cached for better performance?
            &*DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
        )?;
        INSERT_VECTOR_INTO_ROW_OPERATOR.apply(
            self,
            &ElementIndexSelector::All,
            vertex_index.index_ref(),
            &empty_column,
            &OPERATOR_CACHE.boolean_assignment,
            &SelectEntireVector::new(self.graphblas_context_ref()), // TODO: could the mask be cached for better performance?
            &*DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
        )?;
        Ok(())
    }
}

struct OperatorCache {
    boolean_assignment: Assignment<bool>,
}

impl OperatorCache {
    fn new() -> OperatorCache {
        OperatorCache {
            boolean_assignment: Assignment::<bool>::new(),
        }
    }
}
