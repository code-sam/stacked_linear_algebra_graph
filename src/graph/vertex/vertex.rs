use crate::error::GraphComputingError;
use crate::graph::graph::{VertexIndex, VertexTypeIndex};
use crate::graph::value_type::ValueType;

pub type VertexKey = String;
pub type VertexKeyRef = str;

pub type VertexTypeKey = String;
pub type VertexTypeKeyRef = str;

pub trait ToVertexIndex {
    fn vertex_index(&self) -> Result<VertexIndex, GraphComputingError>;
}

pub trait ToVertexTypeIndex {
    fn vertex_type_index(&self) -> Result<VertexTypeIndex, GraphComputingError>;
}

pub trait GetVertexValue<T: ValueType> {
    fn value_ref(&self) -> &T;
}

#[cfg(test)]
mod tests {}
