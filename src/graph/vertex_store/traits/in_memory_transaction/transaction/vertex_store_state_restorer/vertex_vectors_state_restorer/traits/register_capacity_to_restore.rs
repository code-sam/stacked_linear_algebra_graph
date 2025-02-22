use graphblas_sparse_linear_algebra::index::ElementCount;

use crate::graph::vertex_store::traits::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetSparseVectorStateRevertersByVertexTypeMap, GetVertexVectorStateReverter, VertexVectorsStateRestorer};

pub(crate) trait RegisterVertexVectorCapacityToRestore {
    fn register_vertex_vector_capacity_to_restore(&mut self, vertex_capacity: &ElementCount);
}

impl RegisterVertexVectorCapacityToRestore for VertexVectorsStateRestorer {
    fn register_vertex_vector_capacity_to_restore(&mut self, vertex_capacity: &ElementCount) {
        if self.vertex_vector_length_to_restore.is_none() {
            self.vertex_vector_length_to_restore = Some(vertex_capacity.to_owned());
        }
    }
}
