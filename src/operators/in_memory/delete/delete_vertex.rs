use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::map::MapMutableAdjacencyMatrices,
            weighted_adjacency_matrix::operations::DeleteVertexConnections,
        },
        graph::{GetEdgeStore, GetVertexStore},
        indexing::{GetVertexIndexIndex, GetVertexTypeIndex},
        vertex_store::{
            DeleteVertexForAllTypes, DeleteVertexValue as DeleteVertexValueFromVertexStore,
        },
    },
};

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::Graph;
use crate::operators::operators::delete::DropVertexIndex;
use crate::operators::operators::delete::DropPrivateVertexIndex;
use crate::operators::operators::delete::DeleteVertexValue;
use crate::operators::operators::delete::DeletePrivateVertexValue;

impl DropVertexIndex for Graph {
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        // TODO: Consider restricting to valid indices for improved performance
        self.edge_store_mut_ref().map_mut_all_adjacency_matrices(
            |adjacency_matrix: &mut WeightedAdjacencyMatrix| {
                adjacency_matrix.delete_vertex_connections_unchecked(vertex_index)
            },
        )?;

        self.vertex_store_mut_ref()
            .delete_vertex_for_all_valid_public_vertex_types_and_value_types(vertex_index)
    }
}

impl DropPrivateVertexIndex for Graph {
    fn drop_private_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &(impl GetVertexIndexIndex + Sync),
    ) -> Result<(), GraphComputingError> {
        // TODO: Consider restricting to valid indices for improved performance
        self.edge_store_mut_ref()
            .map_mut_all_valid_private_adjacency_matrices(
                |adjacency_matrix: &mut WeightedAdjacencyMatrix| {
                    adjacency_matrix.delete_vertex_connections_unchecked(vertex_index)
                },
            )?;

        self.vertex_store_mut_ref()
            .delete_vertex_for_all_valid_private_vertex_types_and_value_types(vertex_index)
    }
}

impl DeleteVertexValue for Graph {
    fn delete_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_element_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        DeleteVertexValueFromVertexStore::delete_public_vertex_element(
            self.vertex_store_mut_ref(),
            vertex_type_index,
            vertex_element_index,
        )
    }
}

impl DeletePrivateVertexValue for Graph {
    fn delete_private_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_element_index: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        DeleteVertexValueFromVertexStore::delete_private_vertex_element(
            self.vertex_store_mut_ref(),
            vertex_type_index,
            vertex_element_index,
        )
    }
}

#[cfg(test)]
mod tests {

    // use crate::graph::edge::DirectedEdgeDefinedByKeys;
    // use crate::graph::vertex::Vertex;
    // use crate::operations::add_edge::AddEdge;
    // use crate::operations::add_vertex::AddVertex;
    // use crate::operations::read_edge::ReadEdge;
    // use crate::operations::read_vertex_value::ReadVertexValue;

    // #[test]
    // fn delete_vertex_and_connected_edges() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key_1 = String::from("vertex_1");
    //     let vertex_key_2 = String::from("vertex_2");

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

    //     graph
    //         .delete_vertex_and_connected_edges_by_key(vertex_key_1.clone())
    //         .unwrap();

    //     assert!(!graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //         .unwrap());
    //     assert!(!graph
    //         .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //         .unwrap());
    //     assert!(!graph
    //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //         .unwrap());

    //     assert!(!graph.is_valid_vertex_key(&vertex_key_1));
    //     assert!(graph.is_valid_vertex_key(&vertex_key_2));
    // }
}
