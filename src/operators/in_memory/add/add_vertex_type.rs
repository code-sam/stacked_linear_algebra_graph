use crate::error::GraphComputingError;
use crate::graph::graph::{GetVertexStore, Graph};
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::vertex_type::{
    AddPrivateVertexType as AddPublicVertexTypeToVertexStore, AddPublicVertexType,
};
use crate::operators::operators::add::{AddPrivateVertexType, AddVertexType};

impl<T: ValueType + GetValueTypeIdentifier> AddVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddPublicVertexType::<T>::apply(self.vertex_store_mut_ref())
    }
}

impl<T: ValueType + GetValueTypeIdentifier> AddPrivateVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddPublicVertexTypeToVertexStore::<T>::apply(self.vertex_store_mut_ref())
    }
}
