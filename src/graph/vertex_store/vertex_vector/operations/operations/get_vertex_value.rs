use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, GetSparseVectorElementValueUntyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::GetGraphblasSparseVector;

use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::{
    GetValueTypeIdentifierRef, IntoValueType, ValueType, ValueTypeIdentifier,
};

use crate::graph::vertex_store::operations::GetVertexVector;
use crate::graph::vertex_store::vertex_store::VertexStore;
use crate::graph::vertex_store::{
    AsSparseVectorForValueType, GetVertexElementIndexer, GetVertexTypeIndexer,
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

impl<T> GetVertexValue<T> for VertexStore
where
    T: ValueType + AsSparseVectorForValueType<T> + GetSparseVectorElementValueTyped<T> + Default,
    bool: IntoValueType<T>,
    i8: IntoValueType<T>,
    i16: IntoValueType<T>,
    i32: IntoValueType<T>,
    i64: IntoValueType<T>,
    u8: IntoValueType<T>,
    u16: IntoValueType<T>,
    u32: IntoValueType<T>,
    u64: IntoValueType<T>,
    f32: IntoValueType<T>,
    f64: IntoValueType<T>,
    isize: IntoValueType<T>,
    usize: IntoValueType<T>,
{
    fn public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn try_public_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.try_vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn public_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_public_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }

    fn private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn try_private_vertex_value(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.try_vertex_value_unchecked(vertex_type_index, vertex_index)
    }

    fn private_vertex_value_or_default(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_is_valid_private_index(vertex_type_index.index())?;
        self.element_indexer_ref()
            .try_index_validity(vertex_index.index())?;
        self.vertex_value_or_default_unchecked(vertex_type_index, vertex_index)
    }

    fn vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<Option<T>, GraphComputingError> {
        let sparse_vertex_vector = self.vertex_vector_ref_unchecked(vertex_type_index);

        match sparse_vertex_vector.value_type_identifier_ref() {
            &ValueTypeIdentifier::Bool => unsafe {
                get_vector_element_value::<bool, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Int8 => unsafe {
                get_vector_element_value::<i8, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Int16 => unsafe {
                get_vector_element_value::<i16, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Int32 => unsafe {
                get_vector_element_value::<i32, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Int64 => unsafe {
                get_vector_element_value::<i64, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::UInt8 => unsafe {
                get_vector_element_value::<u8, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::UInt16 => unsafe {
                get_vector_element_value::<u16, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::UInt32 => unsafe {
                get_vector_element_value::<u32, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::UInt64 => unsafe {
                get_vector_element_value::<u64, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Float32 => unsafe {
                get_vector_element_value::<f32, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::Float64 => unsafe {
                get_vector_element_value::<f64, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::ISize => unsafe {
                get_vector_element_value::<isize, T>(sparse_vertex_vector, vertex_index)
            },
            &ValueTypeIdentifier::USize => unsafe {
                get_vector_element_value::<usize, T>(sparse_vertex_vector, vertex_index)
            },
        }
    }

    fn try_vertex_value_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<T, GraphComputingError> {
        match self.vertex_value_unchecked(vertex_type_index, vertex_index)? {
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
        match self.vertex_value_unchecked(vertex_type_index, vertex_index)? {
            Some(vertex_value) => Ok(vertex_value),
            None => Ok(T::default()),
        }
    }
}

unsafe fn get_vector_element_value<V, T>(
    vector: &(impl GetGraphblasSparseVector + GetValueTypeIdentifierRef),
    vertex_index: &impl GetVertexIndexIndex,
) -> Result<Option<T>, GraphComputingError>
where
    V: ValueType + Default + GetSparseVectorElementValueUntyped<V> + IntoValueType<T>,
    T: ValueType,
{
    match unsafe { V::element_value(vector, vertex_index.index())? } {
        Some(edge_weight) => Ok(Some(edge_weight.into_value_type())),
        None => Ok(None),
    }
}
