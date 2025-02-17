use graphblas_sparse_linear_algebra::context::{
    Context as GraphblasContext, MatrixStorageFormat as GraphblasMatrixStorageFormat,
    Mode as GraphblasMode,
};

use crate::error::GraphComputingError;
use crate::graph::{graph::Graph, indexing::ElementCount};
use crate::versioned_graph::indexing::versioned_index_map::VersionedIndexMap;

pub(crate) type UniqueVertexTypeIndexMap = VersionedIndexMap;
pub(crate) type UniqueEdgeTypeIndexMap = VersionedIndexMap;
pub(crate) type UniqueVertexIndexIndexMap = VersionedIndexMap;

pub struct VersionedGraph {
    graph: Graph,
    private_graph: Graph,

    unique_vertex_type_index_map: UniqueVertexTypeIndexMap,
    unique_edge_type_index_map: UniqueEdgeTypeIndexMap,
    unique_vertex_index_index_map: UniqueVertexIndexIndexMap,
}

impl VersionedGraph {
    pub fn with_initial_capacity(
        initial_vertex_type_capacity: ElementCount,
        initial_edge_type_capacity: ElementCount,
        initial_vertex_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let graphblas_context = GraphblasContext::init(
            GraphblasMode::NonBlocking,
            GraphblasMatrixStorageFormat::ByColumn,
        )?;

        let graph = Graph::with_context_and_initial_capacity(
            graphblas_context.clone(),
            initial_vertex_type_capacity,
            initial_vertex_capacity,
            initial_edge_type_capacity,
        )?;
        let private_graph = Graph::with_context_and_initial_capacity(
            graphblas_context,
            initial_vertex_type_capacity,
            initial_vertex_capacity,
            initial_edge_type_capacity,
        )?;

        let unique_vertex_type_index_map =
            UniqueVertexTypeIndexMap::with_initial_capacity(initial_vertex_type_capacity);
        let unique_edge_type_index_map =
            UniqueEdgeTypeIndexMap::with_initial_capacity(initial_edge_type_capacity);
        let unique_vertex_index_index_map =
            UniqueVertexIndexIndexMap::with_initial_capacity(initial_vertex_capacity);

        Ok(Self {
            graph,
            private_graph,
            unique_vertex_type_index_map,
            unique_edge_type_index_map,
            unique_vertex_index_index_map,
        })
    }
}
