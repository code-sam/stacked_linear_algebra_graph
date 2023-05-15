use graphblas_sparse_linear_algebra::collections::sparse_matrix::Coordinate;
// use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::{GraphComputingError, LogicError, LogicErrorType, SystemError, SystemErrorType};

use crate::graph::edge::{
    AdjacencyMatrixCoordinate, DirectedEdgeCoordinateDefinedByIndices,
    DirectedEdgeCoordinateDefinedByIndicesTrait, DirectedEdgeCoordinateDefinedByKeysTrait,
    EdgeTypeIndex, WeightedDirectedEdgeDefinedByIndices, WeightedDirectedEdgeDefinedByIndicesTrait,
    WeightedDirectedEdgeDefinedByKeys, WeightedDirectedEdgeDefinedByKeysTrait,
};
use crate::graph::edge_store::operations::add_edge_type::AddEdgeType;
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::{
    AddEdge as AddEdgeToAdjacencyMatrix, Indexing,
};
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};
use crate::graph::vertex_store::vertex_operations::AddVertex;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::operations::Indexing as GraphIndexing;

use super::add_edge_type::AddEdgeType as AddEdgeTypeToGraph;

pub trait AddEdge<T: ValueType> {
    fn add_new_edge_using_keys(
        &mut self,
        edge: WeightedDirectedEdgeDefinedByKeys<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge_using_keys(
        &mut self,
        edge: WeightedDirectedEdgeDefinedByKeys<T>,
    ) -> Result<(), GraphComputingError>;

    /// If the EdgeType already exists, then the edge is added to it.
    /// Existing edges for the EdgesType remain unaffected.
    fn add_new_edge_and_edge_type_using_keys(
        &mut self,
        edge: WeightedDirectedEdgeDefinedByKeys<T>,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;

    fn add_new_edge_using_indices(
        &mut self,
        edge: WeightedDirectedEdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_or_replace_edge_using_indices(
        &mut self,
        edge: WeightedDirectedEdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;
}

macro_rules! implement_add_edge {
    ($value_type:ty) => {
        impl AddEdge<$value_type> for Graph {
            fn add_new_edge_using_keys(
                &mut self,
                edge: WeightedDirectedEdgeDefinedByKeys<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let tail_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().tail_ref())?;
                let head_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().head_ref())?;

                let adjacency_matrix = self
                    .edge_store_mut_ref()
                    .adjacency_matrix_mut_ref_for_key(edge.coordinate_ref().edge_type_ref())?;

                if Indexing::<$value_type>::is_edge(
                    adjacency_matrix,
                    &AdjacencyMatrixCoordinate::new(tail_index, head_index),
                )? {
                    return Err(LogicError::new(
                        LogicErrorType::EdgeAlreadyExists,
                        format!("An edge already existis for: {:?}", edge.coordinate_ref()),
                        None,
                    )
                    .into());
                }

                adjacency_matrix.add_edge_defined_by_indices_without_edge_type_unchecked(
                    &tail_index,
                    &head_index,
                    edge.weight_ref(),
                )?;
                Ok(())
            }

            fn add_or_replace_edge_using_keys(
                &mut self,
                edge: WeightedDirectedEdgeDefinedByKeys<$value_type>,
            ) -> Result<(), GraphComputingError> {
                let tail_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().tail_ref())?;
                let head_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().head_ref())?;

                let adjacency_matrix = self
                    .edge_store_mut_ref()
                    .adjacency_matrix_mut_ref_for_key(edge.coordinate_ref().edge_type_ref())?;

                adjacency_matrix.add_edge_defined_by_indices_without_edge_type_unchecked(
                    &tail_index,
                    &head_index,
                    edge.weight_ref(),
                )?;
                Ok(())
            }

            /// If the EdgeType already exists, then the edge is added to it.
            /// Existing edges for the EdgesType remain unaffected.
            fn add_new_edge_and_edge_type_using_keys(
                &mut self,
                edge: WeightedDirectedEdgeDefinedByKeys<$value_type>,
            ) -> Result<EdgeTypeIndex, GraphComputingError> {
                let edge_type_index = self
                    .edge_store_mut_ref()
                    .add_new_edge_type(edge.coordinate_ref().edge_type_ref())?;

                let tail_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().tail_ref())?;
                let head_index =
                    *self.try_vertex_index_for_key(edge.coordinate_ref().head_ref())?;

                self.edge_store_mut_ref()
                    .adjacency_matrix_mut_ref_unchecked_for_index(&edge_type_index)
                    .add_edge_defined_by_indices_without_edge_type_unchecked(
                        &tail_index,
                        &head_index,
                        edge.weight_ref(),
                    )?;
                Ok(edge_type_index)
            }

            fn add_new_edge_using_indices(
                &mut self,
                edge: WeightedDirectedEdgeDefinedByIndices<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.try_vertex_index_validity(edge.coordinate_ref().tail_ref())?;
                self.try_vertex_index_validity(edge.coordinate_ref().head_ref())?;

                let adjacency_matrix = self
                    .edge_store_mut_ref()
                    .try_adjacency_matrix_mut_ref_for_index(
                        edge.coordinate_ref().edge_type_ref(),
                    )?;

                if Indexing::<$value_type>::is_edge(
                    adjacency_matrix,
                    &AdjacencyMatrixCoordinate::new(
                        *edge.coordinate_ref().tail_ref(),
                        *edge.coordinate_ref().head_ref(),
                    ),
                )? {
                    return Err(LogicError::new(
                        LogicErrorType::EdgeAlreadyExists,
                        format!("An edge already existis for: {:?}", edge.coordinate_ref()),
                        None,
                    )
                    .into());
                }

                adjacency_matrix.add_edge_defined_by_indices_unchecked(&edge)?;
                Ok(())
            }

            fn add_or_replace_edge_using_indices(
                &mut self,
                edge: WeightedDirectedEdgeDefinedByIndices<$value_type>,
            ) -> Result<(), GraphComputingError> {
                self.try_vertex_index_validity(edge.coordinate_ref().tail_ref())?;
                self.try_vertex_index_validity(edge.coordinate_ref().head_ref())?;

                let adjacency_matrix = self
                    .edge_store_mut_ref()
                    .try_adjacency_matrix_mut_ref_for_index(
                        edge.coordinate_ref().edge_type_ref(),
                    )?;

                adjacency_matrix.add_edge_defined_by_indices_unchecked(&edge)?;
                Ok(())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_add_edge);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::DirectedEdgeCoordinateDefinedByKeys;
    use crate::graph::graph::Graph;
    use crate::graph::vertex::VertexDefinedByKey;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::AddVertexType;

    #[test]
    fn add_edge() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let vertex_type_1_key = String::from("vertex_type_1");
        let _vertex_type_2_key = String::from("vertex_type_2");

        let _vertex_type_index = graph
            .add_new_vertex_type(vertex_type_1_key.as_str())
            .unwrap();
        let vertex_1 = VertexDefinedByKey::new(
            vertex_type_1_key.as_str(),
            String::from("vertex_1").as_str(),
            &1u8,
        );
        let vertex_2 = VertexDefinedByKey::new(
            vertex_type_1_key.as_str(),
            String::from("vertex_2").as_str(),
            &2u8,
        );

        let _vertex1_index = graph
            .add_or_update_vertex(vertex_1.clone())
            .unwrap()
            .unwrap();
        let _vertex2_index = graph
            .add_or_update_vertex(vertex_2.clone())
            .unwrap()
            .unwrap();

        let _edge_vertex1_vertex2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new("edge_type_1", "vertex_1", "vertex_2"),
            1u8,
        );
        let _edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new("edge_type_1", "vertex_2", "vertex_1"),
            2u8,
        );

        let _edge_vertex2_vertex1_type2 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new("edge_type_2", "vertex_2", "vertex_1"),
            2u8,
        );

        let _edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new("edge_type_2", "vertex_2", "vertex_1"),
            3u8,
        );
        let _edge_vertex2_vertex1 = WeightedDirectedEdgeDefinedByKeys::new(
            DirectedEdgeCoordinateDefinedByKeys::new("edge_type_2", "vertex_1", "vertex_2"),
            4u8,
        );

        // graph
        //     .add_new_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
        //     .unwrap();
        // assert_eq!(
        //     graph
        //         .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
        //         .unwrap(),
        //     true
        // );
        // assert!(!graph
        //     .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
        //     .unwrap());
        // assert!(!graph
        //     .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
        //     .unwrap());

        // graph
        //     .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
        //     .unwrap();
        // graph
        //     .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
        //     .unwrap();
        // assert!(graph
        //     .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
        //     .unwrap());
        // assert!(graph
        //     .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
        //     .unwrap());
        // assert!(!graph
        //     .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
        //     .unwrap());

        // graph
        //     .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
        //     .unwrap();
        // assert!(graph
        //     .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
        //     .unwrap());
        // assert!(graph
        //     .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
        //     .unwrap());
        // assert!(graph
        //     .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
        //     .unwrap());
    }

    #[test]
    fn add_edge_errors() {
        // let mut graph = Graph::new(5, 5).unwrap();

        // let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        // let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        // let edge_vertex1_vertex2 = WeightedDirectedEdgeDefinedByKeys::new(
        //     vertex_1.clone().into(),
        //     String::from("edge_type_1"),
        //     vertex_2.clone().into(),
        // );

        // match graph.add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone()) {
        //     Err(_) => assert!(true),
        //     Ok(_) => assert!(false),
        // }

        // graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        // match graph.add_edge_and_edge_type_using_keys(edge_vertex1_vertex2) {
        //     Err(_) => assert!(true),
        //     Ok(_) => assert!(false),
        // }
    }
}
