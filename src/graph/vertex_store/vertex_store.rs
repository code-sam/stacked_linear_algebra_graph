use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::GraphComputingError;
use crate::graph::graph::GetGraphblasContext;
use crate::graph::indexing::traits::GetValidIndices;
use crate::graph::indexing::{ElementCount, Index, Indexer, VertexTypeIndex};

use super::VertexVector;

pub(crate) type VertexTypeIndexer = Indexer;
pub(crate) type VertexElementIndexer = Indexer;

#[derive(Clone, Debug)]
pub(crate) struct VertexStore {
    graphblas_context: Arc<GraphblasContext>,
    vertex_type_indexer: VertexTypeIndexer,
    vertex_vectors: Vec<VertexVector>,
    element_indexer: VertexElementIndexer,
}

impl VertexStore {
    pub(crate) fn with_initial_capacity(
        context: Arc<GraphblasContext>,
        initial_vertex_type_capacity: ElementCount,
        initial_vertex_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let vertex_type_indexer = VertexTypeIndexer::with_initial_capacity(
            context.clone(),
            initial_vertex_type_capacity,
        )?;
        let element_indexer =
            VertexElementIndexer::with_initial_capacity(context.clone(), initial_vertex_capacity)?;

        let vertex_vectors = Vec::with_capacity(initial_vertex_type_capacity);

        Ok(Self {
            graphblas_context: context.clone(),
            vertex_type_indexer,
            vertex_vectors,
            element_indexer,
        })
    }
}

pub(crate) trait GetVertexTypeIndexer {
    fn vertex_type_indexer_ref(&self) -> &VertexTypeIndexer;
    fn vertex_type_indexer_mut_ref(&mut self) -> &mut VertexTypeIndexer;
}

pub(crate) trait GetVertexElementIndexer {
    fn element_indexer_ref(&self) -> &VertexElementIndexer;
    fn element_indexer_mut_ref(&mut self) -> &mut VertexElementIndexer;
}

pub(crate) trait GetVertexVectors {
    fn vertex_vector_for_all_vertex_types_ref(&self) -> &[VertexVector];
    fn vertex_vector_for_all_vertex_types_mut_ref(&mut self) -> &mut [VertexVector];
    fn vertex_vector_for_all_vertex_types_mut(&mut self) -> &mut Vec<VertexVector>;
}

impl GetGraphblasContext for VertexStore {
    fn graphblas_context(&self) -> Arc<GraphblasContext> {
        self.graphblas_context.to_owned()
    }

    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
}

impl GetVertexTypeIndexer for VertexStore {
    fn vertex_type_indexer_ref(&self) -> &VertexTypeIndexer {
        &self.vertex_type_indexer
    }
    fn vertex_type_indexer_mut_ref(&mut self) -> &mut VertexTypeIndexer {
        &mut self.vertex_type_indexer
    }
}

impl GetVertexElementIndexer for VertexStore {
    fn element_indexer_ref(&self) -> &VertexElementIndexer {
        &self.element_indexer
    }
    fn element_indexer_mut_ref(&mut self) -> &mut VertexElementIndexer {
        &mut self.element_indexer
    }
}

impl GetVertexVectors for VertexStore {
    fn vertex_vector_for_all_vertex_types_ref(&self) -> &[VertexVector] {
        self.vertex_vectors.as_slice()
    }

    fn vertex_vector_for_all_vertex_types_mut_ref(&mut self) -> &mut [VertexVector] {
        self.vertex_vectors.as_mut_slice()
    }

    fn vertex_vector_for_all_vertex_types_mut(&mut self) -> &mut Vec<VertexVector> {
        &mut self.vertex_vectors
    }
}

// Implemented in module to work around limitations of the borrow checker
impl VertexStore {
    pub(crate) fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer
            .iter_valid_indices()?
            .try_for_each(|i: Index| function_to_apply(&mut self.vertex_vectors[i]))?;
        Ok(())
    }

    pub(crate) fn indexed_map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        mut function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: FnMut(&VertexTypeIndex, &mut VertexVector) -> Result<(), GraphComputingError>
            + Send
            + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer
            .iter_valid_indices()?
            .try_for_each(|i: Index| {
                function_to_apply(&VertexTypeIndex::new(i), &mut self.vertex_vectors[i])
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn add_new_vertex() {
    //     let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

    //     let mut store = VertexStore::<u8>::with_initial_capacity(&graphblas_context, &10).unwrap();

    //     let vertex_1 = Vertex::new(String::from("key"), 1u8);

    //     let index_1 = store.add_new_vertex(vertex_1.clone()).unwrap();
    //     assert!(store.is_valid_index(&index_1).unwrap());
    //     assert!(store.is_valid_key(vertex_1.key_ref()));
    // }
}
