use std::collections::VecDeque;

use crate::{
    error::GraphComputingError,
    graph::{indexing::ElementCount, value_type::ValueType},
    operators::transaction::RestoreState,
};

pub(crate) struct QueueStateReverter<T: ValueType> {
    front_to_restore: VecDeque<T>,
    length_to_restore: ElementCount,
    capacity_to_restore: ElementCount,
}

pub(crate) trait RegisterQueueChangeToRevert<T: ValueType> {
    fn front_popped_value_to_restore(&mut self, popped_value: T);
}

impl<T: ValueType> RestoreState<VecDeque<T>> for QueueStateReverter<T> {
    fn restore(mut self, instance_to_restore: &mut VecDeque<T>) -> Result<(), GraphComputingError> {
        self.front_to_restore.append(instance_to_restore);
        self.front_to_restore.truncate(self.length_to_restore);
        self.front_to_restore.shrink_to(self.capacity_to_restore);
        *instance_to_restore = self.front_to_restore;
        Ok(())
    }
}

impl<T: ValueType> RegisterQueueChangeToRevert<T> for QueueStateReverter<T> {
    fn front_popped_value_to_restore(&mut self, popped_value: T) {
        self.front_to_restore.push_back(popped_value)
    }
}

impl<T: ValueType> QueueStateReverter<T> {
    pub(crate) fn new(length_to_restore: ElementCount, capacity_to_restore: ElementCount) -> Self {
        Self {
            front_to_restore: VecDeque::new(),
            length_to_restore,
            capacity_to_restore,
        }
    }

    pub(crate) fn with_length_and_capacity_from(to_restore: &VecDeque<T>) -> Self {
        QueueStateReverter::new(to_restore.len(), to_restore.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_vec_deque() {
        let mut vec_deque = VecDeque::new();

        for value in 0..10 {
            vec_deque.push_back(value)
        }

        let vec_deque_before_changes = vec_deque.clone();

        let mut state_reverter = QueueStateReverter::with_length_and_capacity_from(&vec_deque);

        for index in 0..3 {
            state_reverter.front_popped_value_to_restore(vec_deque.pop_front().unwrap())
        }

        for value in 0..5 {
            vec_deque.push_back(value)
        }

        state_reverter.restore(&mut vec_deque).unwrap();

        assert_eq!(vec_deque, vec_deque_before_changes);
    }
}
