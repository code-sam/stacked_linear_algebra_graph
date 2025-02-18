use crate::graph::value_type::ValueType;
use crate::versioned_graph::indexing::VersionedEdgeTypeIndex;
use crate::{graph::edge::GetEdgeWeight, versioned_graph::indexing::VersionedVertexIndex};

use super::{
    GetVersionedAdjacencyMatrixCoordinateIndices, GetVersionedDirectedEdgeCoordinateIndex,
    VersionedAdjacencyMatrixCoordinate, VersionedDirectedEdgeCoordinate,
};

#[derive(Clone, Debug)]
pub struct VersionedWeightedDirectedEdge<T: ValueType> {
    coordinate: VersionedDirectedEdgeCoordinate,
    weight: T,
}

impl<T: ValueType> VersionedWeightedDirectedEdge<T> {
    pub fn new(coordinate: VersionedDirectedEdgeCoordinate, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

impl<T: ValueType + Clone + Copy> GetEdgeWeight<T> for VersionedWeightedDirectedEdge<T> {
    fn weight_ref(&self) -> &T {
        &self.weight
    }

    fn weight(&self) -> T {
        self.weight
    }
}

pub trait GetUniqueDirectedEdgeCoordinate {
    fn directed_coordinate_ref(&self) -> &VersionedDirectedEdgeCoordinate;
}

impl<T: ValueType> GetUniqueDirectedEdgeCoordinate for VersionedWeightedDirectedEdge<T> {
    fn directed_coordinate_ref(&self) -> &VersionedDirectedEdgeCoordinate {
        &self.coordinate
    }
}

impl<T: ValueType> GetVersionedDirectedEdgeCoordinateIndex for VersionedWeightedDirectedEdge<T> {
    fn edge_type_ref(&self) -> &VersionedEdgeTypeIndex {
        self.coordinate.edge_type_ref()
    }

    fn tail(&self) -> VersionedVertexIndex {
        self.coordinate.tail()
    }

    fn tail_ref(&self) -> &VersionedVertexIndex {
        self.coordinate.tail_ref()
    }

    fn head(&self) -> VersionedVertexIndex {
        self.coordinate.head()
    }

    fn head_ref(&self) -> &VersionedVertexIndex {
        self.coordinate.head_ref()
    }

    fn adjacency_matrix_coordinate(&self) -> VersionedAdjacencyMatrixCoordinate {
        VersionedAdjacencyMatrixCoordinate::new(
            self.coordinate.tail_ref().to_owned(),
            self.coordinate.head_ref().to_owned(),
        )
    }
}

impl<T: ValueType> GetVersionedAdjacencyMatrixCoordinateIndices
    for VersionedWeightedDirectedEdge<T>
{
    fn tail(&self) -> VersionedVertexIndex {
        self.coordinate.tail()
    }

    fn tail_ref(&self) -> &VersionedVertexIndex {
        self.coordinate.tail_ref()
    }

    fn head(&self) -> VersionedVertexIndex {
        self.coordinate.head()
    }

    fn head_ref(&self) -> &VersionedVertexIndex {
        self.coordinate.head_ref()
    }
}
