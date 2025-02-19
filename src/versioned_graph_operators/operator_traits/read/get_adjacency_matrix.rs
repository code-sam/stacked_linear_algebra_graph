use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElementList as AdjacencyMatrixElementList, SparseMatrix,
};

use crate::graph::indexing::GetEdgeTypeIndex;
use crate::versioned_graph::indexing::GetVersionedEdgeTypeIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

pub trait GetSparseAdjacencyMatrixVersioned<T: ValueType> {
    fn sparse_adjacency_matrix(
        &self,
        edge_type_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub trait GetTransposedSparseAdjacencyMatrixVersioned<T: ValueType> {
    fn transposed_sparse_adjacency_matrix(
        &mut self,
        edge_type_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<SparseMatrix<T>, GraphComputingError>;
}

pub trait GetAdjacencyMatrixElementListVersioned<T: ValueType> {
    fn adjacency_matrix_element_list(
        &self,
        edge_type_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}

pub trait GetTransposedAdjacencyMatrixElementListVersioned<T: ValueType> {
    fn transposed_adjacency_matrix_element_list(
        &mut self,
        edge_type_index: &impl GetVersionedEdgeTypeIndex,
    ) -> Result<AdjacencyMatrixElementList<T>, GraphComputingError>;
}
