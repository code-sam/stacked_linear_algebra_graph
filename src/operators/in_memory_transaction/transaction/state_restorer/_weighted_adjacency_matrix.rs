use graphblas_sparse_linear_algebra::{
    collections::sparse_matrix::{
        operations::{
            drop_sparse_matrix_element, resize_sparse_matrix, DropSparseMatrixElement,
            GetSparseMatrixElementValueTyped, GetSparseMatrixSize, ResizeSparseMatrix,
            SetSparseMatrixElement, SetSparseMatrixElementTyped,
        },
        ColumnIndex, GetGraphblasSparseMatrix, RowIndex, Size, SparseMatrix,
    },
    graphblas_bindings::GrB_Matrix,
};

use crate::{
    error::GraphComputingError,
    graph::{value_type::ValueType, weighted_adjacency_matrix::WeightedAdjacencyMatrix},
    operators::transaction::RestoreState,
};

use super::{
    graphblas_sparse_matrix::SparseMatrixStateReverter, restore_sparse_matrix_state,
    GetSparseMatrixSizeToRestore, RegisterSparseMatrixChangeToRevert,
};

pub(crate) enum WeightedAdjacencyMatrixStateToRestore<T: ValueType> {
    EmptyElement(RowIndex, ColumnIndex),
    ElementValue(RowIndex, ColumnIndex, T),
    AdjacencyMatrix(WeightedAdjacencyMatrix),
}

pub(crate) struct WeightedAdjacencyMatrixMatrixStateReverter<T: ValueType> {
    size_to_restore: Size,
    state_to_restore: Vec<WeightedAdjacencyMatrixStateToRestore<T>>,
    is_state_to_restore_fully_determined: bool,
}

impl<T: ValueType> GetSparseMatrixSizeToRestore for WeightedAdjacencyMatrixMatrixStateReverter<T> {
    fn matrix_size_to_restore_ref(&self) -> &Size {
        &self.size_to_restore
    }
}

pub(crate) trait RegisterWeightedAdjacencyMatrixChangeToRevert<T: ValueType> {
    fn register_element_value_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        element_value: T,
    );

    fn register_empty_element_to_restore(&mut self, row_index: RowIndex, column_index: ColumnIndex);

    fn register_adjacency_matrix_state_to_restore(&mut self, adjacency_matrix: WeightedAdjacencyMatrix);
}


impl<T> RegisterWeightedAdjacencyMatrixChangeToRevert<T> for WeightedAdjacencyMatrixMatrixStateReverter<T>
where
    T: ValueType + GetSparseMatrixElementValueTyped<T> + SetSparseMatrixElementTyped<T> + Default,
{
    fn register_element_value_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        element_value: T,
    ) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(WeightedAdjacencyMatrixStateToRestore::ElementValue(
                    row_index,
                    column_index,
                    element_value,
                ))
        }
    }

    fn register_empty_element_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(WeightedAdjacencyMatrixStateToRestore::EmptyElement(
                    row_index,
                    column_index,
                ))
        }
    }

    fn register_adjacency_matrix_state_to_restore(&mut self, adjacency_matrix: WeightedAdjacencyMatrix) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(WeightedAdjacencyMatrixStateToRestore::AdjacencyMatrix(adjacency_matrix));
            self.is_state_to_restore_fully_determined = true;
        }
    }
}

impl<T: ValueType> WeightedAdjacencyMatrixMatrixStateReverter<T> {
    pub(crate) fn new(
        size_to_restore: Size,
        state_to_restore: Vec<WeightedAdjacencyMatrixStateToRestore<T>>,
        is_state_to_restore_fully_determined: bool,
    ) -> Self {
        Self {
            size_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        }
    }

    pub(crate) fn with_size_to_restore(size_to_restore: Size) -> Self {
        Self::new(size_to_restore, Vec::new(), false)
    }

    pub(crate) fn with_size_to_restore_from_adjacency_matrix(
        to_restore: &WeightedAdjacencyMatrix,
    ) -> Result<Self, GraphComputingError> {
        let size_to_restore = to_restore.size()?;
        Ok(Self::with_size_to_restore(size_to_restore))
    }

    pub(crate) fn from_adjacency_matrix(
        to_restore: &WeightedAdjacencyMatrix,
    ) -> Result<Self, GraphComputingError> {
        let state_to_restore = vec![WeightedAdjacencyMatrixStateToRestore::AdjacencyMatrix(
            to_restore.to_owned(),
        )];
        Ok(Self {
            size_to_restore: to_restore.size()?,
            state_to_restore: state_to_restore,
            is_state_to_restore_fully_determined: true,
        })
    }
}

impl<T: ValueType + SetSparseMatrixElementTyped<T>> RestoreState<WeightedAdjacencyMatrix>
    for WeightedAdjacencyMatrixMatrixStateReverter<T>
{
    fn restore(
        self,
        instance_to_restore: &mut WeightedAdjacencyMatrix,
    ) -> Result<(), GraphComputingError> {
        restore_adjacency_matrix_state(self, instance_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_size_to_restore(self.matrix_size_to_restore_ref().to_owned())
    }
}


pub(crate) fn restore_adjacency_matrix_state<T: ValueType + SetSparseMatrixElementTyped<T>>(
    state_reverter: SparseMatrixStateReverter<T>,
    instance_to_restore: &mut impl GetGraphblasSparseMatrix,
) -> Result<(), GraphComputingError> {
    for state_to_restore in state_reverter.state_to_restore.into_iter().rev() {
        match state_to_restore {
            WeightedAdjacencyMatrixStateToRestore::EmptyElement(column_index, row_index) => {
                drop_sparse_matrix_element(instance_to_restore, row_index, column_index)?
            }
            WeightedAdjacencyMatrixStateToRestore::ElementValue(column_index, row_index, element_value) => {
                T::set_graphblas_matrix_value(
                    instance_to_restore,
                    row_index,
                    column_index,
                    element_value,
                )?
            }
            WeightedAdjacencyMatrixStateToRestore::AdjacencyMatrix(mut sparse_matrix) => std::mem::swap(
                unsafe { instance_to_restore.graphblas_matrix_mut_ref() },
                unsafe { sparse_matrix.graphblas_matrix_mut_ref() },
            ),
        }
    }
    resize_sparse_matrix(instance_to_restore, state_reverter.size_to_restore)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::{
        collections::{sparse_matrix::operations::GetSparseMatrixElementValue, Collection},
        context::Context as GraphBLASContext,
    };

    use crate::{graph::weighted_adjacency_matrix::CreateWeightedAdjacencyMatrix, operators::in_memory_transaction::transaction::RegisterSparseMatrixChangeToRevert};

    use super::*;

    #[test]
    fn restore_sparse_matrix() {
        let context = GraphBLASContext::init_default().unwrap();
        let mut matrix = WeightedAdjacencyMatrix::new(context, 10).unwrap();

        u16::set_graphblas_matrix_value(&mut matrix, 1, 1, 1).unwrap();
        u16::set_graphblas_matrix_value(&mut matrix, 4, 4, 4).unwrap();
        u16::set_graphblas_matrix_value(&mut matrix, 5, 5, 5).unwrap();

        let mut state_reverter =
            WeightedAdjacencyMatrixMatrixStateReverter::with_size_to_restore_from_adjacency_matrix(&matrix).unwrap();

        drop_sparse_matrix_element(&mut matrix, 1, 1).unwrap();
        state_reverter.register_element_value_to_restore(1, 1, 1);

        u16::set_graphblas_matrix_value(&mut matrix, 0, 0, 0).unwrap();
        state_reverter.register_empty_element_to_restore(0, 0);

        u16::set_graphblas_matrix_value(&mut matrix, 0, 0, 10).unwrap();
        state_reverter.register_element_value_to_restore(0, 0, 10);

        u16::set_graphblas_matrix_value(&mut matrix, 4, 4, 40).unwrap();
        state_reverter.register_element_value_to_restore(4, 4, 4);

        state_reverter.register_adjacency_matrix_state_to_restore(matrix.clone());
        resize_sparse_matrix(&mut matrix, (4, 4).into()).unwrap();

        drop_sparse_matrix_element(&mut matrix, 5, 5).unwrap();
        state_reverter.register_element_value_to_restore(5, 5, 5);

        state_reverter.restore(&mut matrix).unwrap();

        assert_eq!(matrix.size().unwrap(), (10, 10).into());
        assert_eq!( GetSparseMatrixElementValueTyped::<u16>::element_value(&matrix, 0, 0).unwrap(), None);
        assert_eq!(matrix.element_value(1, 1).unwrap(), Some(1));
        assert_eq!(matrix.element_value(4, 4).unwrap(), Some(4));
        assert_eq!(matrix.number_of_stored_elements().unwrap(), 3);
    }
}
