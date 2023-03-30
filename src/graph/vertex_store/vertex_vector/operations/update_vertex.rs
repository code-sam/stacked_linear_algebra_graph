use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::indexer::{Indexer, IndexerTrait};
use crate::graph::value_type::NativeDataType as GraphNativeDataType;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait, VertexKeyRef};
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};

pub(crate) trait UpdateVertex<T: ValueType> {
    fn update_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError>;
    fn update_vertex_value_unchecked(
        &mut self,
        index: Index,
        value: T,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl UpdateVertex<$value_type> for VertexStore {
            fn update_vertex_value(
                &mut self,
                index: Index,
                value: $value_type,
            ) -> Result<(), GraphComputingError> {
                self.indexer_ref().try_index_validity(&index)?;
                self.update_vertex_value_unchecked(index, value)
            }

            fn update_vertex_value_unchecked(
                &mut self,
                index: Index,
                value: $value_type,
            ) -> Result<(), GraphComputingError> {
                self.vertex_vector_mut_ref()
                    .set_element((index, value).into())?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_set_vertex_data);
