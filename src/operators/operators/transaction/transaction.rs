use crate::error::GraphComputingError;
use crate::graph::graph::Graph;

pub trait UseAtomicTransaction {
    fn revert(self) -> Result<(), GraphComputingError>;
    fn commit(self) -> Result<(), GraphComputingError>;
}

pub(crate) trait GetGraph {
    fn graph_ref(&self) -> &Graph;
    fn graph_mut_ref(&mut self) -> &mut Graph;
}
