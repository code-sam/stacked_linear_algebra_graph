use std::mem;

use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    drop_sparse_vector_element, resize_sparse_vector, GetSparseVectorElementValueTyped, GetSparseVectorLength, SetSparseVectorElementTyped
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    GetGraphblasSparseVector, SparseVector,
};

use crate::error::GraphComputingError;
use crate::graph::indexing::{ElementCount, ElementIndex};
use crate::graph::value_type::ValueType;
use crate::operators::transaction::RestoreState;

#[derive(Debug, Clone)]
pub(crate) struct SparseVectorStateReverter<T: ValueType> {
    length_to_restore: Option<ElementCount>,
    state_to_restore: Vec<SparseVectorStateToRestore<T>>,
    is_state_to_restore_fully_determined: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum SparseVectorStateToRestore<T: ValueType> {
    EmptyElement(ElementIndex),
    ElementValue(ElementIndex, T),
    SparseVector(SparseVector<T>),
}

pub(crate) trait GetSparseVectorStateToRestore<T: ValueType> {
    fn length_to_restore(&self) -> Option<ElementCount>;
    // fn state_to_restore(&self) -> Vec<SparseVectorStateToRestore<T>>;
    fn state_to_restore_ref(&self) -> &[SparseVectorStateToRestore<T>];
    fn state_to_restore_mut_ref(&mut self) -> &mut [SparseVectorStateToRestore<T>];
    fn is_state_to_restore_fully_determined(&self) -> bool;
}

impl<T: ValueType> GetSparseVectorStateToRestore<T> for SparseVectorStateReverter<T> {
    fn length_to_restore(&self) -> Option<ElementCount> {
        self.length_to_restore
    }

    fn state_to_restore_ref(&self) -> &[SparseVectorStateToRestore<T>] {
        self.state_to_restore.as_slice()
    }

    fn state_to_restore_mut_ref(&mut self) -> &mut [SparseVectorStateToRestore<T>] {
        self.state_to_restore.as_mut_slice()
    }

    fn is_state_to_restore_fully_determined(&self) -> bool {
        self.is_state_to_restore_fully_determined
    }
}

pub(crate) trait RegisterSparseVectorChangeToRevert<T: ValueType> {
    fn register_element_value_to_restore(&mut self, element_index: ElementIndex, element_value: T);

    fn register_empty_element_to_restore(&mut self, element_index: ElementIndex);

    fn register_sparse_vector_state_to_restore(&mut self, sparse_vector: SparseVector<T>);

    fn register_length_to_restore(&mut self, length: ElementCount);
}

impl<T: ValueType + SetSparseVectorElementTyped<T>> RestoreState<SparseVector<T>>
    for SparseVectorStateReverter<T>
{
    fn restore(self, instance_to_restore: &mut SparseVector<T>) -> Result<(), GraphComputingError> {
        restore_sparse_vector_state(self, instance_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_length_to_restore(self.length_to_restore)
    }
}

pub(crate) fn restore_sparse_vector_state<T>(
    sparse_vector_state_restorer: SparseVectorStateReverter<T>,
    instance_to_restore: &mut impl GetGraphblasSparseVector,
) -> Result<(), GraphComputingError>
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    for state_to_restore in sparse_vector_state_restorer
        .state_to_restore
        .into_iter()
        .rev()
    {
        match state_to_restore {
            SparseVectorStateToRestore::EmptyElement(element_index) => {
                drop_sparse_vector_element(instance_to_restore, element_index)?
            }
            SparseVectorStateToRestore::ElementValue(element_index, element_value) => {
                T::set_graphblas_vector_value(instance_to_restore, element_index, element_value)?
            }
            SparseVectorStateToRestore::SparseVector(mut sparse_vector) => {
                unsafe {
                    mem::swap(
                        instance_to_restore.graphblas_vector_mut_ref(),
                        sparse_vector.graphblas_vector_mut_ref(),
                    )
                };
            }
        }
    }

    match sparse_vector_state_restorer.length_to_restore {
        Some(length_to_restore) => {
            resize_sparse_vector(instance_to_restore, length_to_restore)?;
        }
        None => (),
    }

    Ok(())
}

impl<
        T: ValueType + GetSparseVectorElementValueTyped<T> + SetSparseVectorElementTyped<T> + Default,
    > RegisterSparseVectorChangeToRevert<T> for SparseVectorStateReverter<T>
{
    fn register_element_value_to_restore(&mut self, element_index: ElementIndex, element_value: T) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(SparseVectorStateToRestore::ElementValue(
                    element_index,
                    element_value,
                ))
        }
    }

    fn register_empty_element_to_restore(&mut self, element_index: ElementIndex) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(SparseVectorStateToRestore::EmptyElement(element_index))
        }
    }

    fn register_sparse_vector_state_to_restore(&mut self, sparse_vector: SparseVector<T>) {
        if !self.is_state_to_restore_fully_determined {
            self.state_to_restore
                .push(SparseVectorStateToRestore::SparseVector(sparse_vector));
            self.is_state_to_restore_fully_determined = true;
        }
    }

    fn register_length_to_restore(&mut self, length: ElementCount) {
        self.length_to_restore = Some(length)
    }
}

// pub(crate) trait CreateSparseVectorStateReverter<T: ValueType> {
//     fn sparse_vector_state_reverter_with_length_to_restore(
//         length_to_restore: ElementCount,
//     ) -> SparseVectorStateReverter<T>;
// }

// macro_rules! implement_create_sparse_vector_state_reverter {
//     ($value_type:ty) => {
//         impl CreateSparseVectorStateReverter<$value_type> for $value_type {
//             fn sparse_vector_state_reverter_with_length_to_restore(
//                 length_to_restore: ElementCount,
//             ) -> SparseVectorStateReverter<$value_type> {
//                 SparseVectorStateReverter::<$value_type>::with_length_to_restore(length_to_restore)
//             }
//         }
//     };
// }
// implement_macro_for_all_native_value_types!(implement_create_sparse_vector_state_reverter);

impl<T: ValueType> SparseVectorStateReverter<T> {
    pub(crate) fn new(
        length_to_restore: Option<ElementCount>,
        state_to_restore: Vec<SparseVectorStateToRestore<T>>,
        is_state_to_restore_fully_determined: bool,
    ) -> Self {
        Self {
            length_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        }
    }

    pub(crate) fn new_default() -> Self {
        Self {
            length_to_restore: None,
            state_to_restore: Vec::new(),
            is_state_to_restore_fully_determined: false,
        }
    }

    pub(crate) fn with_length_to_restore(length_to_restore: Option<ElementCount>) -> Self {
        let state_to_restore = Vec::new(); // TO REVIEW: initial capacity for optimal size
        let is_state_to_restore_fully_determined = false;

        SparseVectorStateReverter::new(
            length_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        )
    }

    pub(crate) fn with_dimensions_from_sparse_vector(
        to_restore: &SparseVector<T>,
    ) -> Result<Self, GraphComputingError> {
        let length_to_restore = Some(to_restore.length()?);
        let state_to_restore = Vec::new(); // TO REVIEW: initial capacity for optimal size
        let is_state_to_restore_fully_determined = false;

        Ok(SparseVectorStateReverter::new(
            length_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        ))
    }

    // pub(crate) fn from_sparse_vector(
    //     to_restore: &SparseVector<T>,
    // ) -> Result<Self, GraphComputingError> {
    //     let state_to_restore = vec![SparseVectorStateToRestore::SparseVector(
    //         to_restore.to_owned(),
    //     )];
    //     Ok(Self {
    //         length_to_restore: to_restore.length()?,
    //         state_to_restore: state_to_restore,
    //         is_state_to_restore_fully_determined: true,
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
        DeleteSparseVectorElement, GetSparseVectorElementValue, GetSparseVectorLength, ResizeSparseVector, SetSparseVectorElement
    };
    use graphblas_sparse_linear_algebra::collections::Collection;
    use graphblas_sparse_linear_algebra::context::Context as GraphBLASContext;

    use super::*;

    #[test]
    fn restore_sparse_vector() {
        let context = GraphBLASContext::init_default().unwrap();
        let mut vector = SparseVector::<u16>::new(context, 10).unwrap();

        vector.set_value(1, 1).unwrap();
        vector.set_value(4, 4).unwrap();
        vector.set_value(5, 5).unwrap();

        let mut state_reverter =
            SparseVectorStateReverter::new(Some(vector.length().unwrap()), Vec::new(), false);

        vector.drop_element(1).unwrap();
        state_reverter.register_element_value_to_restore(1, 1);

        vector.set_value(0, 0);
        state_reverter.register_empty_element_to_restore(0);

        vector.set_value(0, 10);
        state_reverter.register_element_value_to_restore(0, 10);

        vector.set_value(4, 40).unwrap();
        state_reverter.register_element_value_to_restore(4, 4);

        state_reverter.register_sparse_vector_state_to_restore(vector.clone());
        vector.resize(4).unwrap();

        vector.drop_element(5);
        state_reverter.register_element_value_to_restore(5, 5);

        state_reverter.restore(&mut vector).unwrap();

        assert_eq!(vector.length().unwrap(), 10);
        assert_eq!(vector.element_value(0).unwrap(), None);
        assert_eq!(vector.element_value(1).unwrap(), Some(1));
        assert_eq!(vector.element_value(4).unwrap(), Some(4));
        assert_eq!(vector.number_of_stored_elements().unwrap(), 3);
    }
}
