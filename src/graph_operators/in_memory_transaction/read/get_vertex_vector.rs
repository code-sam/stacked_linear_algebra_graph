use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, VectorElementList as VertexVectorElementList,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::traits::vertex_type::GetVertexVector;
use crate::graph::vertex_store::{ToSparseVector, ToSparseVectorForValueType};
use crate::graph_operators::operator_traits::read::{
    GetSparseVertexVector, GetVertexVectorElementList,
};
use crate::transaction::in_memory::InMemoryGraphTransaction;

impl<'g, T> GetSparseVertexVector<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType + ToSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<SparseVector<T>, GraphComputingError> {
        Ok(self
            .vertex_store_transaction
            .vertex_vector_ref(type_index)?
            .to_sparse_vector()?)
    }
}

impl<'g, T> GetVertexVectorElementList<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType + ToSparseVectorForValueType<T>,
    SparseVector<T>: GetSparseVectorElementList<T>,
{
    fn sparse_vector_element_list(
        &self,
        type_index: &impl GetVertexTypeIndex,
    ) -> Result<VertexVectorElementList<T>, GraphComputingError> {
        Ok(self
            .vertex_store_transaction
            .vertex_vector_ref(type_index)?
            .to_sparse_vector()?
            .element_list()?)
    }
}
