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
    GetVertexElementIndexer, GetVertexTypeIndexer, ToSparseVectorForValueType,
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
