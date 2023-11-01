use crate::error::GraphComputingError;

use crate::graph::edge::{
    AdjacencyMatrixCoordinate, DirectedEdgeCoordinateDefinedByIndicesTrait,
    DirectedEdgeCoordinateDefinedByKeysTrait,
};
use crate::graph::edge::{
    DirectedEdgeCoordinateDefinedByIndices, DirectedEdgeCoordinateDefinedByKeys,
};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::DeleteEdge as DeleteEdgeInAdjacencyMatrix;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::VertexStoreTrait;

pub trait DeleteEdge<T: ValueType> {
    fn delete_edge_defined_by_keys(
        &mut self,
        edge_to_delete: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<(), GraphComputingError>;
    fn delete_edge_defined_by_indices(
        &mut self,
        edge_to_delete: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<(), GraphComputingError>;
    // fn delete_selected_edges(&mut self, edge_selection_to_delete: &EdgeSelection) -> Result<(), GraphComputingError>;
}

impl<T> DeleteEdge<T> for Graph
where
    T: ValueType,
{
    fn delete_edge_defined_by_keys(
        &mut self,
        edge_to_delete: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<(), GraphComputingError> {
        let edge_type_index = *self
            .edge_store_ref()
            .edge_type_indexer_ref()
            .try_index_for_key(edge_to_delete.edge_type_ref())?;
        let coordinate_to_delete = AdjacencyMatrixCoordinate::new(
            *self
                .vertex_store_ref()
                .element_indexer_ref()
                .try_index_for_key(edge_to_delete.tail_ref())?,
            *self
                .vertex_store_ref()
                .element_indexer_ref()
                .try_index_for_key(edge_to_delete.head_ref())?,
        );
        DeleteEdgeInAdjacencyMatrix::<T>::delete_edge_unchecked(
            self.edge_store_mut_ref()
                .try_adjacency_matrix_mut_ref_for_index(&edge_type_index)?,
            &coordinate_to_delete,
        )?;
        Ok(())
    }

    fn delete_edge_defined_by_indices(
        &mut self,
        edge_to_delete: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<(), GraphComputingError> {
        DeleteEdgeInAdjacencyMatrix::<T>::delete_edge_unchecked(
            self.edge_store_mut_ref()
                .try_adjacency_matrix_mut_ref_for_index(edge_to_delete.edge_type_ref())?,
            &edge_to_delete.adjacency_matrix_coordinate(),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::{
        WeightedDirectedEdgeDefinedByKeys, WeightedDirectedEdgeDefinedByKeysTrait,
    };
    use crate::graph::vertex::vertex_defined_by_key::{
        VertexDefinedByKey, VertexDefinedByKeyTrait,
    };
    use crate::operators::add::{AddEdge, AddEdgeType, AddVertex, AddVertexType};
    use crate::operators::read::ReadEdgeWeight;

    #[test]
    fn delete_edge() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type = "vertex_type";
        let edge_type_1 = "edge_type";
        let edge_type_2 = "edge_type_2";

        let vertex_1 = VertexDefinedByKey::new("vertex_type", "vertex_1", &1u8);
        let vertex_2 = VertexDefinedByKey::new("vertex_type", "vertex_2", &2u8);

        let edge_vertex1_vertex2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_1,
                vertex_2.key_ref(),
                vertex_1.key_ref(),
            ),
            2u8,
        );
        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new(
                edge_type_2,
                vertex_1.key_ref(),
                vertex_2.key_ref(),
            ),
            3u8,
        );

        let _vertex_type_1_index = graph.add_new_vertex_type(vertex_type).unwrap();
        let _vertex_1_index = graph.add_new_key_defined_vertex(vertex_1.clone()).unwrap();
        let _vertex_2_index = graph.add_new_key_defined_vertex(vertex_2.clone()).unwrap();

        let _edge_type_1_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_1).unwrap();
        let _edge_type_2_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, edge_type_2).unwrap();

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex1_vertex2.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            // graph
            //     .key_defined_edge_weight(&edge_vertex1_vertex2.coordinate_ref())
            //     .unwrap().unwrap(),
            1u8
        );
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex2_vertex1.coordinate_ref()
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex1_vertex2_type_2.coordinate_ref()
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });

        graph
            .add_new_edge_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex1_vertex2.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            1u8
        );
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex2_vertex1.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex1_vertex2_type_2.coordinate_ref()
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });

        graph
            .add_new_edge_using_keys(edge_vertex1_vertex2_type_2.clone())
            .unwrap();
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex1_vertex2.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            1u8
        );
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex2_vertex1.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex1_vertex2_type_2.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            3u8
        );

        DeleteEdge::<u8>::delete_edge_defined_by_keys(
            &mut graph,
            edge_vertex1_vertex2.coordinate_ref(),
        )
        .unwrap();
        // graph
        //     .delete_edge_defined_by_keys(edge_vertex1_vertex2.coordinate_ref())
        //     .unwrap();
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex1_vertex2_type_2.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            3u8
        );
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex2_vertex1.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex1_vertex2.coordinate_ref()
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });

        DeleteEdge::<u8>::delete_edge_defined_by_keys(
            &mut graph,
            edge_vertex1_vertex2_type_2.coordinate_ref(),
        )
        .unwrap();
        assert_eq!(
            ReadEdgeWeight::<u8>::key_defined_edge_weight(
                &graph,
                edge_vertex2_vertex1.coordinate_ref()
            )
            .unwrap()
            .unwrap(),
            2u8
        );
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex1_vertex2.coordinate_ref()
        )
        .unwrap()
        {
            None => true,
            Some(_) => false,
        });
        assert!(match ReadEdgeWeight::<u8>::key_defined_edge_weight(
            &graph,
            edge_vertex1_vertex2_type_2.coordinate_ref()
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
