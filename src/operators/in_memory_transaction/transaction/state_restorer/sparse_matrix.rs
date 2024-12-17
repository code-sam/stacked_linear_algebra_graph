use std::mem;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    drop_sparse_matrix_element, resize_sparse_matrix, GetSparseMatrixElementValueTyped,
    SetSparseMatrixElementTyped,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    ColumnIndex, GetGraphblasSparseMatrix, RowIndex, Size, SparseMatrix,
};

use crate::error::GraphComputingError;
use crate::graph::value_type::ValueType;
use crate::operators::transaction::RestoreState;

pub(crate) enum SparseMatrixStateToRestore<T: ValueType> {
    EmptyElement(RowIndex, ColumnIndex),
    ElementValue(RowIndex, ColumnIndex, T),
    SparseMatrix(SparseMatrix<T>),
}

pub(crate) struct SparseMatrixStateReverter<T: ValueType> {
    size_to_restore: Option<Size>,
    state_to_restore: Vec<SparseMatrixStateToRestore<T>>,
    is_state_to_restore_fully_determined: bool,
}

pub(crate) trait GetSparseMatrixSizeToRestore {
    fn matrix_size_to_restore(&self) -> Option<Size>;
    fn matrix_size_to_restore_ref(&self) -> &Option<Size>;
}

impl<T: ValueType> GetSparseMatrixSizeToRestore for SparseMatrixStateReverter<T> {
    fn matrix_size_to_restore(&self) -> Option<Size> {
        self.size_to_restore
    }

    fn matrix_size_to_restore_ref(&self) -> &Option<Size> {
        &self.size_to_restore
    }
}

pub(crate) trait RegisterSparseMatrixChangeToRevert<T: ValueType> {
    fn register_element_value_to_restore(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        element_value: T,
    );

    fn register_empty_element_to_restore(&mut self, row_index: RowIndex, column_index: ColumnIndex);

    fn register_sparse_matrix_state_to_restore(&mut self, sparse_matrix: SparseMatrix<T>);

    fn register_size_to_restore(&mut self, size: Size);
}

impl<T> RegisterSparseMatrixChangeToRevert<T> for SparseMatrixStateReverter<T>
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
                .push(SparseMatrixStateToRestore::ElementValue(
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
                .push(SparseMatrixStateToRestore::EmptyElement(
                    row_index,
                    column_index,
                ))
        }
    }

    fn register_sparse_matrix_state_to_restore(&mut self, sparse_matrix: SparseMatrix<T>) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(SparseMatrixStateToRestore::SparseMatrix(sparse_matrix));
            self.is_state_to_restore_fully_determined = true;
        }
    }

    fn register_size_to_restore(&mut self, size: Size) {
        if self.size_to_restore.is_none() {
            self.size_to_restore = Some(size);
        }
    }
}

// pub(crate) trait CreateSparseMatrixStateReverter<T: ValueType> {
//     fn sparse_matrix_state_reverter_with_size_to_restore(
//         size_to_restore: Option<Size>,
//     ) -> SparseMatrixStateReverter<T>;
// }

// macro_rules! implement_create_sparse_matrix_state_reverter {
//     ($value_type:ty) => {
//         impl CreateSparseMatrixStateReverter<$value_type> for $value_type {
//             fn sparse_matrix_state_reverter_with_size_to_restore(
//                 size_to_restore: Option<Size>,
//             ) -> SparseMatrixStateReverter<$value_type> {
//                 SparseMatrixStateReverter::<$value_type>::with_size_to_restore(size_to_restore)
//             }
//         }
//     };
// }
// implement_macro_for_all_native_value_types!(implement_create_sparse_matrix_state_reverter);

impl<T: ValueType> SparseMatrixStateReverter<T> {
    pub(crate) fn new(
        size_to_restore: Option<Size>,
        state_to_restore: Vec<SparseMatrixStateToRestore<T>>,
        is_state_to_restore_fully_determined: bool,
    ) -> Self {
        Self {
            size_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        }
    }

    pub(crate) fn new_default() -> Self {
        Self {
            size_to_restore: None,
            state_to_restore: Vec::new(),
            is_state_to_restore_fully_determined: false,
        }
    }

    pub(crate) fn with_size_to_restore(size_to_restore: Option<Size>) -> Self {
        Self::new(size_to_restore, Vec::new(), false)
    }

    // pub(crate) fn with_size_to_restore_from_sparse_matrix(
    //     to_restore: &SparseMatrix<T>,
    // ) -> Result<Self, GraphComputingError> {
    //     let size_to_restore = to_restore.size()?;
    //     Ok(Self::with_size_to_restore(size_to_restore))
    // }

    // pub(crate) fn from_sparse_matrix(
    //     to_restore: &SparseMatrix<T>,
    // ) -> Result<Self, GraphComputingError> {
    //     let state_to_restore = vec![SparseMatrixStateToRestore::SparseMatrix(
    //         to_restore.to_owned(),
    //     )];
    //     Ok(Self {
    //         size_to_restore: to_restore.size()?,
    //         state_to_restore: state_to_restore,
    //         is_state_to_restore_fully_determined: true,
    //     })
    // }
}

impl<T: ValueType + SetSparseMatrixElementTyped<T>> RestoreState<SparseMatrix<T>>
    for SparseMatrixStateReverter<T>
{
    fn restore(self, instance_to_restore: &mut SparseMatrix<T>) -> Result<(), GraphComputingError> {
        restore_sparse_matrix_state(self, instance_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_size_to_restore(self.size_to_restore)
    }
}

pub(crate) fn restore_sparse_matrix_state<T: ValueType + SetSparseMatrixElementTyped<T>>(
    state_reverter: SparseMatrixStateReverter<T>,
    instance_to_restore: &mut impl GetGraphblasSparseMatrix,
) -> Result<(), GraphComputingError> {
    for state_to_restore in state_reverter.state_to_restore.into_iter().rev() {
        match state_to_restore {
            SparseMatrixStateToRestore::EmptyElement(column_index, row_index) => {
                drop_sparse_matrix_element(instance_to_restore, row_index, column_index)?
            }
            SparseMatrixStateToRestore::ElementValue(column_index, row_index, element_value) => {
                T::set_graphblas_matrix_value(
                    instance_to_restore,
                    row_index,
                    column_index,
                    element_value,
                )?
            }
            SparseMatrixStateToRestore::SparseMatrix(mut sparse_matrix) => {
                unsafe {
                    mem::swap(
                        instance_to_restore.graphblas_matrix_mut_ref(),
                        sparse_matrix.graphblas_matrix_mut_ref(),
                    )
                };
            }
        }
    }

    match state_reverter.size_to_restore {
        Some(size_to_restore) => {
            resize_sparse_matrix(instance_to_restore, size_to_restore)?;
        }
        None => (),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
        DropSparseMatrixElement, GetSparseMatrixElementValue, GetSparseMatrixSize, ResizeSparseMatrix, SetSparseMatrixElement
    };
    use graphblas_sparse_linear_algebra::collections::Collection;
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use super::*;

    #[test]
    fn restore_sparse_matrix() {
        let context = GraphBLASContext::init_default().unwrap();
        let mut matrix = SparseMatrix::<u16>::new(context, (10, 10).into()).unwrap();

        matrix.set_value(1, 1, 1).unwrap();
        matrix.set_value(4, 4, 4).unwrap();
        matrix.set_value(5, 5, 5).unwrap();

        let mut state_reverter =
            SparseMatrixStateReverter::new(Some(matrix.size().unwrap()), Vec::new(), false);

        matrix.drop_element(1, 1).unwrap();
        state_reverter.register_element_value_to_restore(1, 1, 1);

        matrix.set_value(0, 0, 0);
        state_reverter.register_empty_element_to_restore(0, 0);

        matrix.set_value(0, 0, 10);
        state_reverter.register_element_value_to_restore(0, 0, 10);

        matrix.set_value(4, 4, 40).unwrap();
        state_reverter.register_element_value_to_restore(4, 4, 4);

        state_reverter.register_sparse_matrix_state_to_restore(matrix.clone());
        matrix.resize((4, 4).into()).unwrap();

        matrix.drop_element(5, 5);
        state_reverter.register_element_value_to_restore(5, 5, 5);

        restore_sparse_matrix_state(state_reverter, &mut matrix).unwrap();

        assert_eq!(matrix.size().unwrap(), (10, 10).into());
        assert_eq!(matrix.element_value(0, 0).unwrap(), None);
        assert_eq!(matrix.element_value(1, 1).unwrap(), Some(1));
        assert_eq!(matrix.element_value(4, 4).unwrap(), Some(4));
        assert_eq!(matrix.number_of_stored_elements().unwrap(), 3);
    }
}
