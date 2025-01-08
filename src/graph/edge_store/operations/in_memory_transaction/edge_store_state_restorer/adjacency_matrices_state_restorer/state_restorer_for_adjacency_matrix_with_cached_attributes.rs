use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    GetSparseMatrixElementValueTyped, SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    ColumnIndex, RowIndex, Size, SparseMatrix,
};

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_attribute_caching::InvalidateChachedAdjacencyMatrixAttributes;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::{
    GetWeightedAdjacencyMatrix, WeightedAdjacencyMatrixWithCachedAttributes,
};
use crate::graph::value_type::ValueType;
use crate::operators::in_memory_transaction::transaction::{
    restore_sparse_matrix_state, GetSparseMatrixSizeToRestore, RegisterSparseMatrixChangeToRevert,
    SparseMatrixStateReverter, SparseMatrixStateToRestore,
};
use crate::operators::transaction::RestoreState;

pub(crate) struct StateRestorerForAdjacencyMatrixWithCachedAttributes<T: ValueType> {
    sparse_matrix_state_reverter: SparseMatrixStateReverter<T>,
}

impl<T: ValueType> GetSparseMatrixSizeToRestore
    for StateRestorerForAdjacencyMatrixWithCachedAttributes<T>
{
    fn matrix_size_to_restore(&self) -> Option<Size> {
        self.sparse_matrix_state_reverter.matrix_size_to_restore()
    }

    fn matrix_size_to_restore_ref(&self) -> &Option<Size> {
        self.sparse_matrix_state_reverter
            .matrix_size_to_restore_ref()
    }
}

impl<T> RegisterSparseMatrixChangeToRevert<T>
    for StateRestorerForAdjacencyMatrixWithCachedAttributes<T>
where
    T: ValueType
        + GetSparseMatrixElementValueTyped<T>
        + SetSparseMatrixElementTyped<T>
        + Default
        + Clone,
{
    fn register_element_value_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        element_value: T,
    ) {
        self.sparse_matrix_state_reverter
            .register_element_value_to_restore(row_index, column_index, element_value.clone());
    }

    fn register_empty_element_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) {
        self.sparse_matrix_state_reverter
            .register_empty_element_to_restore(row_index, column_index)
    }

    fn register_sparse_matrix_state_to_restore(&mut self, sparse_matrix: SparseMatrix<T>) {
        self.sparse_matrix_state_reverter
            .register_sparse_matrix_state_to_restore(sparse_matrix)
    }

    fn register_size_to_restore(&mut self, size: Size) {
        self.sparse_matrix_state_reverter
            .register_size_to_restore(size);
    }
}

// pub(crate) trait CreateStateReverterForAdjacencyMatrixWithCachedAttributes<T: ValueType> {
//     fn adjacency_matrix_with_cached_attributes_state_reverter_with_size_to_restore(
//         size_to_restore: Size,
//     ) -> StateRestorerForAdjacencyMatrixWithCachedAttributes<T>;
// }

// macro_rules! implement_create_state_reverter_for_adjacency_matrix_with_cached_attributes {
//     ($value_type:ty) => {
//         impl CreateStateReverterForAdjacencyMatrixWithCachedAttributes<$value_type> for $value_type {
//             fn adjacency_matrix_with_cached_attributes_state_reverter_with_size_to_restore(
//                 size_to_restore: Size,
//             ) -> StateRestorerForAdjacencyMatrixWithCachedAttributes<$value_type> {
//                 StateRestorerForAdjacencyMatrixWithCachedAttributes::<$value_type>::with_size_to_restore(size_to_restore)
//             }
//         }
//     };
// }
// implement_macro_for_all_native_value_types!(
//     implement_create_state_reverter_for_adjacency_matrix_with_cached_attributes
// );

impl<T: ValueType> StateRestorerForAdjacencyMatrixWithCachedAttributes<T> {
    pub(crate) fn new(
        size_to_restore: Option<Size>,
        state_to_restore: Vec<SparseMatrixStateToRestore<T>>,
        is_state_to_restore_fully_determined: bool,
    ) -> Self {
        let sparse_matrix_state_reverter = SparseMatrixStateReverter::new(
            size_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        );
        Self {
            sparse_matrix_state_reverter,
        }
    }

    pub(crate) fn new_default() -> Self {
        let sparse_matrix_state_reverter = SparseMatrixStateReverter::new_default();
        Self {
            sparse_matrix_state_reverter,
        }
    }

    // pub(crate) fn with_size_to_restore(size_to_restore: Size) -> Self {
    //     Self::new(size_to_restore, Vec::new(), false)
    // }

    // pub(crate) fn with_size_to_restore_from_sparse_matrix(
    //     to_restore: &SparseMatrix<T>,
    // ) -> Result<Self, GraphComputingError> {
    //     let size_to_restore = to_restore.size()?;
    //     Ok(Self::with_size_to_restore(size_to_restore))
    // }

    // pub(crate) fn from_sparse_matrix(
    //     to_restore: &SparseMatrix<T>,
    // ) -> Result<Self, GraphComputingError> {
    //     let sparse_matrix_state_reverter =
    //         SparseMatrixStateReverter::from_sparse_matrix(to_restore)?;

    //     Ok(Self {
    //         sparse_matrix_state_reverter,
    //     })
    // }
}

impl<T: ValueType + SetSparseMatrixElementTyped<T>>
    RestoreState<WeightedAdjacencyMatrixWithCachedAttributes>
    for StateRestorerForAdjacencyMatrixWithCachedAttributes<T>
{
    fn restore(
        self,
        instance_to_restore: &mut WeightedAdjacencyMatrixWithCachedAttributes,
    ) -> Result<(), GraphComputingError> {
        instance_to_restore.invalidate_all_attributes();

        restore_sparse_matrix_state(
            self.sparse_matrix_state_reverter,
            instance_to_restore.weighted_adjacency_matrix_mut_ref(),
        )
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self {
            sparse_matrix_state_reverter: self
                .sparse_matrix_state_reverter
                .with_reset_state_to_restore(),
        }
    }
}

// pub(crate) fn restore_sparse_matrix_state<T: ValueType + SetSparseMatrixElementTyped<T>>(
//     state_reverter: SparseMatrixStateReverter<T>,
//     instance_to_restore: &mut WeightedAdjacencyMatrixWithCachedAttributes,
// ) -> Result<(), GraphComputingError> {
//     instance_to_restore = WeightedAdjacencyMatrixWithCachedAttributes::fr(graphblas_context, initial_vertex_capacity)
//     instance_to_restore.weighted_adjacency_matrix_cached_attributes_mut_ref() = None;
// }

#[cfg(test)]
mod tests {}
