use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{ElementCount, Queue, VecDequeQueue},
        value_type::ValueType,
    },
    operators::transaction::RestoreState,
};

#[derive(Debug, Clone)]
pub(crate) struct QueueStateReverter<T: ValueType> {
    front_to_restore: VecDequeQueue<T>,
    length_to_restore: ElementCount,
    capacity_to_restore: ElementCount,
}

pub(crate) trait RegisterQueueChangeToRevert<T: ValueType> {
    fn front_popped_value_to_restore(&mut self, popped_value: T);
}

impl<T: ValueType> RestoreState<VecDequeQueue<T>> for QueueStateReverter<T> {
    fn restore(
        mut self,
        instance_to_restore: &mut VecDequeQueue<T>,
    ) -> Result<(), GraphComputingError> {
        if self.front_to_restore.length() > 0 {
            self.front_to_restore.append(instance_to_restore);
            self.front_to_restore
                .truncate_length(self.length_to_restore);
            self.front_to_restore
                .shrink_capacity_to_at_least(self.capacity_to_restore);
            *instance_to_restore = self.front_to_restore;
        }
        Ok(())
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_length_and_capacity_to_restore(self.length_to_restore, self.capacity_to_restore)
    }
}

impl<T: ValueType> RegisterQueueChangeToRevert<T> for QueueStateReverter<T> {
    fn front_popped_value_to_restore(&mut self, popped_value: T) {
        self.front_to_restore.push_back(popped_value)
    }
}

impl<T: ValueType> QueueStateReverter<T> {
    pub(crate) fn new(
        front_to_restore: VecDequeQueue<T>,
        length_to_restore: ElementCount,
        capacity_to_restore: ElementCount,
    ) -> Self {
        Self {
            front_to_restore,
            length_to_restore,
            capacity_to_restore,
        }
    }

    pub(crate) fn with_length_and_capacity_to_restore(
        length_to_restore: ElementCount,
        capacity_to_restore: ElementCount,
    ) -> Self {
        QueueStateReverter::new(VecDequeQueue::new(), length_to_restore, capacity_to_restore)
    }

    pub(crate) fn with_length_and_capacity_to_restore_from(to_restore: &VecDequeQueue<T>) -> Self {
        QueueStateReverter::with_length_and_capacity_to_restore(
            to_restore.length(),
            to_restore.capacity(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_queue() {
        let mut queue = VecDequeQueue::new();

        for value in 0..10 {
            queue.push_back(value)
        }

        let vec_deque_before_changes = queue.clone();

        let mut state_reverter =
            QueueStateReverter::with_length_and_capacity_to_restore_from(&queue);

        for index in 0..3 {
            state_reverter.front_popped_value_to_restore(queue.pop_front().unwrap())
        }

        for value in 0..5 {
            queue.push_back(value)
        }

        state_reverter.restore(&mut queue).unwrap();

        assert_eq!(queue, vec_deque_before_changes);
    }

    #[test]
    fn restore_unchanged_queue() {
        let mut queue = VecDequeQueue::new();

        for value in 0..10 {
            queue.push_back(value)
        }

        let vec_deque_before_changes = queue.clone();

        let state_reverter = QueueStateReverter::with_length_and_capacity_to_restore_from(&queue);

        state_reverter.restore(&mut queue).unwrap();

        assert_eq!(queue, vec_deque_before_changes);
    }
}
