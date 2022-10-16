use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::Arc;

use rayon::prelude::*;

use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;
use graphblas_sparse_linear_algebra::value_types::sparse_matrix::{
    MatrixElement, SetMatrixElement, SparseMatrix,
};
use graphblas_sparse_linear_algebra::value_types::sparse_vector::{
    GetVectorElementValue, SetVectorElement, SparseVector, VectorElement,
};
use graphblas_sparse_linear_algebra::value_types::value_type::{BuiltInValueType, ValueType};

use super::indexer::{Indexer, IndexerTrait};
use crate::error::{GraphComputingError, LogicError, LogicErrorType};
// use crate::graph::edge::EdgeTypeIndex;
use crate::graph::index::{ElementCount, Index, IndexTrait, IndexedDataStoreIndex};
use crate::graph::native_data_type::{
    implement_macro_with_typed_variable_for_all_native_data_types, NativeDataType,
};
use crate::graph::vertex::{Vertex, VertexIndex};

pub(crate) trait IndexedVertexAndEdgeMatrixStoreTrait<T: NativeDataType> {
    // pub(crate) trait IndexedVertexAndEdgeMatrixStoreTrait<T: NativeDataType, I: IndexTrait + Debug> {
    /// Replacement deletes connected edges
    fn add_new_vertex_value(&mut self, vertex_value: T) -> Result<Index, GraphComputingError>;
    // fn update_vertex_value(&mut self, vertex_index: VertexIndex, vertex_value: T) -> Result<(), GraphComputingError>;

    // TODO: these types of methods should, where possible and efficient, only be present in the operations module.
    // fn new_vertex(&mut self, value: T) -> Result<VertexIndex, GraphComputingError>;

    // REVIEW: is the edge value optional, or in a specialised method?
    // fn new_edge(&mut self, edge_type_index: EdgeTypeIndex, edge: IndexDefinedDirectedEdge) -> Result<(), GraphComputingError>;
}

#[derive(Clone, Debug)]
pub(crate) struct IndexedVertexAndEdgeMatrixStore {
    vertex_indexer: Indexer,
    edge_type_indexer: Indexer,

    // TODO: should vertices be kept as a SparseVector or diagonal SparseMatrices? What's more efficient?
    // Using diagonal matrices may bring advantages for combined processing with edge data.
    // The underlying GraphBLAS implementation must however be optimized for diagional matrices,
    // especially in terms of access speed. TODO: bench access speed to diagonal matrices.
    vertices_bool: SparseMatrix<bool>,
    vertices_i8: SparseMatrix<i8>,
    vertices_i16: SparseMatrix<i16>,
    vertices_i32: SparseMatrix<i32>,
    vertices_i64: SparseMatrix<i64>,
    vertices_u8: SparseMatrix<u8>,
    vertices_u16: SparseMatrix<u16>,
    vertices_u32: SparseMatrix<u32>,
    vertices_u64: SparseMatrix<u64>,
    vertices_f32: SparseMatrix<f32>,
    vertices_f64: SparseMatrix<f64>,
    vertices_isize: SparseMatrix<isize>,
    vertices_usize: SparseMatrix<usize>,
    vertices_char: SparseMatrix<u32>, // REVIEW, note that some operations on the u32 may result in invalid chars. Though that's the same for other data types as well
    vertices_unit: SparseMatrix<bool>, // REVIEW

    edges_bool: Vec<SparseMatrix<bool>>,
    edges_i8: Vec<SparseMatrix<i8>>,
    edges_i16: Vec<SparseMatrix<i16>>,
    edges_i32: Vec<SparseMatrix<i32>>,
    edges_i64: Vec<SparseMatrix<i64>>,
    edges_u8: Vec<SparseMatrix<u8>>,
    edges_u16: Vec<SparseMatrix<u16>>,
    edges_u32: Vec<SparseMatrix<u32>>,
    edges_u64: Vec<SparseMatrix<u64>>,
    edges_f32: Vec<SparseMatrix<u32>>,
    edges_f64: Vec<SparseMatrix<u32>>,
    edges_isize: Vec<SparseMatrix<isize>>,
    edges_usize: Vec<SparseMatrix<usize>>,
    edges_char: Vec<SparseMatrix<u32>>,  // REVIEW
    edges_unit: Vec<SparseMatrix<bool>>, // REVIEW
}

impl IndexedVertexAndEdgeMatrixStoreTrait<bool> for IndexedVertexAndEdgeMatrixStore {
    fn add_new_vertex_value(&mut self, vertex_value: bool) -> Result<Index, GraphComputingError> {
        let claimed_index = self.vertex_indexer.claim_available_index()?;
        SetVertexValue::<bool>::set_vertex_value(self, claimed_index.index(), vertex_value);
        // self.set_vertex_value();

        Ok(claimed_index.index())

        // if available_index < self.data.len() {
        //     self.mask_with_valid_indices
        //         .set_element(VectorElement::from_pair(available_index, true))?;
        //     self.data[available_index] = data_to_push;
        // } else {
        //     // REVIEW: can the amount of calls to length() and len() be reduced?
        //     if available_index < self.mask_with_valid_indices.length()? {
        //         self.mask_with_valid_indices
        //             .set_element(VectorElement::from_pair(available_index, true))?;
        //         self.data.push(data_to_push);
        //     } else {
        //         self.data.push(data_to_push);
        //         match self.mask_with_valid_indices.resize(self.data.capacity()) {
        //             Ok(_) => {
        //                 match self
        //                     .mask_with_valid_indices
        //                     .set_element(VectorElement::from_pair(available_index, true))
        //                 {
        //                     Ok(_) => (),
        //                     Err(error) => {
        //                         self.data.pop();
        //                         return Err(error.into());
        //                     }
        //                 }
        //             }
        //             Err(error) => {
        //                 self.data.pop();
        //                 return Err(error.into());
        //             }
        //         }
        //     }
        // }
        // return Ok(IndexedDataStoreIndex::new(available_index));
    }
}

trait SetVertexValue<T: NativeDataType> {
    fn set_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError>;
}

macro_rules! implement_set_vertex_value {
    ($type_identifier: ty, $typed_postfix: ident) => {
        paste::paste! {
            impl SetVertexValue<$type_identifier> for IndexedVertexAndEdgeMatrixStore {
                fn set_vertex_value(
                    &mut self,
                    index: Index,
                    value: $type_identifier,
                ) -> Result<(), GraphComputingError> {
                    self.[<vertices_ $typed_postfix>]
                        .set_element((index, index, value).into())?;
                    Ok(())
                }
            }
        }
    };
}

implement_macro_with_typed_variable_for_all_native_data_types!(implement_set_vertex_value);

impl IndexedVertexAndEdgeMatrixStore {
    // pub fn new(
    //     graphblas_context: &Arc<GraphBLASContext>,
    //     initial_capacity: &ElementCount,
    // ) -> Result<Self, GraphComputingError> {

    // }

    pub fn with_initial_capacity(
        graphblas_context: &Arc<GraphBLASContext>,
        initial_vertex_capacity: &ElementCount,
        initial_edge_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let matrix_size_for_capacity = (
            initial_vertex_capacity.clone(),
            initial_vertex_capacity.clone(),
        )
            .into();
        Ok(Self {
            vertex_indexer: Indexer::with_initial_capacity(
                graphblas_context,
                initial_vertex_capacity,
            )?,
            edge_type_indexer: Indexer::with_initial_capacity(
                graphblas_context,
                initial_edge_type_capacity,
            )?,

            vertices_bool: SparseMatrix::<bool>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_i8: SparseMatrix::<i8>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_i16: SparseMatrix::<i16>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_i32: SparseMatrix::<i32>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_i64: SparseMatrix::<i64>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_u8: SparseMatrix::<u8>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_u16: SparseMatrix::<u16>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_u32: SparseMatrix::<u32>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_u64: SparseMatrix::<u64>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_f32: SparseMatrix::<f32>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_f64: SparseMatrix::<f64>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_isize: SparseMatrix::<isize>::new(
                graphblas_context,
                &matrix_size_for_capacity,
            )?,
            vertices_usize: SparseMatrix::<usize>::new(
                graphblas_context,
                &matrix_size_for_capacity,
            )?,
            vertices_char: SparseMatrix::<u32>::new(graphblas_context, &matrix_size_for_capacity)?,
            vertices_unit: SparseMatrix::<bool>::new(graphblas_context, &matrix_size_for_capacity)?,

            // TODO: would performance increase by edge-specific pre-allocation?
            edges_bool: Vec::<SparseMatrix<bool>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
            edges_i8: Vec::<SparseMatrix<i8>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_i16: Vec::<SparseMatrix<i16>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_i32: Vec::<SparseMatrix<i32>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_i64: Vec::<SparseMatrix<i64>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_u8: Vec::<SparseMatrix<u8>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_u16: Vec::<SparseMatrix<u16>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_u32: Vec::<SparseMatrix<u32>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_u64: Vec::<SparseMatrix<u64>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_f32: Vec::<SparseMatrix<u32>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_f64: Vec::<SparseMatrix<u32>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_isize: Vec::<SparseMatrix<isize>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
            edges_usize: Vec::<SparseMatrix<usize>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
            edges_char: Vec::<SparseMatrix<u32>>::with_capacity(initial_edge_type_capacity.clone()),
            edges_unit: Vec::<SparseMatrix<bool>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
        })
    }
}

// pub(crate) trait IndexedDataStoreTrait<T, I>
// where
//     T: NativeDataType + Send + Sync,
//     I: IndexTrait,
// {
//     fn push(
//         &mut self,
//         // &mut indexer:
//         data_to_push: T,
//     ) -> Result<I, GraphComputingError>;
//     fn get_ref(&self, index: I) -> Result<&T, GraphComputingError>;
//     fn get_mut_ref(&mut self, index: I) -> Result<&mut T, GraphComputingError>;
// }

// #[derive(Clone, Debug)]
// pub(crate) struct IndexedDataStore<T>
// where
//     T: NativeDataType + Send + Sync,
//     // I: IndexerTrait<N>,
//     // N: IndexTrait
// {
//     // _graphblas_context: Arc<GraphBLASContext>,
//     data: SparseVector<T>,
//     // public_to_private_index_map: SparseVector<Index>,
//     // mask_with_valid_public_indices: SparseVector<bool>,
//     // indexer: &'a I,
// }

// impl<T> IndexedDataStore<T>
// where
//     T: NativeDataType + Send + Sync,
//     // I: IndexTrait,
// {
//     pub(crate) fn with_capacity(
//         initial_capacity: &Index,
//         graphblas_context: &Arc<GraphBLASContext>,
//     ) -> Result<Self, GraphComputingError> {
//         Ok(Self {
//             // data: RwLock::new(Vec::with_capacity(*initial_capacity)),
//             // TODO: what if this fails? Include ref to Indexer such that index can be freed?
//             data: SparseVector::<T>::new(graphblas_context, initial_capacity)?,
//         })
//     }
// }

// impl<T: NativeDataType + Send + Sync, I: IndexTrait> IndexedDataStoreTrait<T, I>
//     for IndexedDataStore<T>
// {
//     fn push(&mut self, data_to_push: T) -> Result<IndexedDataStoreIndex, GraphComputingError> {}
// }

// impl<T: Send + Sync> IndexedDataStore<T, Indexer, IndexedDataStoreIndex> {
//     pub(crate) fn with_capacity(
//         initial_capacity: Index,
//         graphblas_context: Arc<GraphBLASContext>,
//     ) -> Result<Self, GraphComputingError> {
//         Ok(Self {
//             // data: RwLock::new(Vec::with_capacity(*initial_capacity)),
//             data: Vec::with_capacity(initial_capacity),
//             indices_available_for_reuse: VecDeque::new(),

//             _graphblas_context: graphblas_context.clone(),
//             mask_with_valid_indices: SparseVector::new(&graphblas_context, &initial_capacity)?,
//         })
//     }

//     pub(crate) fn push(
//         &mut self,
//         data_to_push: T,
//     ) -> Result<IndexedDataStoreIndex, GraphComputingError> {
//         let available_index = self.get_available_index()?;

//         // new indices are popped from a stack. Indices of freed indices are pushed to the stack, and re-used.
//         // benefit: no memory is allocated for unused indices
//         // downside: runtime cost; more complexity; no use of speedy pre-allocation; memory is never deallocated
//         // let data = self.get_write_locked_data()?;
//         if available_index < self.data.len() {
//             self.mask_with_valid_indices
//                 .set_element(VectorElement::from_pair(available_index, true))?;
//             self.data[available_index] = data_to_push;
//         } else {
//             // REVIEW: can the amount of calls to length() and len() be reduced?
//             if available_index < self.mask_with_valid_indices.length()? {
//                 self.mask_with_valid_indices
//                     .set_element(VectorElement::from_pair(available_index, true))?;
//                 self.data.push(data_to_push);
//             } else {
//                 self.data.push(data_to_push);
//                 match self.mask_with_valid_indices.resize(self.data.capacity()) {
//                     Ok(_) => {
//                         match self
//                             .mask_with_valid_indices
//                             .set_element(VectorElement::from_pair(available_index, true))
//                         {
//                             Ok(_) => (),
//                             Err(error) => {
//                                 self.data.pop();
//                                 return Err(error.into());
//                             }
//                         }
//                     }
//                     Err(error) => {
//                         self.data.pop();
//                         return Err(error.into());
//                     }
//                 }
//             }
//         }
//         return Ok(IndexedDataStoreIndex::new(available_index));
//     }

//     pub(crate) fn get_ref<I: IndexTrait>(&self, index: I) -> Result<&T, GraphComputingError> {
//         // #[cfg(debug_assertions)] // TODO: review performance cost of checking the validity of the index
//         self.check_index(&index)?;

//         Ok(&self.data[index.index()])
//     }

//     pub(crate) fn get_mut_ref<I: IndexTrait>(
//         &mut self,
//         index: I,
//     ) -> Result<&mut T, GraphComputingError> {
//         // #[cfg(debug_assertions)]
//         self.check_index(&index)?;

//         Ok(&mut self.data[index.index()])
//     }

//     pub(crate) fn is_valid_index<I: IndexTrait>(
//         &self,
//         index: &I,
//     ) -> Result<bool, GraphComputingError> {
//         Ok(self
//             .mask_with_valid_indices_ref()
//             .get_element_value(index.index_ref())?)
//     }

//     fn check_index<I: IndexTrait>(&self, index: &I) -> Result<(), GraphComputingError> {
//         if self.is_valid_index(index)? {
//             return Ok(());
//         } else {
//             return Err(LogicError::new(
//                 LogicErrorType::IndexOutOfBounds,
//                 format!(
//                     "No valid value at index [{}], the value may have been freed.",
//                     index.index_ref()
//                 ),
//                 None,
//             )
//             .into());
//         }
//     }

//     // The mask is updated at each push() and free() operation.
//     // benefit: mask is pre-computed, resulting in faster query operations
//     // downside: slower push() and free() operations
//     pub(crate) fn mask_with_valid_indices_ref(&self) -> &SparseVector<bool> {
//         &self.mask_with_valid_indices
//     }

//     /// Apply function to all stored elements
//     pub(crate) fn map_mut_all<F>(&mut self, function_to_apply: F) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&mut T) -> Result<(), GraphComputingError> + Send + Sync,
//     {
//         let result: Vec<_> = self
//             .data
//             .as_mut_slice()
//             .into_par_iter()
//             .map(function_to_apply)
//             .collect();
//         for result in result.into_iter() {
//             // TODO: consider parallelization
//             result?
//         }
//         Ok(()) // TODO: check result vector
//     }

//     pub(crate) fn update<I: IndexTrait>(
//         &mut self,
//         index: I,
//         data_to_set: T,
//     ) -> Result<(), GraphComputingError> {
//         // #[cfg(debug_assertions)]
//         self.check_index(&index)?;

//         self.data[index.index()] = data_to_set;
//         Ok(())
//     }

//     // data is not actually deleted. The index is only lined-up for reuse upon the next push of new data
//     pub(crate) fn free<I: IndexTrait + Debug>(
//         &mut self,
//         index: I,
//     ) -> Result<(), GraphComputingError> {
//         self.mask_with_valid_indices
//             .drop_element(index.index_ref().clone())?;
//         self.indices_available_for_reuse.push_back(index.index());
//         Ok(())
//     }

//     pub(crate) fn get_number_of_indexed_elements(&self) -> Result<Index, GraphComputingError> {
//         Ok(self.mask_with_valid_indices.number_of_stored_elements()?)
//     }

//     // includes freed elements
//     // pub(crate) fn get_number_stored_elements(&self) -> Index {
//     //     self.data.len()
//     // }

//     pub(crate) fn get_number_of_stored_and_reusable_elements(
//         &self,
//     ) -> Result<Index, GraphComputingError> {
//         // Ok(self.get_read_locked_data()?.len())
//         Ok(self.data.len())
//     }

//     pub(crate) fn get_capacity(&self) -> Result<Index, GraphComputingError> {
//         // Ok(self.get_read_locked_data()?.capacity())
//         Ok(self.data.capacity())
//     }

//     fn get_available_index(&mut self) -> Result<Index, GraphComputingError> {
//         match self.indices_available_for_reuse.pop_front() {
//             None => self.get_number_of_stored_and_reusable_elements(),
//             Some(index) => Ok(index),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::ops::AddAssign;

//     use graphblas_sparse_linear_algebra::context::Mode as GraphBLASMode;
//     use graphblas_sparse_linear_algebra::value_types::sparse_vector::GetVectorElementValue;

//     #[test]
//     fn new_store() {
//         let mut store = IndexedVertexAndEdgeMatrixStore::with_initial_capacity(
//             &GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
//             &10,
//             &10
//         )
//         .unwrap();

//         let value = 1;
//         let index = store.push(value.clone()).unwrap();
//         assert_eq!(store.get_ref(index).unwrap(), &value);

//         let another_value = 2;
//         let another_index = store.push(another_value.clone()).unwrap();
//         assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
//         assert_eq!(store.get_ref(index).unwrap(), &value)
//     }

//     #[test]
//     fn new_store_with_zero_capacity() {
//         let mut store = IndexedDataStore::<i32>::with_capacity(
//             0,
//             GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
//         )
//         .unwrap();
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .number_of_stored_elements()
//                 .unwrap(),
//             0
//         );
//         assert_eq!(store.mask_with_valid_indices_ref().length().unwrap(), 0);

//         let value = 1;
//         let index = store.push(value.clone()).unwrap();
//         assert_eq!(store.get_ref(index).unwrap(), &value);
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .number_of_stored_elements()
//                 .unwrap(),
//             1
//         );

//         let another_value = 2;
//         let another_index = store.push(another_value.clone()).unwrap();
//         assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
//         assert_eq!(store.get_ref(index).unwrap(), &value);
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .number_of_stored_elements()
//                 .unwrap(),
//             2
//         );

//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .get_element_value(&0)
//                 .unwrap(),
//             true
//         );
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .get_element_value(&1)
//                 .unwrap(),
//             true
//         );
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .get_element_value(&2)
//                 .unwrap(),
//             false
//         );
//     }

//     #[test]
//     fn store_expansion() {
//         let mut store = IndexedVertexAndEdgeMatrixStore::with_initial_capacity(
//             GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
//             1,
//         )
//         .unwrap();

//         let value = 1;
//         let index = store.push(value.clone()).unwrap();
//         assert_eq!(store.get_ref(index).unwrap(), &value);

//         let another_value = 2;
//         let another_index = store.push(another_value.clone()).unwrap();
//         assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
//         assert_eq!(store.get_ref(index).unwrap(), &value);

//         for test_value in 3..1000 {
//             let test_index = store.push(test_value.clone()).unwrap();
//             assert_eq!(store.get_ref(test_index).unwrap(), &test_value);
//             assert_eq!(store.get_ref(index).unwrap(), &value)
//         }

//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &1);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &3);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &4);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &6);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &31);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &100);
//     }

//     #[test]
//     fn free_data() {
//         let mut store = IndexedDataStore::<i32>::with_capacity(
//             1,
//             GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
//         )
//         .unwrap();

//         let value = 1;
//         let index = store.push(value.clone()).unwrap();
//         assert_eq!(store.get_ref(index).unwrap(), &value);
//         assert_eq!(index, IndexedDataStoreIndex::new(0));

//         let another_value = 2;
//         let another_index = store.push(another_value.clone()).unwrap();
//         assert_eq!(store.get_ref(another_index).unwrap(), &another_value);
//         assert_eq!(store.get_ref(index).unwrap(), &value);
//         assert_eq!(another_index, IndexedDataStoreIndex::new(1));

//         store.free(index.clone()).unwrap();
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .number_of_stored_elements()
//                 .unwrap(),
//             1
//         );
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .get_element_value(index.index_ref())
//                 .unwrap(),
//             false
//         );
//         assert_eq!(
//             store
//                 .mask_with_valid_indices_ref()
//                 .get_element_value(another_index.index_ref())
//                 .unwrap(),
//             true
//         );

//         let value_after_free = 3;
//         let index_after_free = store.push(value_after_free.clone()).unwrap();
//         assert_eq!(store.get_ref(index_after_free).unwrap(), &value_after_free);
//         assert_eq!(store.get_ref(index).unwrap(), &value_after_free);
//         assert_eq!(index_after_free, IndexedDataStoreIndex::new(0));

//         for test_value in 2..100 {
//             let test_index = store.push(test_value.clone()).unwrap();
//             assert_eq!(store.get_ref(test_index).unwrap(), &test_value);
//         }

//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &3);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &2);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &3);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &5);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &30);
//         assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &99);
//     }

//     // #[test]
//     // fn test_map_mut_all() {
//     //     let mut store = IndexedDataStore::<i32>::with_capacity(
//     //         1,
//     //         GraphBLASContext::init_ready(GraphBLASMode::NonBlocking).unwrap(),
//     //     )
//     //     .unwrap();

//     //     for test_value in 0..100 {
//     //         store.push(test_value.clone()).unwrap();
//     //     }

//     //     let add_one = |value: &mut i32| -> Result<(), GraphComputingError> {
//     //         value.add_assign(1);
//     //         Ok(())
//     //     };

//     //     store.map_mut_all(add_one).unwrap();

//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(0)).unwrap(), &1);
//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(2)).unwrap(), &3);
//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(3)).unwrap(), &4);
//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(5)).unwrap(), &6);
//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(30)).unwrap(), &31);
//     //     assert_eq!(store.get_ref(IndexedDataStoreIndex::new(99)).unwrap(), &100);
//     // }
// }
