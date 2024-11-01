use crate::error::GraphComputingError;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::{
    AtomicInMemoryVertexStoreTransaction, GetVertexStore, RegisterUpdatedVertexVector,
};
use crate::graph::vertex_store::operations::{
    private_vertex_vector_mut_ref, private_vertex_vector_ref, public_vertex_vector_mut_ref,
    public_vertex_vector_ref, vertex_vector_mut_ref_unchecked, vertex_vector_native_value_type,
    vertex_vector_native_value_type_unchecked, vertex_vector_ref_unchecked, GetVertexVector,
    GetVertexVectorNativeValueType,
};
use crate::graph::vertex_store::{VertexStore, VertexVector};

impl<'s> GetVertexVector<'s> for AtomicInMemoryVertexStoreTransaction<'s> {
    fn public_vertex_vector_ref(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError> {
        public_vertex_vector_ref(self.vertex_store_ref(), vertex_type_index)
    }

    fn public_vertex_vector_mut_ref(
        &'s mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&'s mut VertexVector, GraphComputingError> {
        self.register_updated_vertex_vector(vertex_type_index)?;

        public_vertex_vector_mut_ref(self.vertex_store_mut_ref(), vertex_type_index)
    }

    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &impl GetVertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError> {
    //     store.vertex_vectors
    //     store.vertex_type_indexer_ref()
    //         .try_index_validity(vertex_type_index)?;
    //     Ok(store.vertex_vector_by_index_mut_ref_unchecked(vertex_type_index))
    // }

    fn private_vertex_vector_ref(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&VertexVector, GraphComputingError> {
        private_vertex_vector_ref(self.vertex_store_ref(), vertex_type_index)
    }

    fn private_vertex_vector_mut_ref(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError> {
        self.register_updated_vertex_vector(vertex_type_index)?;

        private_vertex_vector_mut_ref(self.vertex_store_mut_ref(), vertex_type_index)
    }
    // fn vertex_vector_by_index_mut_ref_unsafe(
    //     store: *mut VertexStore,
    //     vertex_type_index: &impl GetVertexTypeIndex,
    // ) -> Result<*mut VertexVector, GraphComputingError> {
    //     store.vertex_vectors
    //     store.vertex_type_indexer_ref()
    //         .try_index_validity(vertex_type_index)?;
    //     Ok(store.vertex_vector_by_index_mut_ref_unchecked(vertex_type_index))
    // }

    fn vertex_vector_ref_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &VertexVector {
        vertex_vector_ref_unchecked(self.vertex_store_ref(), vertex_type_index)
    }

    fn vertex_vector_mut_ref_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&mut VertexVector, GraphComputingError> {
        self.register_updated_vertex_vector(vertex_type_index)?;

        Ok(vertex_vector_mut_ref_unchecked(
            self.vertex_store_mut_ref(),
            vertex_type_index,
        ))
    }
}

impl<'t> GetVertexVectorNativeValueType for AtomicInMemoryVertexStoreTransaction<'t> {
    fn vertex_vector_native_value_type(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> Result<&ValueTypeIdentifier, GraphComputingError> {
        vertex_vector_native_value_type(self.vertex_store_ref(), vertex_type_index)
    }

    fn vertex_vector_native_value_type_unchecked(
        &self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &ValueTypeIdentifier {
        vertex_vector_native_value_type_unchecked(self.vertex_store_ref(), vertex_type_index)
    }
}
