use std::sync::Arc;

use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::graph::index::ElementCount;
use crate::graph::indexer::Indexer;
use crate::{error::GraphComputingError, graph::indexer::GetValidIndices};

use super::{ResizeWeightedAdjacencyMatrix, VertexVector};

pub(crate) type VertexTypeIndexer = Indexer;
pub(crate) type VertexElementIndexer = Indexer;

#[derive(Clone, Debug)]
pub(crate) struct VertexStore {
    graphblas_context: Arc<GraphblasContext>,
    vertex_type_indexer: VertexTypeIndexer,
    vertex_vectors: Vec<VertexVector>,
    element_indexer: VertexElementIndexer,

    mask_to_select_entire_vertex_vector: SelectEntireVector,
}

impl VertexStore {
    pub(crate) fn with_initial_capacity(
        context: &Arc<GraphblasContext>,
        initial_vertex_type_capacity: &ElementCount,
        initial_vertex_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let vertex_type_indexer =
            VertexTypeIndexer::with_initial_capacity(context, initial_vertex_type_capacity)?;
        let element_indexer =
            VertexElementIndexer::with_initial_capacity(context, initial_vertex_capacity)?;

        let vertex_matrix = Vec::with_capacity(*initial_vertex_type_capacity);

        Ok(Self {
            graphblas_context: context.clone(),
            vertex_type_indexer,
            vertex_vectors: vertex_matrix,
            element_indexer,
            mask_to_select_entire_vertex_vector: SelectEntireVector::new(context),
        })
    }
}

pub(crate) trait VertexStoreTrait {
    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext>;
    // fn set_vertex_vector_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
    // fn set_vertex_type_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;

    fn vertex_type_indexer_ref(&self) -> &VertexTypeIndexer;
    fn vertex_type_indexer_mut_ref(&mut self) -> &mut VertexTypeIndexer;

    fn element_indexer_ref(&self) -> &VertexElementIndexer;
    fn element_indexer_mut_ref(&mut self) -> &mut VertexElementIndexer;

    fn vertex_vector_for_all_vertex_types_ref(&self) -> &[VertexVector];
    fn vertex_vector_for_all_vertex_types_mut_ref(&mut self) -> &mut [VertexVector];
    fn vertex_vector_for_all_vertex_types_mut(&mut self) -> &mut Vec<VertexVector>;

    fn mask_to_select_entire_vertex_vector_ref(&self) -> &SelectEntireVector;

    // TODO: consider to move map methods to dedicated trait
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;

    fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_all_valid_vertex_vectors<F>(
        &self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
}

impl VertexStoreTrait for VertexStore {
    // TODO: implementation requires synchronization with adjacency matrices
    // fn set_vertex_vector_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
    //     self.vertices.resize(*new_capacity)?;
    //     Ok(())
    // }

    fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }

    fn vertex_type_indexer_ref(&self) -> &VertexTypeIndexer {
        &self.vertex_type_indexer
    }
    fn vertex_type_indexer_mut_ref(&mut self) -> &mut VertexTypeIndexer {
        &mut self.vertex_type_indexer
    }

    fn element_indexer_ref(&self) -> &VertexElementIndexer {
        &self.element_indexer
    }
    fn element_indexer_mut_ref(&mut self) -> &mut VertexElementIndexer {
        &mut self.element_indexer
    }

    fn vertex_vector_for_all_vertex_types_ref(&self) -> &[VertexVector] {
        self.vertex_vectors.as_slice()
    }

    fn vertex_vector_for_all_vertex_types_mut_ref(&mut self) -> &mut [VertexVector] {
        self.vertex_vectors.as_mut_slice()
    }

    fn vertex_vector_for_all_vertex_types_mut(&mut self) -> &mut Vec<VertexVector> {
        &mut self.vertex_vectors
    }

    fn mask_to_select_entire_vertex_vector_ref(&self) -> &SelectEntireVector {
        &self.mask_to_select_entire_vertex_vector
    }

    // TODO: is this method a useful abstraction, should it move to mod vertex_operations?
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_vertex_vectors(|vertex_vector: &mut VertexVector| {
            vertex_vector.resize(new_vertex_capacity)
        })?;
        Ok(())
    }

    fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.vertex_vectors
            .as_slice()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }

    fn map_mut_all_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.vertex_vectors
            .as_mut_slice()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }

    fn map_all_valid_vertex_vectors<F>(
        &self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| function_to_apply(&self.vertex_vectors[i]))?;
        Ok(())
    }

    fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| function_to_apply(&mut self.vertex_vectors[i]))?;
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
