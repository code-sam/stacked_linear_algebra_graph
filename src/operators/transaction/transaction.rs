use crate::error::GraphComputingError;
use crate::graph::graph::Graph;

pub trait UseAtomicTransaction: Drop {
    fn revert(&mut self) -> Result<(), GraphComputingError>;
    fn commit(&mut self) -> Result<(), GraphComputingError>;
}

pub(crate) trait GetGraph {
    fn graph_ref(&self) -> &Graph;
    fn graph_mut_ref(&mut self) -> &mut Graph;
}

pub(crate) trait RestoreState<T> {
    fn restore(self, instance_to_restore: &mut T) -> Result<(), GraphComputingError>;
    fn with_reset_state_to_restore(&self) -> Self;
}
