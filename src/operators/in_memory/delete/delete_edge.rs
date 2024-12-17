use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::DeleteEdge as DeleteEdgeInAdjacencyMatrix;
use crate::graph::graph::GetEdgeStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::operators::operators::delete::DeleteEdge;
use crate::operators::operators::delete::DeletePrivateEdge;

impl DeleteEdge for Graph {
    fn delete_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_mut_ref()
            .public_adjacency_matrix_mut_ref(edge_type)?
            .delete_edge_weight_unchecked(tail, head)?;
        Ok(())
    }

    fn delete_edge_for_coordinate(
        &mut self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError> {
        self.delete_edge(edge.edge_type_ref(), edge.tail_ref(), edge.head_ref())
    }
}

impl DeletePrivateEdge for Graph {
    fn delete_private_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_mut_ref()
            .private_adjacency_matrix_mut_ref(edge_type)?
            .delete_edge_weight_unchecked(tail, head)?;
        Ok(())
    }

    fn delete_private_edge_for_coordinate(
        &mut self,
        edge: &impl GetDirectedEdgeCoordinateIndex,
    ) -> Result<(), GraphComputingError> {
        self.delete_private_edge(edge.edge_type_ref(), edge.tail_ref(), edge.head_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::operators::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::operators::read::GetEdgeWeight;

    #[test]
    fn delete_edge() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = AddVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .add_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = AddEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = AddEdgeType::<u16>::apply(&mut graph).unwrap();

        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .add_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        assert_eq!(
            GetEdgeWeight::<u8>::edge_weight(
                &graph,
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index
            )
            .unwrap()
            .unwrap(),
            // graph
            //     .key_defined_edge_weight(&edge_vertex1_vertex2.coordinate_ref())
            //     .unwrap().unwrap(),
            1u8
        );
        assert!(match GetEdgeWeight::<u8>::edge_weight(
            &graph,
            &edge_type_1_index,
            &vertex_2_index,
            &vertex_1_index
        )
        .unwrap()
        {
            None => false,
            Some(_) => true,
        });
        assert!(match GetEdgeWeight::<u8>::edge_weight(
            &graph,
            &edge_type_2_index,
            &vertex_1_index,
            &vertex_2_index
        )
        .unwrap()
        {
            None => false,
            Some(_) => true,
        });

        DeleteEdge::delete_edge(
            &mut graph,
            &edge_type_1_index,
            &vertex_1_index,
            &vertex_2_index,
        )
        .unwrap();
        // graph
        //     .delete_edge_defined_by_keys(edge_vertex1_vertex2.coordinate_ref())
        //     .unwrap();
        assert_eq!(
            GetEdgeWeight::<u8>::edge_weight(
                &graph,
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index
            )
            .unwrap()
            .unwrap(),
            3u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::edge_weight(
                &graph,
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert!(match GetEdgeWeight::<u8>::edge_weight(
            &graph,
            &edge_type_1_index,
            &vertex_1_index,
            &vertex_2_index
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });

        DeleteEdge::delete_edge(
            &mut graph,
            &edge_type_2_index,
            &vertex_1_index,
            &vertex_2_index,
        )
        .unwrap();
        assert_eq!(
            GetEdgeWeight::<u8>::edge_weight(
                &graph,
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert!(match GetEdgeWeight::<u8>::edge_weight(
            &graph,
            &edge_type_1_index,
            &vertex_1_index,
            &vertex_2_index
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });
        assert!(match GetEdgeWeight::<u8>::edge_weight(
            &graph,
            &edge_type_2_index,
            &vertex_1_index,
            &vertex_2_index
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });
    }

    // #[test]
    // fn delete_non_existing_edge() {
    //     let mut graph = Graph::new(5, 5).unwrap();

    //     let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
    //     let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

    //     let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
    //         vertex_1.clone().into(),
    //         String::from("edge_type_1"),
    //         vertex_2.clone().into(),
    //     );

    //     let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //     match result {
    //         Err(_) => assert!(true),
    //         _ => assert!(false),
    //     }

    //     graph.add_or_replace_vertex(vertex_1.clone()).unwrap();

    //     let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //     match result {
    //         Err(_) => assert!(true),
    //         _ => assert!(false),
    //     }

    //     // Deleting non-existing edge, connecting existing edges
    //     let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //     match result {
    //         Err(_) => assert!(true),
    //         _ => assert!(false),
    //     }
    // }
}
