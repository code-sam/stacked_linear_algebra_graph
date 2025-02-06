use crate::error::GraphComputingError;
use crate::graph::graph::{GetVertexStore, Graph};
use crate::graph::indexing::VertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};
use crate::graph::vertex_store::operations::vertex_type::AddVertexType as AddVertexTypeToVertexStore;
use crate::operators::operators::new::NewVertexType;

impl<T: ValueType + GetValueTypeIdentifier> NewVertexType<T> for Graph {
    fn apply(&mut self) -> Result<VertexTypeIndex, GraphComputingError> {
        AddVertexTypeToVertexStore::<T>::apply(self.vertex_store_mut_ref())
    }
}
