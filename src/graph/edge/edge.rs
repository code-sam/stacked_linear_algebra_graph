use crate::graph::edge_store::weighted_adjacency_matrix::{
    AdjacencyMatrixCoordinate, GetAdjacencyMatrixCoordinateIndices,
};
use crate::graph::indexing::{EdgeTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;

use super::{DirectedEdgeCoordinate, GetDirectedEdgeCoordinateIndex};

#[derive(Clone, Debug)]
pub struct WeightedDirectedEdge<T: ValueType> {
    coordinate: DirectedEdgeCoordinate,
    weight: T,
}

impl<T: ValueType> WeightedDirectedEdge<T> {
    pub fn new(coordinate: DirectedEdgeCoordinate, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

pub trait GetEdgeWeight<T: ValueType> {
    fn weight(&self) -> T;
    fn weight_ref(&self) -> &T;
}

impl<T: ValueType + Clone + Copy> GetEdgeWeight<T> for WeightedDirectedEdge<T> {
    fn weight_ref(&self) -> &T {
        &self.weight
    }

    fn weight(&self) -> T {
        self.weight
    }
}

pub trait GetDirectedEdgeCoordinate {
    fn directed_coordinate_ref(&self) -> &DirectedEdgeCoordinate;
}

impl<T: ValueType> GetDirectedEdgeCoordinate for WeightedDirectedEdge<T> {
    fn directed_coordinate_ref(&self) -> &DirectedEdgeCoordinate {
        &self.coordinate
    }
}

impl<T: ValueType> GetDirectedEdgeCoordinateIndex for WeightedDirectedEdge<T> {
    fn edge_type_ref(&self) -> &EdgeTypeIndex {
        self.coordinate.edge_type_ref()
    }

    fn tail(&self) -> VertexIndex {
        self.coordinate.tail()
    }

    fn tail_ref(&self) -> &VertexIndex {
        self.coordinate.tail_ref()
    }

    fn head(&self) -> VertexIndex {
        self.coordinate.head()
    }

    fn head_ref(&self) -> &VertexIndex {
        self.coordinate.head_ref()
    }

    fn adjacency_matrix_coordinate(&self) -> AdjacencyMatrixCoordinate {
        AdjacencyMatrixCoordinate::new(
            self.coordinate.tail_ref().to_owned(),
            self.coordinate.head_ref().to_owned(),
        )
    }
}

impl<T: ValueType> GetAdjacencyMatrixCoordinateIndices for WeightedDirectedEdge<T> {
    fn tail(&self) -> VertexIndex {
        self.coordinate.tail()
    }

    fn tail_ref(&self) -> &VertexIndex {
        self.coordinate.tail_ref()
    }

    fn head(&self) -> VertexIndex {
        self.coordinate.head()
    }

    fn head_ref(&self) -> &VertexIndex {
        self.coordinate.head_ref()
    }
}
