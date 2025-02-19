use crate::{
    error::GraphComputingError,
    graph::{indexing::VertexTypeIndex, value_type::ValueType},
    versioned_graph::indexing::VersionedVertexTypeIndex,
};

pub trait NewVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}
