use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    Coordinate, GetCoordinateIndices,
};

use crate::graph::graph::VertexIndex;

pub type AdjacencyMatrixCoordinate = Coordinate;

pub trait GetAdjacencyMatrixCoordinateIndices {
    fn tail_ref(&self) -> &VertexIndex;
    fn head_ref(&self) -> &VertexIndex;
}

impl GetAdjacencyMatrixCoordinateIndices for AdjacencyMatrixCoordinate {
    fn tail_ref(&self) -> &VertexIndex {
        self.row_index_ref()
    }

    fn head_ref(&self) -> &VertexIndex {
        self.column_index_ref()
    }
}
