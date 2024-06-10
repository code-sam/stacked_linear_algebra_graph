use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::graph::graph::GetVertexStore;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::{IntoSparseVector, IntoSparseVectorForValueType};
use crate::operators::operators::read::{GetPrivateSparseVertexVector, GetSparseVertexVector};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::Graph, value_type::ValueType,
        vertex_store::operations::get_vertex_vector::GetVertexVector,
    },
};

impl<T> GetSparseVertexVector<T> for Graph
where
    T: ValueType + IntoSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .public_vertex_vector_ref(type_index)?
            .sparse_vector()?)
    }
}

impl<T> GetPrivateSparseVertexVector<T> for Graph
where
    T: ValueType + IntoSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn private_sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .private_vertex_vector_ref(type_index)?
            .sparse_vector()?)
    }

    fn sparse_vector_unchecked(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref_unchecked(type_index)
            .sparse_vector()?)
    }
}

pub trait GetVertexVectorElementList<T: ValueType> {
    fn sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateVertexVectorElementList<T: ValueType> {
    fn private_sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
    fn sparse_vector_element_list_unchecked(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
}

impl<T> GetVertexVectorElementList<T> for Graph
where
    T: ValueType + IntoSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .public_vertex_vector_ref(type_index)?
            .sparse_vector()?
            .get_element_list()?)
    }
}

impl<T> GetPrivateVertexVectorElementList<T> for Graph
where
    T: ValueType + IntoSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn private_sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .private_vertex_vector_ref(type_index)?
            .sparse_vector()?
            .get_element_list()?)
    }

    fn sparse_vector_element_list_unchecked(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref_unchecked(type_index)
            .sparse_vector()?
            .get_element_list()?)
    }
}
