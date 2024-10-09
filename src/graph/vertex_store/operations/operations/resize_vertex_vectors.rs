use crate::{
    error::GraphComputingError,
    graph::{
        indexing::ElementCount,
        vertex_store::{ResizeWeightedAdjacencyMatrix, VertexStore, VertexVector},
    },
};

use super::map::MapAllVertexVectors;

pub(crate) trait ResizeVertexVectors {
    fn resize_vertex_vectors(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}
