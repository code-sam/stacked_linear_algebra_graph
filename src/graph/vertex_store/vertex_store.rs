use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrixTrait;
use graphblas_sparse_linear_algebra::collections::sparse_vector::SparseVectorTrait;
use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;
use graphblas_sparse_linear_algebra::context::Context;
use graphblas_sparse_linear_algebra::operators::mask::SelectEntireVector;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::error::GraphComputingError;

use crate::graph::index::ElementCount;

use crate::graph::indexer::{Indexer, IndexerTrait};

use super::{VertexVector, VertexVectorTrait};

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
        context: &Arc<Context>,
        inital_vertex_type_capacity: &ElementCount,
        inital_vertex_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            graphblas_context: context.clone(),
            vertex_type_indexer: VertexTypeIndexer::with_initial_capacity(
                context,
                inital_vertex_type_capacity,
            )?,
            vertex_vectors: Vec::with_capacity(*inital_vertex_type_capacity),
            element_indexer: VertexElementIndexer::with_initial_capacity(
                context,
                inital_vertex_capacity,
            )?,
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

    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_vertex_vectors(|vertex_vector: &mut VertexVector| {
            vertex_vector.resize(new_vertex_capacity)
            // .sparse_matrix_mut_ref()
            // .resize(&(new_vertex_capacity, new_vertex_capacity).into())
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

// impl<T: MatrixDataType> SetVertexData<T> for VertexStore<T> {
//     fn set_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError> {
//         self.vertices.set_element((index, index, value).into())?;
//         Ok(())
//     }
// }

// macro_rules! set_sparse_matrix_capacity {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size)?;
//     };
// }

// impl SetCapacity for VertexStore {
//     fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_graph_indentifier_for_all_matrix_data_types!(set_sparse_matrix_capacity, vertices);
//         Ok(())
//     }
// }

// macro_rules! implement_set_capacity {
//     ($dummy:literal, $($y:ident),+) => {
//         impl SetCapacity for VertexStore {
//             fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 implement_set_capacity!($($y),+);
//     };
//     ($type_id:ident, $($y:ident),*) => {
//         paste::paste! {
//             self.[<vertices $type_id>].resize(&target_size)?;
//         }
//         implement_set_capacity!($($y),*);
//     };
//     ($type_id:ident) => {
//                 paste::paste! {
//                     self.[<vertices $type_id>].resize(&target_size)?;
//                 }
//                 Ok(())
//             }
//         }
//     }
// }

// macro_rules! implement_macro_for_all_graph_data_typed_parameter {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(
//             0,
//             _bool,
//             _i8,
//             _i16,
//             _i32,
//             _i64,
//             _u8,
//             _u16,
//             _u32,
//             _u64,
//             _isize,
//             _usize,
//             _char,
//             _unit
//         );
//     };
// }

// implement_macro_for_all_graph_data_typed_parameter!(implement_set_capacity);

// pub(crate) trait SetCapacityTyped<G: GraphNativeDataType, M: MatrixDataType> {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! implement_set_capacity_typed {
//     ($vertices_typed:ident, $graph_data_type:ty, $matrix_data_type:ty) => {
//         impl SetCapacityTyped<$graph_data_type, $matrix_data_type> for VertexStore {
//             fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 Ok(self.$vertices_typed.resize(&target_size)?)
//             }
//         }
//     };
// }

// impl SetCapacityTyped<bool> for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         Ok(self.vertices_bool.resize(&target_size)?)
//     }
// }

// implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types!(implement_set_capacity_typed, vertices);

// pub(crate) trait VertexData {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! resize_vertices {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size);
//     };
// }

// impl VertexData for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_indentifier_for_all_native_data_types!(resize_vertices, vertices);
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::{
        collections::sparse_vector::SparseVector,
        context::{Context as GraphblasContext, Mode as GraphblasMode},
        index::ElementIndex as GraphblasElementIndex,
    };

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
