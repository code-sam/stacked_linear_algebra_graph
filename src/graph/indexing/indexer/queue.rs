use std::collections::VecDeque;

use crate::graph::indexing::ElementCount;

pub(crate) trait Queue<T> {
    fn push_back(&mut self, value: T);
    fn pop_front(&mut self) -> Option<T>;

    fn length(&self) -> ElementCount;
    fn capacity(&self) -> ElementCount;

    fn append(&mut self, to_append: &mut Self);
    fn truncate_length(&mut self, length: ElementCount);
    fn shrink_capacity_to_at_least(&mut self, min_capacity: ElementCount);
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VecDequeQueue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> for VecDequeQueue<T> {
    fn push_back(&mut self, value: T) {
        self.queue.push_back(value)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn length(&self) -> ElementCount {
        self.queue.len()
    }

    fn capacity(&self) -> ElementCount {
        self.queue.capacity()
    }

    fn append(&mut self, to_append: &mut Self) {
        self.queue.append(&mut to_append.queue)
    }

    fn truncate_length(&mut self, length: ElementCount) {
        self.queue.truncate(length)
    }

    fn shrink_capacity_to_at_least(&mut self, min_capacity: ElementCount) {
        self.queue.shrink_to(min_capacity)
    }
}

impl<T> VecDequeQueue<T> {
    pub(crate) fn new() -> Self {
        VecDequeQueue {
            queue: VecDeque::new(),
        }
    }
}
