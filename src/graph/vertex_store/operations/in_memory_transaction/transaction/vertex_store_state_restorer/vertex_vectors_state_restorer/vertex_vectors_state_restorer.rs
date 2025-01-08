use std::collections::HashMap;

use crate::graph::indexing::{
    BuildIndexHasher, ElementCount, ElementIndex, ElementIndexMap, GetVertexTypeIndex,
};
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};
use crate::operators::in_memory_transaction::transaction::SparseVectorStateReverter;

pub(crate) struct VertexVectorsStateRestorer {
    pub(super) vertex_vector_length_to_restore: Option<ElementCount>,
    pub(super) vertex_type_vector_length_to_restore: ElementCount,
    pub(super) vertex_vector_state_reverters: TypedSparseVectorStateReverters,
}

pub(super) struct TypedSparseVectorStateReverters {
    pub(super) sparse_vector_state_reverters_bool: ElementIndexMap<SparseVectorStateReverter<bool>>,
    pub(super) sparse_vector_state_reverters_i8: ElementIndexMap<SparseVectorStateReverter<i8>>,
    pub(super) sparse_vector_state_reverters_i16: ElementIndexMap<SparseVectorStateReverter<i16>>,
    pub(super) sparse_vector_state_reverters_i32: ElementIndexMap<SparseVectorStateReverter<i32>>,
    pub(super) sparse_vector_state_reverters_i64: ElementIndexMap<SparseVectorStateReverter<i64>>,
    pub(super) sparse_vector_state_reverters_u8: ElementIndexMap<SparseVectorStateReverter<u8>>,
    pub(super) sparse_vector_state_reverters_u16: ElementIndexMap<SparseVectorStateReverter<u16>>,
    pub(super) sparse_vector_state_reverters_u32: ElementIndexMap<SparseVectorStateReverter<u32>>,
    pub(super) sparse_vector_state_reverters_u64: ElementIndexMap<SparseVectorStateReverter<u64>>,
    pub(super) sparse_vector_state_reverters_f32: ElementIndexMap<SparseVectorStateReverter<f32>>,
    pub(super) sparse_vector_state_reverters_f64: ElementIndexMap<SparseVectorStateReverter<f64>>,
    pub(super) sparse_vector_state_reverters_isize:
        ElementIndexMap<SparseVectorStateReverter<isize>>,
    pub(super) sparse_vector_state_reverters_usize:
        ElementIndexMap<SparseVectorStateReverter<usize>>,
}

pub trait GetVertexTypeVectorLengthToRestore {
    fn vertex_type_vector_length_to_restore(&self) -> ElementCount;
}

impl GetVertexTypeVectorLengthToRestore for VertexVectorsStateRestorer {
    fn vertex_type_vector_length_to_restore(&self) -> ElementCount {
        self.vertex_type_vector_length_to_restore
    }
}

pub trait GetVertexVectorLengthToRestore {
    fn vertex_vector_length_to_restore(&self) -> Option<ElementCount>;
}

impl GetVertexVectorLengthToRestore for VertexVectorsStateRestorer {
    fn vertex_vector_length_to_restore(&self) -> Option<ElementCount> {
        self.vertex_vector_length_to_restore
    }
}

pub(crate) trait GetVertexVectorStateReverter<T: ValueType> {
    // fn sparse_vector_state_reverter_ref(
    //     &'a self,
    //     vertex_type_index: VertexTypeIndex,
    // ) -> &'a SparseVectorStateReverter<T>;
    fn vertex_vector_state_reverter_mut_ref(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &mut SparseVectorStateReverter<T>;
}

impl<T> GetVertexVectorStateReverter<T> for VertexVectorsStateRestorer
where
    T: ValueType + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn vertex_vector_state_reverter_mut_ref(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
    ) -> &mut SparseVectorStateReverter<T> {
        let sparse_vector_state_reverters_vertex_type_map =
            T::sparse_vector_state_reverters_by_vertex_type_map_mut_ref(
                &mut self.vertex_vector_state_reverters,
            );

        sparse_vector_state_reverters_vertex_type_map
            .entry(vertex_type_index.index())
            .or_insert_with(|| SparseVectorStateReverter::new_default())
    }
}

pub(crate) trait GetSparseVectorStateRevertersByVertexTypeMap<T: ValueType> {
    fn sparse_vector_state_reverters_by_vertex_type_map_ref(
        vertex_vectors_state_restorer: &TypedSparseVectorStateReverters,
    ) -> &ElementIndexMap<SparseVectorStateReverter<T>>;

    fn sparse_vector_state_reverters_by_vertex_type_map_mut_ref(
        vertex_vectors_state_restorer: &mut TypedSparseVectorStateReverters,
    ) -> &mut ElementIndexMap<SparseVectorStateReverter<T>>;
}

macro_rules! implement_get_sparse_vector_state_reverter_by_vertex_type_map {
    ($typed_map_identifier: ident, $value_type: ty) => {
        impl<'a> GetSparseVectorStateRevertersByVertexTypeMap<$value_type> for $value_type {
            fn sparse_vector_state_reverters_by_vertex_type_map_ref(
                vertex_vectors_state_restorer: &TypedSparseVectorStateReverters,
            ) -> &ElementIndexMap<SparseVectorStateReverter<$value_type>> {
                &vertex_vectors_state_restorer.$typed_map_identifier
            }

            fn sparse_vector_state_reverters_by_vertex_type_map_mut_ref(
                vertex_vectors_state_restorer: &mut TypedSparseVectorStateReverters,
            ) -> &mut ElementIndexMap<SparseVectorStateReverter<$value_type>> {
                &mut vertex_vectors_state_restorer.$typed_map_identifier
            }
        }
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_get_sparse_vector_state_reverter_by_vertex_type_map,
    sparse_vector_state_reverters
);

impl VertexVectorsStateRestorer {
    pub(crate) fn new(
        vertex_vector_length_to_restore: Option<ElementCount>,
        vertex_type_vector_length_to_restore: ElementCount,
    ) -> Self {
        Self {
            vertex_vector_length_to_restore,
            vertex_type_vector_length_to_restore,
            vertex_vector_state_reverters: TypedSparseVectorStateReverters::new(),
        }
    }

    pub(crate) fn with_vertex_type_vector_length_to_restore(
        vertex_type_vector_length_to_restore: ElementCount,
    ) -> Self {
        Self {
            vertex_vector_length_to_restore: None,
            vertex_type_vector_length_to_restore,
            vertex_vector_state_reverters: TypedSparseVectorStateReverters::new(),
        }
    }
}

pub(crate) trait GetVertexVectorStateReverters<T: ValueType> {
    fn vertex_vector_state_reverters_ref(&self) -> &ElementIndexMap<SparseVectorStateReverter<T>>;
    fn vertex_vector_state_reverters_mut_ref(
        &mut self,
    ) -> &mut ElementIndexMap<SparseVectorStateReverter<T>>;
}

macro_rules! implement_get_vertex_vector_state_reverters {
    ($typed_sparse_vector_state_reverters_identifier:ident, $value_type:ty) => {
        impl GetVertexVectorStateReverters<$value_type> for VertexVectorsStateRestorer {
            fn vertex_vector_state_reverters_ref(
                &self,
            ) -> &ElementIndexMap<SparseVectorStateReverter<$value_type>> {
                &self
                    .vertex_vector_state_reverters
                    .$typed_sparse_vector_state_reverters_identifier
            }

            fn vertex_vector_state_reverters_mut_ref(
                &mut self,
            ) -> &mut ElementIndexMap<SparseVectorStateReverter<$value_type>> {
                &mut self
                    .vertex_vector_state_reverters
                    .$typed_sparse_vector_state_reverters_identifier
            }
        }
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_get_vertex_vector_state_reverters,
    sparse_vector_state_reverters
);

impl TypedSparseVectorStateReverters {
    pub(crate) fn new() -> Self {
        Self {
            sparse_vector_state_reverters_bool: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<bool>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_i8: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<i8>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_i16: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<i16>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_i32: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<i32>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_i64: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<i64>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_u8: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<u8>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_u16: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<u16>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_u32: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<u32>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_u64: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<u64>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_f32: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<f32>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_f64: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<f64>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_isize: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<isize>,
                BuildIndexHasher,
            >::default(),
            sparse_vector_state_reverters_usize: HashMap::<
                ElementIndex,
                SparseVectorStateReverter<usize>,
                BuildIndexHasher,
            >::default(),
        }
    }
}
