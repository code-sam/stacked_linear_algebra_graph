use graphblas_sparse_linear_algebra::collections::sparse_matrix::GetCoordinateIndices;

use crate::graph::{
    edge_store::weighted_adjacency_matrix::AdjacencyMatrixCoordinate, indexing::{EdgeTypeIndex, VertexIndex},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectedEdgeCoordinate {
    edge_type: EdgeTypeIndex,
    tail: VertexIndex,
    head: VertexIndex,
}

impl DirectedEdgeCoordinate {
    pub fn new(edge_type: EdgeTypeIndex, tail: VertexIndex, head: VertexIndex) -> Self {
        // TODO: review if a self-connected edge is allowed
        Self {
            edge_type,
            tail,
            head,
        }
    }
}

pub trait GetDirectedEdgeCoordinateIndex {
    fn edge_type_ref(&self) -> &EdgeTypeIndex;
    fn tail_ref(&self) -> &VertexIndex;
    fn head_ref(&self) -> &VertexIndex;
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> AdjacencyMatrixCoordinate;
}

impl GetDirectedEdgeCoordinateIndex for DirectedEdgeCoordinate {
    fn edge_type_ref(&self) -> &EdgeTypeIndex {
        &self.edge_type
    }
    fn tail_ref(&self) -> &VertexIndex {
        &self.tail
    }
    fn head_ref(&self) -> &VertexIndex {
        &self.head
    }
    // TODO: consider caching
    fn adjacency_matrix_coordinate(&self) -> AdjacencyMatrixCoordinate {
        AdjacencyMatrixCoordinate::new(self.tail, self.head)
    }
}

impl GetCoordinateIndices for DirectedEdgeCoordinate {
    fn row_index(&self) -> graphblas_sparse_linear_algebra::index::ElementIndex {
        self.tail
    }

    fn row_index_ref(&self) -> &graphblas_sparse_linear_algebra::index::ElementIndex {
        &self.tail
    }

    fn column_index(&self) -> graphblas_sparse_linear_algebra::index::ElementIndex {
        self.head
    }

    fn column_index_ref(&self) -> &graphblas_sparse_linear_algebra::index::ElementIndex {
        &self.head_ref()
    }
}
