use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetMatrixElementTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;

use crate::error::GraphComputingError;

use crate::graph::indexer::IndexerTrait;

use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::value_type::SparseVertexMatrixForValueType;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex::GetVertexValue;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndex;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndexTrait;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKeyTrait;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKey;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKeyTrait;
// use crate::graph::vertex_store::type_operations::get_vertex_matrix::GetVertexMatrix;
use crate::graph::vertex_store::vertex_matrix::SparseVertexMatrix;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::IsElementInVertexMatrix;
use crate::graph::vertex_store::SetVertexMatrixValue;

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

impl<T: ValueType + Copy + SparseVertexMatrixForValueType<T> + SetMatrixElementTyped<T>>
    UpdateVertex<T> for VertexStore
{
    fn update_key_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByKey<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_index = *self
            .element_indexer_ref()
            .try_index_for_key(vertex.key_ref())?;
        let vertex_type_index = self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex.type_key_ref())?
            .to_owned();
        IsElementInVertexMatrix::<T>::try_is_vertex_element(
            self.vertex_matrix_ref(),
            &vertex_type_index,
            &vertex_index,
        )?;
        Ok(self.vertex_matrix_mut_ref().set_vertex_value(
            &vertex_type_index,
            &vertex_index,
            *vertex.value_ref(),
        )?)
    }

    fn update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: &VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_index = self
            .element_indexer_ref()
            .try_index_for_key(vertex.key_ref())?
            .to_owned();
        Ok(self.vertex_matrix_mut_ref().set_vertex_value(
            vertex.type_index_ref(),
            &vertex_index,
            *vertex.value_ref(),
        )?)
    }

    fn update_index_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_index_validity(vertex.index_ref())?;
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex.type_index_ref())?;
        Ok(self.vertex_matrix_mut_ref().set_vertex_value(
            vertex.type_index_ref(),
            vertex.index_ref(),
            *vertex.value_ref(),
        )?)
    }

    fn update_index_defined_vertex_unchecked(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        Ok(self.vertex_matrix_mut_ref().set_vertex_value(
            vertex.type_index_ref(),
            vertex.index_ref(),
            *vertex.value_ref(),
        )?)
    }
}
