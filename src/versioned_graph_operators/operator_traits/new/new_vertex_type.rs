use crate::{
    error::GraphComputingError,
    graph::{indexing::VertexTypeIndex, value_type::ValueType}, versioned_graph::indexing::VersionedVertexTypeIndex,
};

pub trait NewVertexTypeVersioned<T: ValueType> {
    fn apply(&mut self) -> Result<VersionedVertexTypeIndex, GraphComputingError>;
}
