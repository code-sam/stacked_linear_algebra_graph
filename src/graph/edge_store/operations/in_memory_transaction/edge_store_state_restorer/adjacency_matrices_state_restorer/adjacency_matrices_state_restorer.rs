use std::collections::HashMap;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::Size;

use crate::graph::indexing::{
    BuildIndexHasher, ElementCount, ElementIndex, ElementIndexMap, GetEdgeTypeIndex,
};
use crate::graph::value_type::{
    implement_1_type_macro_with_typed_indentifier_for_all_value_types, ValueType,
};

use super::state_restorer_for_adjacency_matrix_with_cached_attributes::{
    // CreateStateReverterForAdjacencyMatrixWithCachedAttributes,
    StateRestorerForAdjacencyMatrixWithCachedAttributes,
};

#[derive(Debug)]
pub(crate) struct AdjacencyMatricesWithCachedAttributesStateRestorer {
    pub(super) adjacency_matrix_size_to_restore: Option<Size>,
    pub(super) adjacency_matrix_vector_length_to_restore: ElementCount,
    pub(super) adjacency_matrix_state_reverters:
        TypedAdjacencyMatrixWithCachedAttributesStateReverters,
}

#[derive(Debug)]
pub(super) struct TypedAdjacencyMatrixWithCachedAttributesStateReverters {
    pub(super) adjacency_matrix_state_reverters_bool:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<bool>>,
    pub(super) adjacency_matrix_state_reverters_i8:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<i8>>,
    pub(super) adjacency_matrix_state_reverters_i16:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<i16>>,
    pub(super) adjacency_matrix_state_reverters_i32:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<i32>>,
    pub(super) adjacency_matrix_state_reverters_i64:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<i64>>,
    pub(super) adjacency_matrix_state_reverters_u8:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<u8>>,
    pub(super) adjacency_matrix_state_reverters_u16:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<u16>>,
    pub(super) adjacency_matrix_state_reverters_u32:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<u32>>,
    pub(super) adjacency_matrix_state_reverters_u64:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<u64>>,
    pub(super) adjacency_matrix_state_reverters_f32:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<f32>>,
    pub(super) adjacency_matrix_state_reverters_f64:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<f64>>,
    pub(super) adjacency_matrix_state_reverters_isize:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<isize>>,
    pub(super) adjacency_matrix_state_reverters_usize:
        ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<usize>>,
}

pub trait GetAdjacencyMatrixVectorLengthToRestore {
    fn adjacency_matrix_vector_length_to_restore(&self) -> ElementCount;
}

impl GetAdjacencyMatrixVectorLengthToRestore
    for AdjacencyMatricesWithCachedAttributesStateRestorer
{
    fn adjacency_matrix_vector_length_to_restore(&self) -> ElementCount {
        self.adjacency_matrix_vector_length_to_restore
    }
}

pub trait GetAdjacencyMatrixSizeToRestore {
    fn adjacency_matrix_size_to_restore(&self) -> Option<Size>;
}

impl GetAdjacencyMatrixSizeToRestore for AdjacencyMatricesWithCachedAttributesStateRestorer {
    fn adjacency_matrix_size_to_restore(&self) -> Option<Size> {
        self.adjacency_matrix_size_to_restore
    }
}

pub(crate) trait GetAdjacencyMatrixStateReverter<T: ValueType> {
    // fn sparse_vector_state_reverter_ref(
    //     &'a self,
    //     vertex_type_index: VertexTypeIndex,
    // ) -> &'a SparseVectorStateReverter<T>;
    fn adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut StateRestorerForAdjacencyMatrixWithCachedAttributes<T>;
}

impl<T> GetAdjacencyMatrixStateReverter<T> for AdjacencyMatricesWithCachedAttributesStateRestorer
where
    T: ValueType + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>, // + CreateStateReverterForAdjacencyMatrixWithCachedAttributes<T>,
{
    fn adjacency_matrix_with_cached_attributes_state_reverter_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &mut StateRestorerForAdjacencyMatrixWithCachedAttributes<T> {
        let adjacency_matrix_state_restorers_edge_type_map =
            T::adjacency_matrix_state_reverters_by_edge_type_map_mut_ref(
                &mut self.adjacency_matrix_state_reverters,
            );

        adjacency_matrix_state_restorers_edge_type_map
            .entry(edge_type_index.index())
            .or_insert_with(|| StateRestorerForAdjacencyMatrixWithCachedAttributes::new_default())
    }
}

pub(crate) trait GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T: ValueType> {
    fn adjacency_matrix_state_reverters_by_edge_type_map_ref(
        adjacency_matrix_state_restorer: &TypedAdjacencyMatrixWithCachedAttributesStateReverters,
    ) -> &ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<T>>;

    fn adjacency_matrix_state_reverters_by_edge_type_map_mut_ref(
        adjacency_matrix_state_restorer: &mut TypedAdjacencyMatrixWithCachedAttributesStateReverters,
    ) -> &mut ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<T>>;
}

macro_rules! implement_get_adjacency_matrix_state_reverter_by_vertex_type_map {
    ($typed_map_identifier: ident, $value_type: ty) => {
        impl<'a> GetAdjacencyMatrixStateRevertersByEdgeTypeMap<$value_type> for $value_type {
            fn adjacency_matrix_state_reverters_by_edge_type_map_ref(
                adjacency_matrix_state_restorer: &TypedAdjacencyMatrixWithCachedAttributesStateReverters,
            ) -> &ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<$value_type>> {
                &adjacency_matrix_state_restorer.$typed_map_identifier
            }

            fn adjacency_matrix_state_reverters_by_edge_type_map_mut_ref(
                adjacency_matrix_state_restorer: &mut TypedAdjacencyMatrixWithCachedAttributesStateReverters,
            ) -> &mut ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<$value_type>> {
                &mut adjacency_matrix_state_restorer.$typed_map_identifier
            }
        }
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_get_adjacency_matrix_state_reverter_by_vertex_type_map,
    adjacency_matrix_state_reverters
);

impl AdjacencyMatricesWithCachedAttributesStateRestorer {
    pub(crate) fn with_edge_type_length_to_restore(
        adjacency_matrix_vector_length_to_restore: ElementCount,
    ) -> Self {
        Self {
            adjacency_matrix_size_to_restore: None,
            adjacency_matrix_vector_length_to_restore,
            adjacency_matrix_state_reverters:
                TypedAdjacencyMatrixWithCachedAttributesStateReverters::new(),
        }
    }

    pub(crate) fn with_edge_type_length_and_adjacency_matrix_size_to_restore(
        adjacency_matrix_vector_length_to_restore: ElementCount,
        adjacency_matrix_size_to_restore: Option<Size>,
    ) -> Self {
        Self {
            adjacency_matrix_size_to_restore,
            adjacency_matrix_vector_length_to_restore,
            adjacency_matrix_state_reverters:
                TypedAdjacencyMatrixWithCachedAttributesStateReverters::new(),
        }
    }
}

pub(crate) trait GetAdjacencyMatrixStateReverters<T: ValueType> {
    fn adjacency_matrix_state_reverters_ref(
        &self,
    ) -> &ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<T>>;
    fn adjacency_matrix_state_reverters_mut_ref(
        &mut self,
    ) -> &mut ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<T>>;
}

macro_rules! implement_get_weighted_adjacency_matrix_state_reverters {
    ($typed_sparse_matrix_state_reverters_identifier:ident, $value_type:ty) => {
        impl GetAdjacencyMatrixStateReverters<$value_type>
            for AdjacencyMatricesWithCachedAttributesStateRestorer
        {
            fn adjacency_matrix_state_reverters_ref(
                &self,
            ) -> &ElementIndexMap<StateRestorerForAdjacencyMatrixWithCachedAttributes<$value_type>>
            {
                &self
                    .adjacency_matrix_state_reverters
                    .$typed_sparse_matrix_state_reverters_identifier
            }

            fn adjacency_matrix_state_reverters_mut_ref(
                &mut self,
            ) -> &mut ElementIndexMap<
                StateRestorerForAdjacencyMatrixWithCachedAttributes<$value_type>,
            > {
                &mut self
                    .adjacency_matrix_state_reverters
                    .$typed_sparse_matrix_state_reverters_identifier
            }
        }
    };
}
implement_1_type_macro_with_typed_indentifier_for_all_value_types!(
    implement_get_weighted_adjacency_matrix_state_reverters,
    adjacency_matrix_state_reverters
);

impl TypedAdjacencyMatrixWithCachedAttributesStateReverters {
    pub(crate) fn new() -> Self {
        Self {
            adjacency_matrix_state_reverters_bool: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<bool>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_i8: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<i8>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_i16: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<i16>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_i32: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<i32>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_i64: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<i64>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_u8: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<u8>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_u16: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<u16>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_u32: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<u32>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_u64: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<u64>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_f32: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<f32>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_f64: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<f64>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_isize: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<isize>,
                BuildIndexHasher,
            >::default(),
            adjacency_matrix_state_reverters_usize: HashMap::<
                ElementIndex,
                StateRestorerForAdjacencyMatrixWithCachedAttributes<usize>,
                BuildIndexHasher,
            >::default(),
        }
    }
}
