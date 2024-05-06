use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementList;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElementList as AdjacencyMatrixElementList, SparseMatrix,
};

use crate::graph::edge_store::operations::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    IntoSparseMatrix, IntoSparseMatrixForValueType,
};
use crate::graph::graph::GetEdgeStore;
use crate::graph::indexing::EdgeTypeIndex;
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
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateSparseAdjacencyMatrix<T: ValueType> {
    fn private_sparse_adjacency_matrix(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn sparse_adjacency_matrix_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

impl<T> GetSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn sparse_adjacency_matrix(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_public_adjacency_matrix_ref(edge_type_index)?
            .sparse_matrix()?)
    }
}

impl<T> GetPrivateSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn private_sparse_adjacency_matrix(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .try_private_adjacency_matrix_ref(edge_type_index)?
            .sparse_matrix()?)
    }

    fn sparse_adjacency_matrix_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_ref()
            .adjacency_matrix_ref_unchecked(edge_type_index)
            .sparse_matrix()?)
    }
}

pub trait GetTransposedSparseAdjacencyMatrix<T: ValueType> {
    fn transposed_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub(crate) trait GetTransposedPrivateSparseAdjacencyMatrix<T: ValueType> {
    fn transposed_private_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
    fn transposed_sparse_adjacency_matrix_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

impl<T> GetTransposedSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn transposed_sparse_adjacency_matrix(
        &mut self,
        edge_edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        self.try_edge_type_index_validity(edge_edge_type_index)?;
        Ok(self
            .edge_store_mut_ref()
            .transposed_adjacency_matrix_ref_unchecked(edge_edge_type_index)
            .sparse_matrix()?)
    }
}

impl<T> GetTransposedPrivateSparseAdjacencyMatrix<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn transposed_private_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        self.try_is_valid_private_edge_type_index(edge_type_index)?;
        Ok(self
            .edge_store_mut_ref()
            .transposed_adjacency_matrix_ref_unchecked(edge_type_index)
            .sparse_matrix()?)
    }

    fn transposed_sparse_adjacency_matrix_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError> {
        Ok(self
            .edge_store_mut_ref()
            .transposed_adjacency_matrix_ref_unchecked(edge_type_index)
            .sparse_matrix()?)
    }
}

pub trait GetAdjacencyMatrixElementList<T: ValueType> {
    fn adjacency_matrix_element_list(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub(crate) trait GetPrivateAdjacencyMatrixElementList<T: ValueType> {
    fn private_adjacency_matrix_element_list(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn adjacency_matrix_element_list_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

impl<T> GetAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn adjacency_matrix_element_list(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }
}

impl<T> GetPrivateAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn private_adjacency_matrix_element_list(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .private_sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }

    fn adjacency_matrix_element_list_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .sparse_adjacency_matrix_unchecked(edge_type_index)?
            .element_list()?)
    }
}

pub trait GetTransposedAdjacencyMatrixElementList<T: ValueType> {
    fn transposed_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub(crate) trait GetTransposedPrivateAdjacencyMatrixElementList<T: ValueType> {
    fn transposed_private_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
    fn transposed_adjacency_matrix_element_list_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

impl<T> GetTransposedAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn transposed_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .transposed_sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }
}

impl<T> GetTransposedPrivateAdjacencyMatrixElementList<T> for Graph
where
    SparseMatrix<T>: GetSparseMatrixElementList<T>,
    T: ValueType + IntoSparseMatrixForValueType<T>,
{
    fn transposed_private_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .transposed_private_sparse_adjacency_matrix(edge_type_index)?
            .element_list()?)
    }

    fn transposed_adjacency_matrix_element_list_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError> {
        Ok(self
            .transposed_sparse_adjacency_matrix_unchecked(edge_type_index)?
            .element_list()?)
    }
}
