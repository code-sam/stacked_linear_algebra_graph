use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector;
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::insert::{
    InsertVectorIntoColumn, InsertVectorIntoColumnTrait,
};
use graphblas_sparse_linear_algebra::operators::insert::{
    InsertVectorIntoRow, InsertVectorIntoRowTrait,
};
use graphblas_sparse_linear_algebra::operators::options::OperatorOptions;
use once_cell::sync::Lazy;

use crate::error::GraphComputingError;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    WeightedAdjacencyMatrix, WeightedAdjacencyMatrixSparseMatrixTrait, WeightedAdjacencyMatrixTrait,
};
use crate::graph::graph::VertexIndex;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static INSERT_VECTOR_INTO_COLUMN_OPERATOR: Lazy<InsertVectorIntoColumn> =
    Lazy::new(|| InsertVectorIntoColumn::new());

static INSERT_VECTOR_INTO_ROW_OPERATOR: Lazy<InsertVectorIntoRow> =
    Lazy::new(|| InsertVectorIntoRow::new());

pub(crate) trait DeleteVertexConnections<T: ValueType> {
    fn delete_vertex_connections_unchecked(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexConnectionsForAllTypes {
    fn delete_vertex_connections_for_all_value_types_unchecked(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_delete_vertex_connections {
    ($value_type:ty) => {
        impl DeleteVertexConnections<$value_type> for WeightedAdjacencyMatrix {
            fn delete_vertex_connections_unchecked(
                &mut self,
                vertex_index: &VertexIndex,
            ) -> Result<(), GraphComputingError> {
                let empty_column = SparseVector::<$value_type>::new(
                    &self.graphblas_context_ref(),
                    &self.vertex_capacity()?,
                )?;

                // TODO: cache the accumulator for better performance
                let accumulator = Assignment::<$value_type>::new();

                // TODO: is inserting an empty vector the fastest way to delete a row/column?
                INSERT_VECTOR_INTO_COLUMN_OPERATOR.apply(
                    self.sparse_matrix_mut_ref(),
                    &ElementIndexSelector::All,
                    vertex_index,
                    &empty_column,
                    &accumulator,
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;
                INSERT_VECTOR_INTO_ROW_OPERATOR.apply(
                    self.sparse_matrix_mut_ref(),
                    &ElementIndexSelector::All,
                    vertex_index,
                    &empty_column,
                    &accumulator,
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_delete_vertex_connections);

impl DeleteVertexConnectionsForAllTypes for WeightedAdjacencyMatrix {
    fn delete_vertex_connections_for_all_value_types_unchecked(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        DeleteVertexConnections::<bool>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<i8>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<i16>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<i32>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<i64>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<u8>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<u16>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<u32>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<u64>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<f32>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<f64>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<isize>::delete_vertex_connections_unchecked(self, vertex_index)?;
        DeleteVertexConnections::<usize>::delete_vertex_connections_unchecked(self, vertex_index)?;
        Ok(())
    }
}
