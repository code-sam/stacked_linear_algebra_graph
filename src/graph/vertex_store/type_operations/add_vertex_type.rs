use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::VertexTypeIndex,
        indexer::{AssignedIndexTrait, IndexerTrait},
        vertex::vertex::VertexTypeKeyRef,
        vertex_store::{
            vertex_store::VertexStoreTrait, VertexMatrix, VertexMatrixTrait, VertexStore,
        },
    },
};

pub(crate) trait AddVertexType {
    fn add_new_vertex_type(
        &mut self,
        key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError>;
}

impl AddVertexType for VertexStore {
    fn add_new_vertex_type(
        &mut self,
        key: &VertexTypeKeyRef,
    ) -> Result<VertexTypeIndex, GraphComputingError> {
        let new_type_index = self.vertex_type_indexer_mut_ref().add_new_key(key)?;
        if let Some(new_capacity) = new_type_index.new_index_capacity() {
            self.vertex_matrix_mut_ref()
                .set_vertex_type_capacity(new_capacity);
        }
        Ok(*new_type_index.index_ref())
    }
}
