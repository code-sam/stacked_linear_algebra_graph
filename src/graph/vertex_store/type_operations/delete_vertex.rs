use graphblas_sparse_linear_algebra::operators::insert::InsertVectorIntoColumnTrait;
use graphblas_sparse_linear_algebra::{
    collections::{
        sparse_matrix::{Coordinate, SparseMatrixTrait},
        sparse_vector::SparseVector,
    },
    index::ElementIndexSelector,
    operators::{
        binary_operator::Assignment,
        insert::{InsertVectorIntoColumn, InsertVectorIntoRow},
        options::OperatorOptions,
    },
};
use once_cell::sync::Lazy;

use crate::graph::vertex_store::vertex_matrix::VertexMatrixTrait;
use crate::graph::vertex_store::VertexMatrix;
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{VertexIndex, VertexTypeIndex},
        value_type::{implement_macro_for_all_native_value_types, ValueType},
        vertex_store::SparseVertexMatrix,
    },
};

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static INSERT_VECTOR_INTO_COLUMN_OPERATOR: Lazy<InsertVectorIntoColumn> =
    Lazy::new(|| InsertVectorIntoColumn::new());

static INSERT_VECTOR_INTO_ROW_OPERATOR: Lazy<InsertVectorIntoRow> =
    Lazy::new(|| InsertVectorIntoRow::new());

pub(crate) trait DeleteVertex<T: ValueType> {
    fn delete_vertex(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_vertex_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_delete_vertex {
    ($value_type:ty) => {
        impl DeleteVertex<$value_type> for VertexMatrix {
            fn delete_vertex(
                &mut self,
                vertex_type_index: &VertexTypeIndex,
                vertex_index: &VertexIndex,
            ) -> Result<(), GraphComputingError> {
                Ok(
                    SparseVertexMatrix::<$value_type>::sparse_matrix_mut_ref(self).drop_element(
                        Coordinate::new(vertex_type_index.to_owned(), vertex_index.to_owned()),
                    )?,
                )
            }

            fn delete_vertex_for_all_vertex_types(
                &mut self,
                vertex_index: &VertexIndex,
            ) -> Result<(), GraphComputingError> {
                let empty_column = SparseVector::<$value_type>::new(
                    &self.graphblas_context_ref(),
                    &self.vertex_capacity()?,
                )?;

                // TODO: cache the accumulator for better performance
                let accumulator = Assignment::<$value_type>::new();

                Ok(INSERT_VECTOR_INTO_COLUMN_OPERATOR.apply(
                    SparseVertexMatrix::<$value_type>::sparse_matrix_mut_ref(self),
                    &ElementIndexSelector::All,
                    vertex_index,
                    &empty_column,
                    &accumulator,
                    &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
                )?)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_delete_vertex);

pub(crate) trait DeleteVertexForAllValueTypes {
    fn delete_vertex_for_all_value_types(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_for_all_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertexForAllValueTypes for VertexMatrix {
    fn delete_vertex_for_all_value_types(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        DeleteVertex::<bool>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<i8>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<i16>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<i32>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<i64>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<u8>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<u16>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<u32>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<u64>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<f32>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<f64>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<isize>::delete_vertex(self, vertex_type_index, vertex_index)?;
        DeleteVertex::<usize>::delete_vertex(self, vertex_type_index, vertex_index)?;
        Ok(())
    }

    fn delete_vertex_for_all_vertex_types_and_value_types(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        DeleteVertex::<bool>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<i8>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<i16>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<i32>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<i64>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<u8>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<u16>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<u32>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<u64>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<f32>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<f64>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<isize>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        DeleteVertex::<usize>::delete_vertex_for_all_vertex_types(self, vertex_index)?;
        Ok(())
    }
}
