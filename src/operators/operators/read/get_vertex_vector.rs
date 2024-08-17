use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::graph::indexing::GetVertexTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

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
