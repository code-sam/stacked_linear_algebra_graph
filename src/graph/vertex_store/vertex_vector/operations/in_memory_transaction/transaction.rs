use std::sync::Arc;

use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::GetGraphblasSparseVector,
    context::{Context as GraphBLASContext, GetContext},
    graphblas_bindings::GrB_Vector,
    operators::mask::VectorMask,
};

use crate::graph::graph::GetGraphblasContext;
use crate::graph::value_type::{GetValueTypeIdentifierRef, ValueTypeIdentifier};
use crate::graph::vertex_store::{
    operations::in_memory_transaction::transaction::VertexStoreStateRestorer, VertexVector,
};

pub(crate) struct VertexVectorTransaction<'s> {
    pub(in crate::graph::vertex_store::vertex_vector::operations::in_memory_transaction) vertex_vector:
        &'s mut VertexVector,
    pub(in crate::graph::vertex_store::vertex_vector::operations::in_memory_transaction) vertex_store_state_restorer:
        &'s mut VertexStoreStateRestorer,
}

impl<'s> VertexVectorTransaction<'s> {
    pub(crate) fn new(
        vertex_vector: &'s mut VertexVector,
        vertex_store_state_restorer: &'s mut VertexStoreStateRestorer,
    ) -> Self {
        Self {
            vertex_vector,
            vertex_store_state_restorer,
        }
    }
}

impl<'s> GetGraphblasContext for VertexVectorTransaction<'s> {
    fn graphblas_context(&self) -> Arc<GraphBLASContext> {
        self.vertex_vector.graphblas_context()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.vertex_vector.graphblas_context_ref()
    }
}

impl<'s> GetContext for VertexVectorTransaction<'s> {
    fn context(&self) -> Arc<GraphBLASContext> {
        self.graphblas_context()
    }

    fn context_ref(&self) -> &Arc<GraphBLASContext> {
        &self.graphblas_context_ref()
    }
}

impl<'s> GetGraphblasSparseVector for VertexVectorTransaction<'s> {
    unsafe fn graphblas_vector(&self) -> GrB_Vector {
        GetGraphblasSparseVector::graphblas_vector(self.vertex_vector)
    }

    unsafe fn graphblas_vector_ref(&self) -> &GrB_Vector {
        self.vertex_vector.graphblas_vector_ref()
    }

    unsafe fn graphblas_vector_mut_ref(&mut self) -> &mut GrB_Vector {
        self.vertex_vector.graphblas_vector_mut_ref()
    }
}

impl<'s> VectorMask for VertexVectorTransaction<'s> {
    unsafe fn graphblas_vector(&self) -> GrB_Vector {
        VectorMask::graphblas_vector(self.vertex_vector)
    }
}

impl<'s> GetValueTypeIdentifierRef for VertexVectorTransaction<'s> {
    fn value_type_identifier_ref(&self) -> &ValueTypeIdentifier {
        &self.vertex_vector.value_type_identifier_ref()
    }
}
