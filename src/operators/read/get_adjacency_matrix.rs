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
        edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix,
        graph::{EdgeTypeIndex, Graph, GraphTrait},
        value_type::ValueType,
    },
};

pub trait GetSparseAdjacencyMatrix<T: ValueType> {
    fn sparse_adjacency_matrix(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn sparse_adjacency_matrix_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

impl<T> GetSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn sparse_adjacency_matrix(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_adjacency_matrix_ref(type_index)?
            .sparse_matrix()?)
    }

    fn sparse_adjacency_matrix_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_unchecked(type_index)
            .sparse_matrix()?)
    }
}

pub trait GetAdjacencyMatrixElementList<T: ValueType> {
    fn adjacency_matrix_element_list(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn adjacency_matrix_element_list_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

impl<T> GetAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn adjacency_matrix_element_list(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_adjacency_matrix_ref(type_index)?
            .sparse_matrix()?
            .element_list()?)
    }

    fn adjacency_matrix_element_list_unchecked(
        &self,
        type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_unchecked(type_index)
            .sparse_matrix()?
            .element_list()?)
    }
}
