use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;

use crate::error::GraphComputingError;

use crate::graph::edge::{EdgeTypeIndex, EdgeTypeKeyRef};
use crate::graph::edge_store::weighted_adjacency_matrix::{
    WeightedAdjacencyMatrix, WeightedAdjacencyMatrixTrait,
};
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait, VertexTypeIndex};
use crate::graph::indexer::IndexerTrait;
use crate::graph::vertex::VertexTypeKeyRef;
use crate::graph::vertex_store::VertexStoreTrait;

pub trait DropVertexType {
    /// Deletes the vertex type, and all its edges
    fn drop_vertex_type_with_key(
        &mut self,
        vertex_type: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn drop_vertex_type_with_index(
        &mut self,
        vertex_type: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DropVertexType for Graph {
    fn drop_vertex_type_with_key(
        &mut self,
        edge_type: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .vertex_type_indexer_mut_ref()
            .free_key(edge_type)
    }

    fn drop_vertex_type_with_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .vertex_type_indexer_mut_ref()
            .free_index(*vertex_type_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn drop_edge_type() {
    //     let mut graph = Graph::new(5, 5).unwrap();

    //     let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
    //     let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

    //     let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
    //         vertex_1.clone().into(),
    //         String::from("edge_type_1"),
    //         vertex_2.clone().into(),
    //     );
    //     let edge_vertex2_vertex1 = DirectedEdgeDefinedByKeys::new(
    //         vertex_2.clone().into(),
    //         String::from("edge_type_1"),
    //         vertex_1.clone().into(),
    //     );
    //     let edge_vertex1_vertex2_type2 = DirectedEdgeDefinedByKeys::new(
    //         vertex_1.clone().into(),
    //         String::from("edge_type_2"),
    //         vertex_2.clone().into(),
    //     );

    //     graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
    //     graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

    //     graph
    //         .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
    //         .unwrap();
    //     graph
    //         .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
    //         .unwrap();
    //     graph
    //         .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
    //         .unwrap();

    //     assert!(graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //         .unwrap());
    //     assert!(graph
    //         .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //         .unwrap());
    //     assert!(graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //         .unwrap());

    //     graph
    //         .drop_edge_type_with_key(edge_vertex1_vertex2.edge_type_ref())
    //         .unwrap();
    //     assert!(!graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //         .unwrap());
    //     assert!(!graph
    //         .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //         .unwrap());
    //     assert!(graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //         .unwrap());
    // }
}
