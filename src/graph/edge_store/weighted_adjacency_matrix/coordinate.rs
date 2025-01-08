use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::graph::indexing::{GetIndex, VertexIndex};

pub struct AdjacencyMatrixCoordinate {
    tail: VertexIndex,
    head: VertexIndex,
}

impl AdjacencyMatrixCoordinate {
    pub fn new(tail: VertexIndex, head: VertexIndex) -> Self {
        Self { tail, head }
    }
}

pub trait GetAdjacencyMatrixCoordinateIndices {
    fn tail(&self) -> VertexIndex;
    fn tail_ref(&self) -> &VertexIndex;

    fn head(&self) -> VertexIndex;
    fn head_ref(&self) -> &VertexIndex;
}

impl GetAdjacencyMatrixCoordinateIndices for AdjacencyMatrixCoordinate {
    fn tail(&self) -> VertexIndex {
        self.tail.to_owned()
    }
    fn tail_ref(&self) -> &VertexIndex {
        &self.tail
    }

    fn head(&self) -> VertexIndex {
        self.head.to_owned()
    }
    fn head_ref(&self) -> &VertexIndex {
        &self.head
    }
}

impl GetCoordinateIndices for AdjacencyMatrixCoordinate {
    fn row_index(&self) -> graphblas_sparse_linear_algebra::index::ElementIndex {
        self.tail.index()
    }

    fn row_index_ref(&self) -> &graphblas_sparse_linear_algebra::index::ElementIndex {
        self.tail.index_ref()
    }

    fn column_index(&self) -> graphblas_sparse_linear_algebra::index::ElementIndex {
        self.head.index()
    }

    fn column_index_ref(&self) -> &graphblas_sparse_linear_algebra::index::ElementIndex {
        self.head.index_ref()
    }
}
