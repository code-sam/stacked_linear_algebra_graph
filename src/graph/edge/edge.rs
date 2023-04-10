use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::graph::Graph;
use crate::graph::graph::GraphTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::VertexKey;

use super::{DirectedEdgeCoordinateDefinedByIndices, DirectedEdgeCoordinateDefinedByKeys};
use super::{EdgeTypeIndex, EdgeTypeKey, EdgeTypeKeyRef};

#[derive(Clone, Debug)]
pub struct WeightedDirectedEdgeDefinedByIndices<T: ValueType> {
    coordinate: DirectedEdgeCoordinateDefinedByIndices,
    weight: T,
}

impl<T: ValueType> WeightedDirectedEdgeDefinedByIndices<T> {
    pub fn new(coordinate: DirectedEdgeCoordinateDefinedByIndices, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

pub trait WeightedDirectedEdgeDefinedByIndicesTrait<T: ValueType> {
    fn coordinate_ref(&self) -> &DirectedEdgeCoordinateDefinedByIndices;
    fn weight_ref(&self) -> &T;
}

impl<T: ValueType> WeightedDirectedEdgeDefinedByIndicesTrait<T>
    for WeightedDirectedEdgeDefinedByIndices<T>
{
    fn coordinate_ref(&self) -> &DirectedEdgeCoordinateDefinedByIndices {
        &self.coordinate
    }

    fn weight_ref(&self) -> &T {
        &self.weight
    }
}

#[derive(Clone, Debug)]
pub struct WeightedDirectedEdgeDefinedByKeys<T: ValueType> {
    coordinate: DirectedEdgeCoordinateDefinedByKeys,
    weight: T,
}

impl<T: ValueType> WeightedDirectedEdgeDefinedByKeys<T> {
    pub fn new(coordinate: DirectedEdgeCoordinateDefinedByKeys, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

pub trait WeightedDirectedEdgeDefinedByKeysTrait<T: ValueType> {
    fn coordinate_ref(&self) -> &DirectedEdgeCoordinateDefinedByKeys;
    fn weight_ref(&self) -> &T;
}

impl<T: ValueType> WeightedDirectedEdgeDefinedByKeysTrait<T>
    for WeightedDirectedEdgeDefinedByKeys<T>
{
    fn coordinate_ref(&self) -> &DirectedEdgeCoordinateDefinedByKeys {
        &self.coordinate
    }

    fn weight_ref(&self) -> &T {
        &self.weight
    }
}
