use graphblas_sparse_linear_algebra::{
    collections::sparse_vector::SparseVector,
    context::{Context as GraphblasContext, Mode as GraphblasMode},
    index::ElementIndex as GraphblasElementIndex,
};

use super::graph::Graph;

use crate::error::{GraphComputingError, LogicError, LogicErrorType, UserError, UserErrorType};
// use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
// use crate::graph::edge::{EdgeType, EdgeTypeIndex, EdgeTypeRef};
use crate::graph::index::ElementCount;
// use crate::graph::indexed_data_store::data_store::IndexedDataStore;
// use crate::graph::graph::indexed_vertex_and_adjacency_matrix_store::indexed_vertex_and_adjacency_matrix_store::{
//     IndexedVertexAndAdjacencyMatrixStore, IndexedVertexAndAdjacencyMatrixStoreTrait,
// };
use crate::graph::vertex::{Vertex, VertexIndex, VertexKey};
// use crate::operations::{add_edge_type::AddEdgeType, drop_edge_type::DropEdgeType};

pub trait GraphMonitoring {
    fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError>;
    fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError>;
    // TODO: number of edges
    // TODO: number of edges per edge type, etc
}

// impl GraphMonitoring for Graph {
//     fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError> {
//         let number_of_vertices = self
//             .index_mask_with_all_vertices()
//             .number_of_stored_elements()?;
//         Ok(number_of_vertices)
//     }

//     fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError> {
//         let number_of_edge_types = self
//             .index_mask_with_all_adjacency_matrices()
//             .number_of_stored_elements()?;
//         Ok(number_of_edge_types)
//     }

//     fn vertex_capacity(&self) -> Result<ElementCount, GraphComputingError> {
//         Ok(self.vertex_store.get_capacity()?)
//     }

//     // TODO: number of edges
//     // TODO: number of edges for edge type
// }
