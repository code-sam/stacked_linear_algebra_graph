use crate::error::GraphComputingError;

use crate::graph::edge_store::operations::operations::edge_type::delete_edge_type::DropEdgeType as DropEdgeTypeFromEdgeStore;
use crate::graph::graph::{GetEdgeStore, Graph};
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::operators::operators::drop::DropEdgeType;

impl DropEdgeType for Graph {
    fn drop_edge_type(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_mut_ref().drop_edge_type(edge_type_index)
    }
}

#[cfg(test)]
mod tests {

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
