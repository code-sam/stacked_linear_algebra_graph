use crate::error::GraphComputingError;
use crate::graph::indexing::ElementCount;
use crate::graph::vertex_store::operations::vertex_type::ResizeVertexVectors;
use crate::graph::vertex_store::GetVertexVectors;
use crate::graph::vertex_store::ResizeVertexVector;
use crate::graph::vertex_store::VertexStore;

impl ResizeVertexVectors for VertexStore {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        for vertex_vector in self.vertex_vector_for_all_vertex_types_mut_ref().iter_mut() {
            vertex_vector.resize(new_vertex_capacity)?;
        }
        Ok(())
    }
}
