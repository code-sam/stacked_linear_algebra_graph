use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::graph::graph::GetVertexStore;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::{IntoSparseVector, IntoSparseVectorForValueType};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::Graph, value_type::ValueType,
        vertex_store::operations::GetVertexVector,
    },
};

pub trait GetSparseVertexVector<T: ValueType> {
    fn sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateSparseVertexVector<T: ValueType> {
    fn private_sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError>;
    fn sparse_vector_unchecked(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError>;
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
