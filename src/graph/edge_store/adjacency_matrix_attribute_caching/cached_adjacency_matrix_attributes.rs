use std::sync::Arc;

use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;
use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::GetGraphblasSparseMatrix, context::GetContext,
};

use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_bool;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_f32;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_f64;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_i16;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_i32;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_i64;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_i8;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_isize;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_u16;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_u32;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_u64;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_u8;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::transpose_adjacency_matrix_usize;
use crate::graph::graph::GraphblasContext;
use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix,
        value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier},
    },
};

#[derive(Clone, Debug)]
pub(crate) struct CachedAdjacencyMatrixAttributes {
    transpose: Option<WeightedAdjacencyMatrix>,
    select_entire_adjacency_matrix: SelectEntireMatrix,
}

impl CachedAdjacencyMatrixAttributes {
    pub(crate) fn new(context: Arc<GraphblasContext>) -> Self {
        CachedAdjacencyMatrixAttributes {
            transpose: None,
            select_entire_adjacency_matrix: SelectEntireMatrix::new(context),
        }
    }
}

pub(crate) trait InvalidateChachedAdjacencyMatrixAttributes {
    fn invalidate_all_attributes(&mut self) -> ();
}

impl InvalidateChachedAdjacencyMatrixAttributes for CachedAdjacencyMatrixAttributes {
    fn invalidate_all_attributes(&mut self) -> () {
        self.transpose = None
    }
}

pub(crate) trait GetAdjacencyMatrixTranspose {
    fn transpose_ref(
        &mut self,
        adjacency_matrix: &(impl GetValueTypeIdentifierRef + GetGraphblasSparseMatrix + GetContext),
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
}

impl GetAdjacencyMatrixTranspose for CachedAdjacencyMatrixAttributes {
    fn transpose_ref(
        &mut self,
        adjacency_matrix: &(impl GetValueTypeIdentifierRef + GetGraphblasSparseMatrix + GetContext),
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError> {
        self.compute_transpose(adjacency_matrix)?;
        if self.transpose.is_none() {
            self.compute_transpose(adjacency_matrix)?;
        }
        return Ok(self.transpose.as_ref().unwrap());
    }
}

impl CachedAdjacencyMatrixAttributes {
    fn compute_transpose(
        &mut self,
        adjacency_matrix: &(impl GetValueTypeIdentifierRef + GetGraphblasSparseMatrix + GetContext),
    ) -> Result<(), GraphComputingError> {
        let transpose = match adjacency_matrix.value_type_identifier_ref() {
            &ValueTypeIdentifier::Bool => transpose_adjacency_matrix_bool(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Int8 => transpose_adjacency_matrix_i8(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Int16 => transpose_adjacency_matrix_i16(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Int32 => transpose_adjacency_matrix_i32(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Int64 => transpose_adjacency_matrix_i64(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::UInt8 => transpose_adjacency_matrix_u8(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::UInt16 => transpose_adjacency_matrix_u16(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::UInt32 => transpose_adjacency_matrix_u32(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::UInt64 => transpose_adjacency_matrix_u64(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Float32 => transpose_adjacency_matrix_f32(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::Float64 => transpose_adjacency_matrix_f64(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::ISize => transpose_adjacency_matrix_isize(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
            &ValueTypeIdentifier::USize => transpose_adjacency_matrix_usize(
                adjacency_matrix,
                &self.select_entire_adjacency_matrix,
            )?,
        };
        self.transpose = Some(transpose);
        return Ok(());
    }
}
