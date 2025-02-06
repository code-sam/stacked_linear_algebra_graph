use crate::{
    error::GraphComputingError,
    graph::indexing::{ElementCount, GetEdgeTypeIndex},
};

pub(crate) trait ResizeWeightedAdjacencyMatrix {
    fn resize(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}
