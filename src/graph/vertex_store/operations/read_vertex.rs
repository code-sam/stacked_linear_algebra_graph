use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetMatrixElementValue;

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
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl ReadVertex<$value_type> for VertexStore<$value_type> {
            fn vertex_value_by_key(
                &self,
                key: &VertexKey,
            ) -> Result<$value_type, GraphComputingError> {
                let index = self.indexer_ref().try_index_for_key(key)?.clone();
                Ok(self
                    .vertex_matrix_ref()
                    .get_element_value_or_default(&(index, index).into())?)
            }

            fn vertex_value_by_index(
                &self,
                index: VertexIndex,
            ) -> Result<$value_type, GraphComputingError> {
                self.indexer_ref().try_index_validity(&index)?;
                Ok(self
                    .vertex_matrix_ref()
                    .get_element_value_or_default(&(index, index).into())?)
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_set_vertex_data);
