use crate::{error::GraphComputingError, graph::indexing::ElementCount};

pub(crate) trait ResizeWeightedAdjacencyMatrix {
    fn resize(&mut self, new_vertex_capacity: ElementCount) -> Result<(), GraphComputingError>;
}
