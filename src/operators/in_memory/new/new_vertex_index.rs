use crate::error::GraphComputingError;
use crate::graph::graph::GetVertexStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetAssignedIndexData, VertexIndex};
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex as CreateVertexIndexInVertexStore;
use crate::operators::operators::new::NewVertexIndex;

impl NewVertexIndex for Graph {
    fn new_vertex_index(&mut self) -> Result<VertexIndex, GraphComputingError> {
        let assigned_index = self.vertex_store_mut_ref().new_vertex_index()?;
        Ok(VertexIndex::new(assigned_index.index()))
    }
}
