use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::error::GraphComputingError;
use crate::graph::graph::GetVertexStore;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::{ToSparseVector, ToSparseVectorForValueType};
use crate::graph::{graph::Graph, value_type::ValueType};
use crate::operators::operator_traits::read::{GetSparseVertexVector, GetVertexVectorElementList};

impl<T> GetSparseVertexVector<T> for Graph
where
    T: ValueType + ToSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref(type_index)?
            .to_sparse_vector()?)
    }
}

impl<T> GetVertexVectorElementList<T> for Graph
where
    T: ValueType + ToSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_ref()
            .vertex_vector_ref(type_index)?
            .to_sparse_vector()?
            .element_list()?)
    }
}
