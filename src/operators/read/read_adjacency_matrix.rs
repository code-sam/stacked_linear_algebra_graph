use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementList;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElementList as AdjacencyMatrixElementList, SparseMatrix,
};

use crate::graph::edge_store::weighted_adjacency_matrix::{
    IntoSparseMatrix, IntoSparseMatrixForValueType,
};
use crate::{
    error::GraphComputingError,
    graph::{
        edge::EdgeTypeKeyRef,
        edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix,
        graph::{EdgeTypeIndex, Graph, GraphTrait},
        value_type::ValueType,
    },
};

pub trait GetSparseAdjacencyMatrix<T: ValueType> {
    fn with_index(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn with_index_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn with_key(&self, type_key: &EdgeTypeKeyRef) -> Result<SparseMatrix<T>, GraphComputingError>;
}

impl<T> GetSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn with_index(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_adjacency_matrix_ref_for_index(type_index)?
            .sparse_matrix()?)
    }

    fn with_index_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_for_index_unchecked(type_index)
            .sparse_matrix()?)
    }

    fn with_key(&self, type_key: &EdgeTypeKeyRef) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_for_key(type_key)?
            .sparse_matrix()?)
    }
}

pub trait ReadAdjacencyMatrixElementList<T: ValueType> {
    fn with_index(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn with_index_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn with_key(
        &self,
        type_key: &EdgeTypeKeyRef,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

impl<T> ReadAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn with_index(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_adjacency_matrix_ref_for_index(type_index)?
            .sparse_matrix()?
            .get_element_list()?)
    }

    fn with_index_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_for_index_unchecked(type_index)
            .sparse_matrix()?
            .get_element_list()?)
    }

    fn with_key(
        &self,
        type_key: &EdgeTypeKeyRef,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_for_key(type_key)?
            .sparse_matrix()?
            .get_element_list()?)
    }
}
