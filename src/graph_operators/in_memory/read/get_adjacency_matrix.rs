use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementList;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElementList as AdjacencyMatrixElementList, SparseMatrix,
};

use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::graph::GetEdgeStore;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::weighted_adjacency_matrix::{ToSparseMatrix, ToSparseMatrixForValueType};
use crate::graph_operators::operator_traits::indexing::CheckIndex;
use crate::graph_operators::operator_traits::read::{GetAdjacencyMatrixElementList, GetSparseAdjacencyMatrix, GetTransposedAdjacencyMatrixElementList, GetTransposedSparseAdjacencyMatrix};
use crate::graph::value_type::ValueType;
use crate::graph::graph::Graph;
use crate::graph::edge_store::traits::traits::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::error::GraphComputingError;

impl<T> GetSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + ToSparseMatrixForValueType<T>,
{
    fn sparse_adjacency_matrix(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref(edge_type_index)?
            .to_sparse_matrix()?)
    }
}

impl<T> GetTransposedSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + ToSparseMatrixForValueType<T>,
{
    fn transposed_sparse_adjacency_matrix(
        &mut self,
        edge_edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        self.try_edge_type_index_validity(edge_edge_type_index)?;
        Ok(self
            .edge_store_mut_ref()
            .transposed_adjacency_matrix_ref_unchecked(edge_edge_type_index)
            .to_sparse_matrix()?)
    }
}

impl<T> GetAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + ToSparseMatrixForValueType<T>,
{
    fn adjacency_matrix_element_list(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }
}

impl<T> GetTransposedAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + ToSparseMatrixForValueType<T>,
{
    fn transposed_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .transposed_sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }
}
