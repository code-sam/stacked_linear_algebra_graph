use crate::{
    error::GraphComputingError,
    graph::{indexing::VertexTypeIndex, value_type::ValueType},
};

pub trait NewVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}
