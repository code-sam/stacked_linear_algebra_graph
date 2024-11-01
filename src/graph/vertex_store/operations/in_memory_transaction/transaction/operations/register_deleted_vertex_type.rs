use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::operations::in_memory_transaction::RegisterFreedIndexToRestore;
use crate::graph::indexing::{
    AssignedIndex, GetAssignedIndexData, GetVertexTypeIndex, VertexIndex, VertexTypeIndex,
};
use crate::graph::indexing::{GetIndex, GetIndexCapacity};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, GetVertexStoreStateRestorer,
    GetVertexStoreStateReverters,
};
use crate::graph::vertex_store::operations::GetVertexVector;
use crate::graph::vertex_store::{CreateVertexVector, GetVertexElementIndexer, VertexVector};

use super::RegisterExpandedVertexCapacity;

pub(crate) trait RegisterDeletedVertexType<'t> {
    fn register_deleted_public_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;

    fn register_deleted_private_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl<'t> RegisterDeletedVertexType<'t> for AtomicInMemoryVertexStoreTransaction<'t> {
    fn register_deleted_public_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_deleted_public_vertex_type(vertex_type_index)
    }

    fn register_deleted_private_vertex_type(
        &'t mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.register_deleted_private_vertex_type(vertex_type_index)
    }
}
