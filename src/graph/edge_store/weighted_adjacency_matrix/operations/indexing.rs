use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    is_sparse_matrix_element, try_is_sparse_matrix_element, GetSparseMatrixElementList,
    GetSparseMatrixElementListTyped, GetSparseMatrixElementValueTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElementList;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetVectorElementList;
use graphblas_sparse_linear_algebra::operators::monoid::AnyMonoidTyped;

use crate::graph::edge::AdjacencyMatrixCoordinate;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::select_edge_vertices::SelectEdgeVertices;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    IntoSparseMatrix, IntoSparseMatrixForValueType,
};
use crate::graph::value_type::ValueType;
use crate::{
    error::GraphComputingError,
    graph::{edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix, graph::VertexIndex},
};

pub(crate) trait Indexing<T: ValueType> {
    fn is_edge(&self, coordinate: &AdjacencyMatrixCoordinate) -> Result<bool, GraphComputingError>;
    fn try_is_edge(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;

    fn adjacency_matrix_coordinates(
        &self,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError>;
    fn indices_of_vertices_with_outgoing_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;
    fn indices_of_vertices_with_incoming_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;

    fn indices_of_connected_vertices(&self) -> Result<Vec<VertexIndex>, GraphComputingError>;
}

impl<
        T: ValueType
            + IntoSparseMatrixForValueType<T>
            + GetSparseMatrixElementListTyped<T>
            + AnyMonoidTyped<T>
            + Clone
            + Copy,
    > Indexing<T> for WeightedAdjacencyMatrix
{
    fn is_edge(&self, coordinate: &AdjacencyMatrixCoordinate) -> Result<bool, GraphComputingError> {
        Ok(is_sparse_matrix_element(self, *coordinate)?)
    }

    fn try_is_edge(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError> {
        Ok(try_is_sparse_matrix_element(self, *coordinate)?)
    }

    fn adjacency_matrix_coordinates(
        &self,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError> {
        let matrix_element_list: MatrixElementList<T> = self.sparse_matrix()?.get_element_list()?;
        let element_indices_vertices_with_outgoing_edges = matrix_element_list.row_indices_ref();
        let element_indices_vertices_with_incoming_edges = matrix_element_list.column_indices_ref();

        let mut coordinates: Vec<AdjacencyMatrixCoordinate> =
            Vec::with_capacity(matrix_element_list.length());
        for element_index in 0..matrix_element_list.length() {
            let element_coordinate = AdjacencyMatrixCoordinate::new(
                element_indices_vertices_with_outgoing_edges[element_index],
                element_indices_vertices_with_incoming_edges[element_index],
            );
            coordinates.push(element_coordinate);
        }
        Ok(coordinates)
    }

    fn indices_of_vertices_with_outgoing_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        Ok(
            SelectEdgeVertices::<T>::select_vertices_with_outgoing_edges(self)?
                .get_element_list()?
                .indices_ref()
                .to_vec(),
        )
    }

    fn indices_of_vertices_with_incoming_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        Ok(
            SelectEdgeVertices::<T>::select_vertices_with_incoming_edges(self)?
                .get_element_list()?
                .indices_ref()
                .to_vec(),
        )
    }

    ///
    fn indices_of_connected_vertices(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        Ok(SelectEdgeVertices::<T>::select_connected_vertices(self)?
            .get_element_list()?
            .indices_ref()
            .to_vec())
    }
}
