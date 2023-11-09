use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::try_is_element;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexer::IndexerTrait;

use crate::graph::value_type::ValueType;
use crate::graph::vertex::vertex::GetVertexValue;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndex;
use crate::graph::vertex::vertex_defined_by_index::VertexDefinedByIndexTrait;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKey;
use crate::graph::vertex::vertex_defined_by_key::VertexDefinedByKeyTrait;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKey;
use crate::graph::vertex::vertex_defined_by_vertex_type_index_and_vertex_key::VertexDefinedByTypeIndexAndVertexKeyTrait;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::VertexVector;

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

impl<T: ValueType + Copy + SetVectorElementTyped<T>> UpdateVertex<T> for VertexStore {
    fn update_key_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByKey<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_index = *self
            .element_indexer_ref()
            .try_index_for_key(vertex.key_ref())?;
        let vertex_vector = self.vertex_vector_mut_ref_by_key(vertex.type_key_ref())?;
        try_is_element(vertex_vector, vertex_index)?;
        T::set_element(vertex_vector, (vertex_index, *vertex.value_ref()).into())?;
        Ok(())
    }

    fn update_vertex_defined_by_type_index_and_vertex_key(
        &mut self,
        vertex: &VertexDefinedByTypeIndexAndVertexKey<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_index = *self
            .element_indexer_ref()
            .try_index_for_key(vertex.key_ref())?;
        let vertex_vector = self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
        try_is_element(vertex_vector, vertex_index)?;
        T::set_element(vertex_vector, (vertex_index, *vertex.value_ref()).into())?;
        Ok(())
    }

    fn update_index_defined_vertex(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        self.element_indexer_ref()
            .try_index_validity(vertex.index_ref())?;
        let vertex_vector: &mut VertexVector =
            self.vertex_vector_mut_ref_by_index(vertex.type_index_ref())?;
        try_is_element(vertex_vector, *vertex.index_ref())?;

        T::set_element(
            vertex_vector,
            (*vertex.index_ref(), *vertex.value_ref()).into(),
        )?;
        Ok(())
    }

    fn update_index_defined_vertex_unchecked(
        &mut self,
        vertex: &VertexDefinedByIndex<T>,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_by_index_unchecked(vertex.type_index_ref());
        try_is_element(vertex_vector, *vertex.index_ref())?;
        T::set_element(
            vertex_vector,
            (*vertex.index_ref(), *vertex.value_ref()).into(),
        )?;
        Ok(())
    }
}
