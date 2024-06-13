use crate::error::GraphComputingError;
use crate::graph::graph::Graph;
use crate::operators::transaction::{GetGraph, UseAtomicTransaction};

pub struct AtomicInMemoryTransaction<'t> {
    graph: &'t mut Graph,
}

impl<'t> UseAtomicTransaction for AtomicInMemoryTransaction<'t> {
    fn revert(&mut self) -> Result<(), GraphComputingError> {
        self.revert_private()
    }

    fn commit(&mut self) -> Result<(), GraphComputingError> {
        todo!()
    }
}

impl<'t> AtomicInMemoryTransaction<'t> {
    pub fn new(graph: &'t mut Graph) -> Self {
        Self { graph }
    }

    fn revert_private(&mut self) -> Result<(), GraphComputingError> {
        todo!()
    }
}

impl<'t> GetGraph for AtomicInMemoryTransaction<'t> {
    fn graph_ref(&self) -> &Graph {
        &self.graph
    }

    fn graph_mut_ref(&mut self) -> &mut Graph {
        &mut self.graph
    }
}

impl<'t> Drop for AtomicInMemoryTransaction<'t> {
    fn drop(&mut self) {
        self.revert_private();
    }
}
