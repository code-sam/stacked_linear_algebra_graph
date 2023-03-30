use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVector;
use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};
use crate::graph::vertex::{VertexTypeKey, VertexTypeKeyRef};
use crate::{
    error::GraphComputingError,
    graph::{index::ElementCount, indexer::Indexer as VertexIndexer},
};

#[derive(Clone, Debug)]
pub(crate) struct VertexVector {
    vertex_type: VertexTypeKey,
    sparse_vector_bool: SparseVector<bool>,
    sparse_vector_i8: SparseVector<i8>,
    sparse_vector_i16: SparseVector<i16>,
    sparse_vector_i32: SparseVector<i32>,
    sparse_vector_i64: SparseVector<i64>,
    sparse_vector_u8: SparseVector<u8>,
    sparse_vector_u16: SparseVector<u16>,
    sparse_vector_u32: SparseVector<u32>,
    sparse_vector_u64: SparseVector<u64>,
    sparse_vector_f32: SparseVector<f32>,
    sparse_vector_f64: SparseVector<f64>,
    sparse_vector_isize: SparseVector<isize>,
    sparse_vector_usize: SparseVector<usize>,
}

impl VertexVector {
    pub(crate) fn new(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_vertex_capacity: &ElementCount,
        vertex_type: &VertexTypeKeyRef,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            vertex_type: vertex_type.to_owned(),
            sparse_vector_bool: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i8: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i16: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_i64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u8: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u16: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_u64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_f32: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_f64: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_isize: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
            sparse_vector_usize: SparseVector::new(graphblas_context, initial_vertex_capacity)?,
        })
    }
}

pub(crate) trait VertexVectorTrait<T: ValueType> {
    fn sparse_vector_ref(&self) -> &SparseVector<T>;
    fn sparse_vector_mut_ref(&mut self) -> &mut SparseVector<T>;
}

macro_rules! implement_vertex_vector_trait {
    ($typed_sparse_vector:ident, $value_type: ty) => {
        impl VertexVectorTrait<$value_type> for VertexVector {
            fn sparse_vector_ref(&self) -> &SparseVector<$value_type> {
                &self.$typed_sparse_vector
            }
            fn sparse_vector_mut_ref(&mut self) -> &mut SparseVector<$value_type> {
                &mut self.$typed_sparse_vector
            }
        }
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_vertex_vector_trait,
    sparse_vector
);
