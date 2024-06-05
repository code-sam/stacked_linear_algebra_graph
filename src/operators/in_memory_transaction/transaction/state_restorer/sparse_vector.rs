use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    operations::{
        DeleteSparseVectorElement, GetSparseVectorLength, GetVectorElementValueTyped,
        ResizeSparseVector, SetVectorElement, SetVectorElementTyped,
    },
    SparseVector,
};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{ElementCount, ElementIndex},
        value_type::ValueType,
    },
    operators::transaction::RestoreState,
};

enum SparseVectorStateToRestore<T: ValueType> {
    EmptyElement(ElementIndex),
    ElementValue(ElementIndex, T),
    SparseVector(SparseVector<T>),
}

pub(crate) struct SparseVectorStateReverter<T: ValueType> {
    length_to_restore: ElementCount,
    state_to_restore: Vec<SparseVectorStateToRestore<T>>,
    is_state_to_restore_fully_determined: bool,
}

pub(crate) trait RegisterSparseVectorChangeToRevert<T: ValueType> {
    fn register_element_value_to_restore(&mut self, element_index: ElementIndex, element_value: T);

    fn register_empty_element_to_restore(&mut self, element_index: ElementIndex);

    fn register_sparse_vector_state_to_restore(&mut self, sparse_vector: SparseVector<T>);
}

impl<T: ValueType + SetVectorElementTyped<T>> RestoreState<SparseVector<T>>
    for SparseVectorStateReverter<T>
{
    fn restore(
        mut self,
        instance_to_restore: &mut SparseVector<T>,
    ) -> Result<(), GraphComputingError> {
        for state_to_restore in self.state_to_restore.into_iter().rev() {
            match state_to_restore {
                SparseVectorStateToRestore::EmptyElement(element_index) => {
                    instance_to_restore.drop_element(element_index)?
                }
                SparseVectorStateToRestore::ElementValue(element_index, element_value) => {
                    instance_to_restore.set_value(&element_index, element_value)?
                }
                SparseVectorStateToRestore::SparseVector(sparse_vector) => {
                    *instance_to_restore = sparse_vector;
                }
            }
        }
        instance_to_restore.resize(self.length_to_restore)?;
        Ok(())
    }
}

impl<T: ValueType + GetVectorElementValueTyped<T> + SetVectorElementTyped<T> + Default>
    RegisterSparseVectorChangeToRevert<T> for SparseVectorStateReverter<T>
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
}

impl<T: ValueType> SparseVectorStateReverter<T> {
    pub(crate) fn new(
        length_to_restore: ElementCount,
        state_to_restore: Vec<SparseVectorStateToRestore<T>>,
        is_state_to_restore_fully_determined: bool,
    ) -> Self {
        Self {
            length_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        }
    }

    pub(crate) fn with_dimensions_from_sparse_vector(
        to_restore: &SparseVector<T>,
    ) -> Result<Self, GraphComputingError> {
        let length_to_restore = to_restore.length()?;
        let state_to_restore = Vec::new(); // TO REVIEW: initial capacity for optimal size
        let is_state_to_restore_fully_determined = false;

        Ok(SparseVectorStateReverter::new(
            length_to_restore,
            state_to_restore,
            is_state_to_restore_fully_determined,
        ))
    }

    pub(crate) fn from_sparse_vector(
        to_restore: &SparseVector<T>,
    ) -> Result<Self, GraphComputingError> {
        let state_to_restore = vec![SparseVectorStateToRestore::SparseVector(
            to_restore.to_owned(),
        )];
        Ok(Self {
            length_to_restore: to_restore.length()?,
            state_to_restore: state_to_restore,
            is_state_to_restore_fully_determined: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::{
        collections::{sparse_vector::operations::GetVectorElementValue, Collection},
        context::Context as GraphBLASContext,
    };

    use super::*;

    #[test]
    fn restore_sparse_vector() {
        let context = GraphBLASContext::init_default().unwrap();
        let mut vector = SparseVector::<u16>::new(&context, &10).unwrap();

        vector.set_value(&1, 1).unwrap();
        vector.set_value(&4, 4).unwrap();
        vector.set_value(&5, 5).unwrap();

        let mut state_reverter =
            SparseVectorStateReverter::with_dimensions_from_sparse_vector(&vector).unwrap();

        vector.drop_element(1).unwrap();
        state_reverter.register_element_value_to_restore(1, 1);

        vector.set_value(&0, 0);
        state_reverter.register_empty_element_to_restore(0);

        vector.set_value(&0, 10);
        state_reverter.register_element_value_to_restore(0, 10);

        vector.set_value(&4, 40).unwrap();
        state_reverter.register_element_value_to_restore(4, 4);

        state_reverter.register_sparse_vector_state_to_restore(vector.clone());
        vector.resize(4).unwrap();

        vector.drop_element(5);
        state_reverter.register_element_value_to_restore(5, 5);

        state_reverter.restore(&mut vector).unwrap();

        assert_eq!(vector.length().unwrap(), 10);
        assert_eq!(vector.element_value(&0).unwrap(), None);
        assert_eq!(vector.element_value(&1).unwrap(), Some(1));
        assert_eq!(vector.element_value(&4).unwrap(), Some(4));
        assert_eq!(vector.number_of_stored_elements().unwrap(), 3);
    }
}
