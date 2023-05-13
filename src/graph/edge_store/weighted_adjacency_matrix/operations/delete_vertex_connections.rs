use graphblas_sparse_linear_algebra::collections::sparse_matrix::{Coordinate, SparseMatrixTrait};
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
use crate::graph::value_type::{
    implement_1_type_macro_with_2_typed_indentifiers_for_all_value_types,
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

macro_rules! create_insert_vector_into_column_operators {
    ($operator_identifier:ident, $value_type:ty) => {
        static $operator_identifier: Lazy<InsertVectorIntoColumn<$value_type, $value_type>> =
            Lazy::new(|| {
                InsertVectorIntoColumn::<$value_type, $value_type>::new(
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                    &Assignment::new(),
                )
            });
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    create_insert_vector_into_column_operators,
    INSERT_VECTOR_INTO_COLUMN_OPERATOR
);

macro_rules! create_insert_vector_into_row_operators {
    ($operator_identifier:ident, $value_type:ty) => {
        static $operator_identifier: Lazy<InsertVectorIntoRow<$value_type, $value_type>> =
            Lazy::new(|| {
                InsertVectorIntoRow::<$value_type, $value_type>::new(
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                    &Assignment::new(),
                )
            });
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    create_insert_vector_into_row_operators,
    INSERT_VECTOR_INTO_ROW_OPERATOR
);

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
    ($insert_vector_into_column_operator_identifier:ident,
        $insert_vector_into_row_operator_identifier:ident,
        $value_type:ty) => {
        impl DeleteVertexConnections<$value_type> for WeightedAdjacencyMatrix {
            fn delete_vertex_connections_unchecked(
                &mut self,
                vertex_index: &VertexIndex,
            ) -> Result<(), GraphComputingError> {
                let empty_column = SparseVector::<$value_type>::new(
                    &self.graphblas_context_ref(),
                    &self.vertex_capacity()?,
                )?;

                // TODO: is inserting an empty vector the fastest way to delete a row/column?
                $insert_vector_into_column_operator_identifier.apply(
                    self.sparse_matrix_mut_ref(),
                    &ElementIndexSelector::All,
                    vertex_index,
                    &empty_column,
                )?;
                $insert_vector_into_row_operator_identifier.apply(
                    self.sparse_matrix_mut_ref(),
                    &ElementIndexSelector::All,
                    vertex_index,
                    &empty_column,
                )?;
                Ok(())
            }
        }
    };
}
implement_1_type_macro_with_2_typed_indentifiers_for_all_value_types!(
    implement_delete_vertex_connections,
    INSERT_VECTOR_INTO_COLUMN_OPERATOR,
    INSERT_VECTOR_INTO_ROW_OPERATOR
);

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
