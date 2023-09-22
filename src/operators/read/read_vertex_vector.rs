use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::graph::vertex_store::{SparseVertexVector, VertexVector};
use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait, VertexTypeIndex},
        value_type::{SparseVertexVectorForValueType, ValueType},
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::type_operations::get_vertex_vector::GetVertexVector,
    },
};

pub trait ReadVertexVectorElementList<T: ValueType> {
    fn with_index(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
    fn with_index_unchecked(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
    fn with_key(
        &self,
        type_key: &VertexTypeKeyRef,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError>;
}

impl<T> ReadVertexVectorElementList<T> for Graph
where
    T: ValueType + SparseVertexVectorForValueType<T>,
    VertexVector: SparseVertexVector<T>,
    SparseVector<T>: GetVectorElementList<T>,
{
    fn with_index(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref_by_index(type_index)?
            .sparse_vector_ref()
            .get_element_list()?)
    }

    fn with_index_unchecked(
        &self,
        type_index: &VertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref_by_index_unchecked(type_index)
            .sparse_vector_ref()
            .get_element_list()?)
    }

    fn with_key(
        &self,
        type_key: &VertexTypeKeyRef,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref_by_key(type_key)?
            .sparse_vector_ref()
            .get_element_list()?)
    }
}
