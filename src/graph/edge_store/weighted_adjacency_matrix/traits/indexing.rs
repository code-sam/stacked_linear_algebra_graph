use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    is_sparse_matrix_element, is_sparse_matrix_element_at_coordinate, try_is_sparse_matrix_element,
    try_is_sparse_matrix_element_at_coordinate, MatrixElementCoordinateIterator,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;
use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::GetSparseVectorElementList;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::graph::edge_store::weighted_adjacency_matrix::traits::select_edge_vertices::SelectEdgeVertices;
use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::indexing::{GetVertexIndexIndex, VertexIndex};
use crate::{
    error::GraphComputingError,
    graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix,
};

pub(crate) trait Indexing {
    fn is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError>;
    fn is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError>;
    fn try_is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
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

impl Indexing for WeightedAdjacencyMatrix {
    fn is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<bool, GraphComputingError> {
        Ok(is_sparse_matrix_element(self, tail.index(), head.index())?)
    }

    fn is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<bool, GraphComputingError> {
        Ok(is_sparse_matrix_element_at_coordinate(self, coordinate)?)
    }

    fn try_is_edge(
        &self,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        Ok(try_is_sparse_matrix_element(
            self,
            tail.index(),
            head.index(),
        )?)
    }

    fn try_is_edge_at_coordinate(
        &self,
        coordinate: &(impl GetCoordinateIndices + GetAdjacencyMatrixCoordinateIndices),
    ) -> Result<(), GraphComputingError> {
        Ok(try_is_sparse_matrix_element_at_coordinate(
            self, coordinate,
        )?)
    }

    fn adjacency_matrix_coordinates(
        &self,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError> {
        let matrix_element_index_iterator = MatrixElementCoordinateIterator::new(self)?;

        let coordinates = matrix_element_index_iterator
            .into_iter()
            .map(|coordinate| {
                AdjacencyMatrixCoordinate::new(
                    VertexIndex::new(coordinate.row_index()),
                    VertexIndex::new(coordinate.column_index()),
                )
            })
            .collect();

        // // TODO: use an iterator, or other method for better performance. Probably, this requires to re-implement a specialized iterator for WeightedAdjacencyMatrix
        // let matrix_element_list: MatrixElementList<T> = self.to_sparse_matrix()?.element_list()?;
        // let element_indices_vertices_with_outgoing_edges = matrix_element_list.row_indices_ref();
        // let element_indices_vertices_with_incoming_edges = matrix_element_list.column_indices_ref();

        // let mut coordinates: Vec<AdjacencyMatrixCoordinate> =
        //     Vec::with_capacity(matrix_element_list.length());
        // for element_index in 0..matrix_element_list.length() {
        //     let element_coordinate = AdjacencyMatrixCoordinate::new(
        //         VertexIndex::new(element_indices_vertices_with_outgoing_edges[element_index]),
        //         VertexIndex::new(element_indices_vertices_with_incoming_edges[element_index]),
        //     );
        //     coordinates.push(element_coordinate);
        // }
        Ok(coordinates)
    }

    fn indices_of_vertices_with_outgoing_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        Ok(
            // TODO: use element iterator for better performance
            SelectEdgeVertices::select_vertices_with_outgoing_edges(self)?
                .element_list()?
                .indices_ref()
                .into_par_iter()
                .map(|index| VertexIndex::new(index.to_owned()))
                .collect(),
        )
    }

    fn indices_of_vertices_with_incoming_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError> {
        Ok(
            // TODO: use element iterator for better performance
            SelectEdgeVertices::select_vertices_with_incoming_edges(self)?
                .element_list()?
                .indices_ref()
                .into_par_iter()
                .map(|index| VertexIndex::new(index.to_owned()))
                .collect(),
        )
    }

    ///
    fn indices_of_connected_vertices(&self) -> Result<Vec<VertexIndex>, GraphComputingError> {
        // TODO: use element iterator for better performance
        Ok(SelectEdgeVertices::select_connected_vertices(self)?
            .element_list()?
            .indices_ref()
            .into_par_iter()
            .map(|index| VertexIndex::new(index.to_owned()))
            .collect())
    }
}
