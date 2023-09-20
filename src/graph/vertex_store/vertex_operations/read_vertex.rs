use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetMatrixElementValueTyped;

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::graph::VertexIndex;
use crate::graph::graph::VertexTypeIndex;
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, SparseVertexMatrixForValueType,
};

use crate::graph::vertex::vertex::VertexKeyRef;
use crate::graph::vertex::vertex::VertexTypeKeyRef;
// use crate::graph::vertex_store::type_operations::get_vertex_matrix::GetVertexMatrix;
use crate::graph::vertex_store::vertex_matrix::SparseVertexMatrix;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::ReadVertexValueInVertexMatrix;

pub(crate) trait ReadVertex<T: ValueType> {
    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError>;

    // fn vertex_value_by_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<
        T: ValueType + SparseVertexMatrixForValueType<T> + GetMatrixElementValueTyped<T> + Default,
    > ReadVertex<T> for VertexStore
{
    fn vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<Option<T>, GraphComputingError> {
        let type_index = self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)?;
        let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
        self.vertex_value_by_index(type_index, vertex_index)
    }

    fn try_vertex_value_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError> {
        let type_index = self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)?;
        let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
        self.try_vertex_value_by_index(type_index, vertex_index)
    }

    fn vertex_value_or_default_by_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_key: &VertexKeyRef,
    ) -> Result<T, GraphComputingError> {
        let type_index = self
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)?;
        let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
        self.vertex_value_or_default_by_index(type_index, vertex_index)
    }

    // fn vertex_value_by_type_index_and_vertex_key(
    //     &self,
    //     vertex_type_index: &VertexTypeIndex,
    //     vertex_key: &VertexKeyRef,
    // ) -> Result<$value_type, GraphComputingError> {
    //     self.vertex_type_indexer_ref()
    //         .try_index_validity(vertex_type_index);
    //     let vertex_index = self.element_indexer_ref().try_index_for_key(vertex_key)?;
    //     self.vertex_value_by_index_unchecked(vertex_type_index, vertex_index)
    // }

    fn vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        Ok(self
            .vertex_matrix_ref()
            .get_vertex_value(vertex_type_index, vertex_index)?)
    }

    fn try_vertex_value_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        match self.vertex_matrix_ref().get_vertex_value(vertex_type_index, vertex_index)? {
                        Some(value) => Ok(value),
                        None => Err(LogicError::new(
                            LogicErrorType::EdgeMustExist,
                            format!("No vertex value exists at vertex index: {:?}, for vertex type index: {:?}, and value type: {}",
                                vertex_index, vertex_type_index, std::any::type_name::<T>()),
                            None,
                        )
                        .into()),
                    }
    }

    fn vertex_value_or_default_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index)?;
        Ok(self
            .vertex_matrix_ref()
            .get_vertex_value_or_default(vertex_type_index, vertex_index)?)
    }

    fn vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index)?;
        self.vertex_value_by_index_unchecked(vertex_type_index, vertex_index)
    }

    fn try_vertex_value_by_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index)?;
        self.vertex_matrix_ref()
            .try_vertex_value(vertex_type_index, vertex_index)
    }

    fn vertex_value_or_default_by_index_unchecked(
        &self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<T, GraphComputingError> {
        Ok(self
            .vertex_matrix_ref()
            .get_vertex_value_or_default(vertex_type_index, vertex_index)?)
    }
}
