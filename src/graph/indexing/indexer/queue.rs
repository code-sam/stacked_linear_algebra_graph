use std::collections::VecDeque;

use crate::graph::indexing::ElementCount;

pub(crate) trait Queue<T> {
    fn push_back(&mut self, value: T);
    fn pop_front(&mut self) -> Option<T>;
    fn len(&self) -> ElementCount;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VecDequeQueue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> for VecDequeQueue<T> {
    fn push_back(&mut self, value: T) {
        self.push_back(value)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn len(&self) -> ElementCount {
        self.len()
    }
}

impl<T> VecDequeQueue<T> {
    pub(crate) fn new() -> Self {
        VecDequeQueue {
            queue: VecDeque::new(),
        }
    }
}
