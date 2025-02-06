use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::drop_sparse_vector_element;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::{operations::FreeIndex, GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_element::{
    DeleteVertexForAllTypes, DeleteVertexValue,
};
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::{
    GetVertexElementIndexer, GetVertexTypeIndexer, VertexStore, VertexVector,
};

impl DeleteVertexValue for VertexStore {
    fn delete_vertex_element(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index.index())?;
        self.delete_vertex_element_unchecked(vertex_type_index, vertex_index)
    }

    fn delete_vertex_element_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self.vertex_vector_mut_ref_unchecked(vertex_type_index)?;
        drop_sparse_vector_element(vertex_vector, *vertex_index.index_ref())?;
        Ok(())
    }
}

impl DeleteVertexForAllTypes for VertexStore {
    fn delete_vertex_for_all_valid_vertex_types_and_value_types(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_valid_vertex_vectors(|vertex_vector: &mut VertexVector| {
            Ok(drop_sparse_vector_element(
                vertex_vector,
                vertex_index.index(),
            )?)
        })?;

        self.element_indexer_mut_ref()
            .free_index_unchecked(vertex_index.index())?;
        self.element_indexer_mut_ref()
            .free_index_unchecked(vertex_index.index())
    }
}
