use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use crate::error::GraphComputingError;
use crate::graph::indexing::{
    BuildIndexHasher, ElementCount, ElementIndex, ElementIndexMap, GetIndex, GetVertexTypeIndex,
    VertexTypeIndex,
};
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};
use crate::graph::vertex_store::VertexVector;
use crate::operators::in_memory_transaction::transaction::{
    CreateSparseVectorStateReverter, SparseVectorStateReverter,
};
use crate::operators::transaction::RestoreState;

pub(crate) struct VertexVectorsStateRestorer {
    length_to_restore: ElementCount,
    vertex_vector_state_reverters: TypedSparseVectorStateReverters,
}

struct TypedSparseVectorStateReverters {
    sparse_vector_state_reverters_bool: ElementIndexMap<SparseVectorStateReverter<bool>>,
    sparse_vector_state_reverters_i8: ElementIndexMap<SparseVectorStateReverter<i8>>,
    sparse_vector_state_reverters_i16: ElementIndexMap<SparseVectorStateReverter<i16>>,
    sparse_vector_state_reverters_i32: ElementIndexMap<SparseVectorStateReverter<i32>>,
    sparse_vector_state_reverters_i64: ElementIndexMap<SparseVectorStateReverter<i64>>,
    sparse_vector_state_reverters_u8: ElementIndexMap<SparseVectorStateReverter<u8>>,
    sparse_vector_state_reverters_u16: ElementIndexMap<SparseVectorStateReverter<u16>>,
    sparse_vector_state_reverters_u32: ElementIndexMap<SparseVectorStateReverter<u32>>,
    sparse_vector_state_reverters_u64: ElementIndexMap<SparseVectorStateReverter<u64>>,
    sparse_vector_state_reverters_f32: ElementIndexMap<SparseVectorStateReverter<f32>>,
    sparse_vector_state_reverters_f64: ElementIndexMap<SparseVectorStateReverter<f64>>,
    sparse_vector_state_reverters_isize: ElementIndexMap<SparseVectorStateReverter<isize>>,
    sparse_vector_state_reverters_usize: ElementIndexMap<SparseVectorStateReverter<usize>>,
}

pub trait GetLengthToRestore {
    fn length_to_restore(&self) -> ElementCount;
}

impl GetLengthToRestore for VertexVectorsStateRestorer {
    fn length_to_restore(&self) -> ElementCount {
        self.length_to_restore
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
    T: ValueType
        + GetSparseVectorStateRevertersByVertexTypeMap<T>
        + CreateSparseVectorStateReverter<T>,
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
            .or_insert_with(|| {
                T::sparse_vector_state_reverter_with_length_to_restore(self.length_to_restore)
            })
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
    pub(crate) fn with_length_to_restore(length_to_restore: ElementCount) -> Self {
        Self {
            length_to_restore,
            vertex_vector_state_reverters: TypedSparseVectorStateReverters::with_length_to_restore(
                length_to_restore,
            ),
        }
    }
}

// Implementation in module to circumvent ownership problems
pub(crate) fn restore_vertex_vectors_state(
    vertex_vectors_state_restorer: VertexVectorsStateRestorer,
    vectors_to_restore: &mut Vec<VertexVector>,
) -> Result<(), crate::error::GraphComputingError> {
    let vertex_vector_state_reverters = vertex_vectors_state_restorer.vertex_vector_state_reverters;

    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_bool,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i8,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i16,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u8,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u16,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_f32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_f64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_isize,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_usize,
        vectors_to_restore,
    )?;

    vectors_to_restore.truncate(vertex_vectors_state_restorer.length_to_restore);

    Ok(())
}

fn restore_vertex_vectors<T>(
    typed_sparse_vector_state_reverters_for_vertex_type: ElementIndexMap<
        SparseVectorStateReverter<T>,
    >,
    vectors_to_restore: &mut Vec<VertexVector>,
) -> Result<(), GraphComputingError>
where
    T: ValueType,
    SparseVectorStateReverter<T>: RestoreState<VertexVector>,
{
    for (vertex_type_index, sparse_vector_state_reverter) in
        typed_sparse_vector_state_reverters_for_vertex_type.into_iter()
    {
        sparse_vector_state_reverter.restore(&mut vectors_to_restore[vertex_type_index])?;
    }
    Ok(())
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
    pub(crate) fn with_length_to_restore(length_to_restore: ElementCount) -> Self {
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
