use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetMatrixElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetVectorElementValue;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;

use crate::error::GraphComputingError;
use crate::graph::graph::VertexIndex;
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::VertexKey;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};

pub(crate) trait ReadVertex<T: ValueType> {
    fn vertex_value_by_key(&self, key: &VertexKey) -> Result<T, GraphComputingError>;
    fn vertex_value_by_index(&self, vertex_index: VertexIndex) -> Result<T, GraphComputingError>;
    fn vertex_value_by_index_unchecked(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl ReadVertex<$value_type> for VertexStore {
            fn vertex_value_by_key(
                &self,
                key: &VertexKey,
            ) -> Result<$value_type, GraphComputingError> {
                let index = self.indexer_ref().try_index_for_key(key)?.clone();
                Ok(self
                    .vertex_vector_ref()
                    .get_element_value_or_default(&index)?)
            }

            fn vertex_value_by_index(
                &self,
                index: VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                self.indexer_ref().try_index_validity(&index)?;
                self.vertex_value_by_index_unchecked(index)
            }

            fn vertex_value_by_index_unchecked(
                &self,
                index: VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                Ok(self
                    .vertex_vector_ref()
                    .get_element_value_or_default(&index)?)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_set_vertex_data);
