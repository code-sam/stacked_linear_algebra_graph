use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    SetMatrixElement, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;

use crate::error::GraphComputingError;

use crate::graph::indexer::IndexerTrait;

use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::VertexDefinedByKeyTrait;
use crate::graph::vertex::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex::{
    VertexDefinedByIndex, VertexDefinedByIndexTrait, VertexDefinedByKey,
    VertexDefinedByTypeIndexAndVertexKey,
};
use crate::graph::vertex_store::type_operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::vertex_vector::SparseVertexVector;

pub(crate) trait UpdateVertex<T: ValueType> {
    fn update_key_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByKey<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: &VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_index_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;

    fn update_index_defined_vertex_unchecked(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl UpdateVertex<$value_type> for VertexStore {
            fn update_key_defined_vertex(
                &mut self,
                vertex: &VertexDefinedByKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let vertex_index = *self
                    .element_indexer_ref()
                    .try_index_for_key(vertex.key_ref())?;
                let vertex_vector = self.vertex_vector_mut_ref_by_key(vertex.type_key_ref())?;
                SparseVertexVector::<$value_type>::sparse_vector_ref(vertex_vector)
                    .try_is_element(vertex_index)?;
                vertex_vector
                    .sparse_vector_mut_ref()
                    .set_element((vertex_index, *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_vertex_defined_by_type_index_and_vertex_key(
                &mut self,
                vertex: &VertexDefinedByTypeIndexAndVertexKey<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let vertex_index = *self
                    .element_indexer_ref()
                    .try_index_for_key(vertex.key_ref())?;
                let vertex_vector = self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
                SparseVertexVector::<$value_type>::sparse_vector_ref(vertex_vector)
                    .try_is_element(vertex_index)?;
                vertex_vector
                    .sparse_vector_mut_ref()
                    .set_element((vertex_index, *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_index_defined_vertex(
                &mut self,
                vertex: &VertexDefinedByIndex<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.element_indexer_ref()
                    .try_index_validity(vertex.index_ref())?;
                let vertex_vector = self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
                SparseVertexVector::<$value_type>::sparse_vector_ref(vertex_vector)
                    .try_is_element(*vertex.index_ref())?;
                vertex_vector
                    .sparse_vector_mut_ref()
                    .set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
                Ok(())
            }

            fn update_index_defined_vertex_unchecked(
                &mut self,
                vertex: &VertexDefinedByIndex<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let vertex_vector =
                    self.vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref());
                SparseVertexVector::<$value_type>::sparse_vector_ref(vertex_vector)
                    .try_is_element(*vertex.index_ref())?;
                vertex_vector
                    .sparse_vector_mut_ref()
                    .set_element((*vertex.index_ref(), *vertex.value_ref()).into())?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_set_vertex_data);
