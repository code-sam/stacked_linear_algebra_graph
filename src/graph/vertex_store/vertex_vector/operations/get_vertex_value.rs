use std::fmt::Debug;

use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetVectorElementValue, GetVectorElementValueTyped,
};

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::{
    GetVertexIndexIndex, GetVertexTypeIndex, VertexIndex, VertexTypeIndex,
};
use crate::graph::value_type::ValueType;

use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::VertexStore;
use crate::graph::vertex_store::{
    GetVertexElementIndexer, GetVertexTypeIndexer, IntoSparseVector, IntoSparseVectorForValueType,
};

pub(crate) trait GetVertexValue<T: ValueType> {
    fn public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn public_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError>;

    fn try_vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError>;
}

impl<T: ValueType + IntoSparseVectorForValueType<T> + GetVectorElementValueTyped<T> + Default>
    GetVertexValue<T> for VertexStore
{
    fn public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn try_public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.try_vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn public_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }

    fn private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.try_vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index_ref())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index_ref())?;
        self.vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        // Ok(T::get_element_value(self
        //     .vertex_vector_ref_by_index_unchecked(vertex_type_index), vertex_index)?)
        Ok(self
            .vertex_vector_ref_unchecked(vertex_type_index)
            .sparse_vector()?
            .element_value(vertex_index.index_ref())?)
    }

    fn try_vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        match self
        .vertex_vector_ref_unchecked(vertex_type_index)
        .sparse_vector()?
        .element_value(vertex_index.index_ref())? {
            Some(weight) => Ok(weight),
            None => Err(LogicError::new(
                LogicErrorType::EdgeMustExist,
                format!("No vertex value exists at vertex index: {:?}, for vertex type index: {:?}, and value type: {}",
                    vertex_index, vertex_type_index, std::any::type_name::<T>()),
                None,
            )
            .into()),
        }
    }

    fn vertex_value_or_default_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        Ok(self
            .vertex_vector_ref_unchecked(vertex_type_index)
            .sparse_vector()?
            .element_value_or_default(vertex_index.index_ref())?)
    }
}
