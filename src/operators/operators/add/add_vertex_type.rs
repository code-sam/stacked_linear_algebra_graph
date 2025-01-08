use crate::{
    error::GraphComputingError,
    graph::{indexing::VertexTypeIndex, value_type::ValueType},
};

pub trait AddVertexType<T: ValueType> {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError>;
}
