use graphblas_sparse_linear_algebra::value_type::ValueType;

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
use crate::graph::value_type::implement_macro_for_all_native_value_types;
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

macro_rules! implement_delete_edge {
    ($value_type:ty) => {
        impl DeleteEdge<$value_type> for Graph {
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
                DeleteEdgeInAdjacencyMatrix::<$value_type>::delete_edge_unchecked(
                    self.edge_store_mut_ref()
                        .try_adjacency_matrix_mut_ref(&edge_type_index)?,
                    &coordinate_to_delete,
                )?;
                Ok(())
            }
        
            fn delete_edge_defined_by_indices(
                &mut self,
                edge_to_delete: &DirectedEdgeCoordinateDefinedByIndices,
            ) -> Result<(), GraphComputingError> {
                DeleteEdgeInAdjacencyMatrix::<$value_type>::delete_edge_unchecked(
                    self.edge_store_mut_ref()
                        .try_adjacency_matrix_mut_ref(edge_to_delete.edge_type_ref())?,
                    &edge_to_delete.adjacency_matrix_coordinate(),
                )?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_delete_edge);


#[cfg(test)]
mod tests {
    use super::*;

    //     use crate::graph::vertex::Vertex;
    //     use crate::operations::add_edge::AddEdge;
    //     use crate::operations::add_vertex::AddVertex;
    //     use crate::operations::read_edge::ReadEdge;

    //     #[test]
    //     fn delete_edge() {
    //         let mut graph = Graph::new(5, 5).unwrap();

    //         let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
    //         let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

    //         let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
    //             vertex_1.clone().into(),
    //             String::from("edge_type_1"),
    //             vertex_2.clone().into(),
    //         );
    //         let edge_vertex2_vertex1 = DirectedEdgeDefinedByKeys::new(
    //             vertex_2.clone().into(),
    //             String::from("edge_type_1"),
    //             vertex_1.clone().into(),
    //         );
    //         let edge_vertex1_vertex2_type2 = DirectedEdgeDefinedByKeys::new(
    //             vertex_1.clone().into(),
    //             String::from("edge_type_2"),
    //             vertex_2.clone().into(),
    //         );

    //         graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
    //         graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

    //         graph
    //             .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
    //             .unwrap();
    //         assert_eq!(
    //             graph
    //                 .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //                 .unwrap(),
    //             true
    //         );
    //         assert!(!graph
    //             .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //             .unwrap());
    //         assert!(!graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //             .unwrap());

    //         graph
    //             .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
    //             .unwrap();
    //         graph
    //             .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
    //             .unwrap();
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //             .unwrap());
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //             .unwrap());
    //         assert!(!graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //             .unwrap());

    //         graph
    //             .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
    //             .unwrap();
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //             .unwrap());
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //             .unwrap());
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //             .unwrap());

    //         graph
    //             .delete_edge_defined_by_keys(&edge_vertex1_vertex2)
    //             .unwrap();
    //         assert!(!graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
    //             .unwrap());
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //             .unwrap());
    //         graph
    //             .delete_edge_defined_by_keys(&edge_vertex2_vertex1)
    //             .unwrap();
    //         assert!(!graph
    //             .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
    //             .unwrap());
    //         assert!(graph
    //             .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
    //             .unwrap())
    //     }

    //     #[test]
    //     fn delete_non_existing_edge() {
    //         let mut graph = Graph::new(5, 5).unwrap();

    //         let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
    //         let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

    //         let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
    //             vertex_1.clone().into(),
    //             String::from("edge_type_1"),
    //             vertex_2.clone().into(),
    //         );

    //         let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //         match result {
    //             Err(_) => assert!(true),
    //             _ => assert!(false),
    //         }

    //         graph.add_or_replace_vertex(vertex_1.clone()).unwrap();

    //         let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //         match result {
    //             Err(_) => assert!(true),
    //             _ => assert!(false),
    //         }

    //         // Deleting non-existing edge, connecting existing edges
    //         let result = graph.delete_edge_defined_by_keys(&edge_vertex1_vertex2);
    //         match result {
    //             Err(_) => assert!(true),
    //             _ => assert!(false),
    //         }
    //     }
}
