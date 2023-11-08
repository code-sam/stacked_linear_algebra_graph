use crate::graph::edge::{
    AdjacencyMatrixCoordinate, DirectedEdgeCoordinateDefinedByIndicesTrait,
    DirectedEdgeCoordinateDefinedByKeysTrait, WeightedDirectedEdgeDefinedByIndicesTrait,
    WeightedDirectedEdgeDefinedByKeysTrait,
};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::UpdateEdgeWeight as UpdateEdgeWeightInEdgeStore;

use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::VertexStoreTrait;
use crate::{
    error::GraphComputingError,
    graph::{
        edge::{WeightedDirectedEdgeDefinedByIndices, WeightedDirectedEdgeDefinedByKeys},
        graph::{Graph, GraphTrait},
    },
};

// REVIEW update vs set
pub trait UpdateEdgeWeight<T: ValueType> {
    fn update_edge_weight_by_key(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByKeys<T>,
    ) -> Result<(), GraphComputingError>;
    fn update_edge_weight_by_index(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;
    // fn update_vertex_defined_by_type_index_and_vertex_key(
    //     &mut self,
    //     edge: &WeightedDi<T>
    // ) -> Result<(), GraphComputingError>;
    // fn update_vertex_defined_by_type_key_and_vertex_index(
    //     &mut self,
    //     vertex: &VertexDefinedByTypeKeyAndVertexIndex<T>
    // ) -> Result<(), GraphComputingError>;
}

impl UpdateEdgeWeight<bool> for Graph {
    fn update_edge_weight_by_key(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByKeys<bool>,
    ) -> Result<(), GraphComputingError> {
        let coordinate = AdjacencyMatrixCoordinate::new(
            *self
                .vertex_store_ref()
                .element_indexer_ref()
                .try_index_for_key(edge.coordinate_ref().tail_ref())?,
            *self
                .vertex_store_ref()
                .element_indexer_ref()
                .try_index_for_key(edge.coordinate_ref().head_ref())?,
        );
        self.edge_store_mut_ref()
            .adjacency_matrix_mut_ref_for_key(edge.coordinate_ref().edge_type_ref())?
            .update_edge_weight_unchecked(&coordinate, edge.weight_ref())
    }

    fn update_edge_weight_by_index(
        &mut self,
        edge: &WeightedDirectedEdgeDefinedByIndices<bool>,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_mut_ref()
            .try_adjacency_matrix_mut_ref_for_index(edge.coordinate_ref().edge_type_ref())?
            .update_edge_weight_unchecked(
                &edge.coordinate_ref().adjacency_matrix_coordinate(),
                edge.weight_ref(),
            )
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::operations::read_vertex_value::ReadVertexValue;

    // #[test]
    // fn update_vertex() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_replace_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph.update_vertex(another_vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }

    // #[test]
    // fn add_or_update_vertex() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_update_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph
    //         .add_or_update_vertex(another_vertex_to_add.clone())
    //         .unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }

    // #[test]
    // fn update_vertex_value_by_index() {
    //     let mut graph = Graph::new(5, 5).unwrap();
    //     let vertex_key = String::from("A key");
    //     let vertex_property = String::from("A property");
    //     let another_vertex_property = String::from("Another property");

    //     let vertex_to_add = Vertex::new(vertex_key.clone(), vertex_property.clone().into());
    //     graph.add_or_update_vertex(vertex_to_add.clone()).unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         vertex_to_add.value()
    //     );

    //     let another_vertex_to_add =
    //         Vertex::new(vertex_key.clone(), another_vertex_property.clone().into());
    //     graph
    //         .add_or_update_vertex(another_vertex_to_add.clone())
    //         .unwrap();

    //     assert_eq!(
    //         *graph.vertex_value(&vertex_key).unwrap(),
    //         another_vertex_to_add.value()
    //     )
    // }
}
