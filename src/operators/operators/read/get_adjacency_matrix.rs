use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementList;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElementList as AdjacencyMatrixElementList, SparseMatrix,
};

use crate::graph::edge_store::operations::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    IntoSparseMatrix, IntoSparseMatrixForValueType,
};
use crate::graph::graph::GetEdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::indexing::{CheckIndex, CheckPrivateIndex};
use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix, graph::Graph,
        value_type::ValueType,
    },
};

pub trait GetSparseAdjacencyMatrix<T: ValueType> {
    fn sparse_adjacency_matrix(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateSparseAdjacencyMatrix<T: ValueType> {
    fn private_sparse_adjacency_matrix(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn sparse_adjacency_matrix_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub trait GetTransposedSparseAdjacencyMatrix<T: ValueType> {
    fn transposed_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait GetTransposedPrivateSparseAdjacencyMatrix<T: ValueType> {
    fn transposed_private_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn transposed_sparse_adjacency_matrix_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub trait GetAdjacencyMatrixElementList<T: ValueType> {
    fn adjacency_matrix_element_list(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateAdjacencyMatrixElementList<T: ValueType> {
    fn private_adjacency_matrix_element_list(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn adjacency_matrix_element_list_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub trait GetTransposedAdjacencyMatrixElementList<T: ValueType> {
    fn transposed_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub(crate) trait GetTransposedPrivateAdjacencyMatrixElementList<T: ValueType> {
    fn transposed_private_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn transposed_adjacency_matrix_element_list_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}