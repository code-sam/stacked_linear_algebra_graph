use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::graph::Graph;
use crate::graph::graph::GraphTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::{VertexIndex, VertexKey};
use crate::graph::vertex_store::operations::Indexing;

use super::{EdgeCoordinateDefinedByIndices, EdgeCoordinateDefinedByKeys};
use super::{EdgeTypeIndex, EdgeTypeKey, EdgeTypeKeyRef};

pub struct EdgeDefinedByIndices<T: ValueType> {
    coordinate: EdgeCoordinateDefinedByIndices,
    weight: T,
}

impl<T: ValueType> EdgeDefinedByIndices<T> {
    pub fn new(coordinate: EdgeCoordinateDefinedByIndices, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

pub trait EdgeDefinedByIndicesTrait<T: ValueType> {
    fn coordinate_ref(&self) -> &EdgeCoordinateDefinedByIndices;
    fn weight_ref(&self) -> &T;
}

impl<T: ValueType> EdgeDefinedByIndicesTrait<T> for EdgeDefinedByIndices<T> {
    fn coordinate_ref(&self) -> &EdgeCoordinateDefinedByIndices {
        &self.coordinate
    }

    fn weight_ref(&self) -> &T {
        &self.weight
    }
}

pub struct EdgeDefinedByKeys<T: ValueType> {
    coordinate: EdgeCoordinateDefinedByKeys,
    weight: T,
}

impl<T: ValueType> EdgeDefinedByKeys<T> {
    pub fn new(coordinate: EdgeCoordinateDefinedByKeys, weight: T) -> Self {
        Self { coordinate, weight }
    }
}

pub trait EdgeDefinedByKeysTrait<T: ValueType> {
    fn coordinate_ref(&self) -> &EdgeCoordinateDefinedByKeys;
    fn weight_ref(&self) -> &T;
}

impl<T: ValueType> EdgeDefinedByKeysTrait<T> for EdgeDefinedByKeys<T> {
    fn coordinate_ref(&self) -> &EdgeCoordinateDefinedByKeys {
        &self.coordinate
    }

    fn weight_ref(&self) -> &T {
        &self.weight
    }
}
