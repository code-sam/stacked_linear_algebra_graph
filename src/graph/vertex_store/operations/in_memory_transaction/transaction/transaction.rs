use std::mem;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::in_memory_transaction::{
    AtomicInMemoryIndexerTransaction, RegisterFreedIndexToRestore,
};
use crate::graph::indexing::operations::GetValidIndices;
use crate::graph::indexing::{
    ElementCount, GetIndexCapacity, GetVertexTypeIndex, VertexIndex, VertexTypeIndex,
};
use crate::graph::value_type::GetValueTypeIdentifierRef;
use crate::graph::value_type::ValueTypeIdentifier;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::RegisterVertexValueToRestoreTyped;
use crate::graph::vertex_store::operations::{
    indexed_map_mut_all_valid_private_vertex_vectors,
    indexed_map_mut_all_valid_public_vertex_vectors, indexed_map_mut_all_valid_vertex_vectors,
    map_mut_all_valid_vertex_vectors, GetVertexVector, GetVertexVectorNativeValueType,
    ResizeVertexVectors,
};
use crate::graph::vertex_store::{
    GetVertexElementIndexer, GetVertexTypeIndexer, GetVertexVectors, VertexStore, VertexVector,
};
use crate::operators::transaction::{RestoreState, UseAtomicTransaction};

use super::vertex_store_state_restorer::RegisterVertexVectorToRestore;
use super::{
    GetVertexStoreStateReverters, RegisterVertexCapacityToRestore, VertexStoreStateRestorer,
};

pub(crate) trait UseVertexStoreTransaction: UseAtomicTransaction {}

// #[derive(Clone, Debug)]
// pub(crate) struct VertexStore {
//     graphblas_context: Arc<GraphblasContext>,
//     vertex_type_indexer: VertexTypeIndexer,
//     vertex_vectors: Vec<VertexVector>,
//     element_indexer: VertexElementIndexer,
// }

pub(crate) struct AtomicInMemoryVertexStoreTransaction<'s> {
    pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store:
        &'s mut VertexStore,
    pub(in crate::graph::vertex_store::operations::in_memory_transaction) vertex_store_state_restorer:
        VertexStoreStateRestorer,
}

impl<'s> AtomicInMemoryVertexStoreTransaction<'s> {
    pub(crate) fn new(vertex_store: &'s mut VertexStore) -> Result<Self, GraphComputingError> {
        let vertex_store_state_restorer = VertexStoreStateRestorer::new_for_indexers(
            vertex_store.vertex_type_indexer_ref(),
            vertex_store.element_indexer_ref(),
        )?;

        Ok(Self {
            vertex_store,
            vertex_store_state_restorer,
        })
    }
}

pub(crate) trait GetVertexStore {
    fn vertex_store_ref(&self) -> &VertexStore;
    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore;
}

impl<'t> GetVertexStore for AtomicInMemoryVertexStoreTransaction<'t> {
    fn vertex_store_ref(&self) -> &VertexStore {
        &self.vertex_store
    }

    fn vertex_store_mut_ref(&mut self) -> &mut VertexStore {
        &mut self.vertex_store
    }
}

pub(crate) trait GetVertexStoreStateRestorer {
    fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer;
    fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer;
}

impl<'t> GetVertexStoreStateRestorer for AtomicInMemoryVertexStoreTransaction<'t> {
    fn vertex_store_state_restorer_ref(&self) -> &VertexStoreStateRestorer {
        &self.vertex_store_state_restorer
    }

    fn vertex_store_state_restorer_mut_ref(&mut self) -> &mut VertexStoreStateRestorer {
        &mut self.vertex_store_state_restorer
    }
}

impl<'s> UseAtomicTransaction for AtomicInMemoryVertexStoreTransaction<'s> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        let reset_vertex_store_state_restorer = self
            .vertex_store_state_restorer
            .with_reset_state_to_restore();
        let vertex_store_state_restorer = mem::replace(
            &mut self.vertex_store_state_restorer,
            reset_vertex_store_state_restorer,
        );

        vertex_store_state_restorer.restore(&mut self.vertex_store)
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        self.vertex_store_state_restorer =
            VertexStoreStateRestorer::new_for_vertex_store(self.vertex_store)?;
        Ok(())
    }
}

impl<'s> Drop for AtomicInMemoryVertexStoreTransaction<'s> {
    fn drop(&mut self) {
        self.revert();
    }
}

impl<'s> AtomicInMemoryVertexStoreTransaction<'s> {
    pub(in crate::graph::vertex_store::operations::in_memory_transaction) fn register_vertex_value_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .vertex_vector_ref_unchecked(vertex_type_index);

        match vertex_vector.value_type_identifier_ref() {
            ValueTypeIdentifier::Bool => {
                bool::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int8 => {
                i8::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int16 => {
                i16::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int32 => {
                i32::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Int64 => {
                i64::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt8 => {
                u8::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt16 => {
                u16::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt32 => {
                u32::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::UInt64 => {
                u64::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float32 => {
                f32::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::Float64 => {
                f64::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::ISize => {
                isize::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
            ValueTypeIdentifier::USize => {
                usize::register_vertex_value_to_restore(
                    &mut self.vertex_store_state_restorer,
                    vertex_type_index,
                    vertex_vector,
                    vertex_index,
                )?;
            }
        }
        Ok(())
    }
}

impl<'s> AtomicInMemoryVertexStoreTransaction<'s> {
    fn register_updated_private_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .private_vertex_vector_ref(vertex_type_index)?;

        self.vertex_store_state_restorer
            .register_updated_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
        Ok(())
    }

    fn register_updated_public_vertex_vector_to_restore(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .public_vertex_vector_ref(vertex_type_index)?;

        self.vertex_store_state_restorer
            .register_updated_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
        Ok(())
    }

    fn register_updated_vertex_vector_to_restore_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        let vertex_vector = self
            .vertex_store
            .vertex_vector_ref_unchecked(vertex_type_index);

        self.vertex_store_state_restorer
            .register_updated_vertex_vector_to_restore(vertex_type_index, vertex_vector)?;
        Ok(())
    }
}
