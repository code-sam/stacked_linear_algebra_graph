use crate::{
    error::{GraphComputingError, LogicError},
    graph::{
        graph::{VertexIndex, VertexTypeIndex},
        indexer::IndexerTrait,
        value_type::{
            implement_macro_for_all_native_value_types, SparseVertexMatrixForValueType, ValueType,
        },
        vertex::vertex::{VertexKeyRef, VertexTypeKeyRef},
        vertex_store::{
            type_operations::delete_vertex::DeleteVertexForAllValueTypes,
            DeleteVertexValueInVertexMatrix, SparseVertexMatrix, VertexMatrixStore,
            VertexMatrixTrait, VertexStore, VertexStoreTrait,
        },
    },
};

pub(crate) trait DeleteVertexElement<T: ValueType> {
    fn delete_vertex_element_by_key(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn delete_vertex_element_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait DeleteVertexForAllTypes {
    fn delete_vertex_for_all_vertex_types_and_value_types_by_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn delete_vertex_for_all_vertex_types_and_value_types_by_index(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType + SparseVertexMatrixForValueType<T>> DeleteVertexElement<T> for VertexStore {
    fn delete_vertex_element_by_key(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError> {
        let vertex_type_index = *self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)?;
        let vertex_index = *self.element_indexer_ref().try_index_for_key(vertex_key)?;
        Ok(DeleteVertexValueInVertexMatrix::<T>::delete_vertex_value(
            self.vertex_matrix_mut_ref(),
            &vertex_type_index,
            &vertex_index,
        )?)
    }

    fn delete_vertex_element_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(DeleteVertexValueInVertexMatrix::<T>::delete_vertex_value(
            self.vertex_matrix_mut_ref(),
            vertex_type_index,
            vertex_index,
        )?)
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_vertex_types_and_value_types_by_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError> {
        let vertex_element_index = *self.element_indexer_ref().try_index_for_key(vertex_key)?;
        self.delete_vertex_for_all_vertex_types_and_value_types_by_index(&vertex_element_index)
    }

    fn delete_vertex_for_all_vertex_types_and_value_types_by_index(
        &mut self,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_matrix_mut_ref()
            .delete_vertex_for_all_vertex_types_and_value_types(vertex_element_index)?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(*vertex_element_index)
    }
}
