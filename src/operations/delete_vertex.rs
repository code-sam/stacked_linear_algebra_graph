use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::operators::{
    binary_operator::Assignment,
    insert::{
        InsertVectorIntoColumn, InsertVectorIntoColumnTrait, InsertVectorIntoRow,
        InsertVectorIntoRowTrait,
    },
    options::OperatorOptions,
};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::weighted_adjacency_matrix::operations::{
            DeleteVertexConnections, DeleteVertexConnectionsForAllTypes,
        },
        graph::{GraphTrait, VertexIndex, VertexTypeIndex},
        indexer::IndexerTrait,
        value_type::implement_macro_for_all_native_value_types,
        vertex::{VertexKeyRef, VertexTypeKeyRef},
        vertex_store::{
            vertex_operations::{
                DeleteVertexElement as DeleteVertexElementFromVertexStore, DeleteVertexForAllTypes,
            },
            VertexStoreTrait,
        },
    },
};

use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::graph::Graph;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::VertexKey;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static INSERT_VECTOR_INTO_COLUMN_OPERATOR: Lazy<InsertVectorIntoColumn<bool, bool>> =
    Lazy::new(|| {
        InsertVectorIntoColumn::<bool, bool>::new(
            &DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS,
            &Assignment::new(),
        )
    });

static INSERT_VECTOR_INTO_ROW_OPERATOR: Lazy<InsertVectorIntoRow<bool, bool>> = Lazy::new(|| {
    InsertVectorIntoRow::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, &Assignment::new())
});

pub trait DeleteVertex {
    fn drop_vertex_key_and_connected_edges(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;
    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
    // fn delete_selected_vertices_and_connected_edges(
    //     &mut self,
    //     vertex_selection: VertexSelection,
    // ) -> Result<(), GraphComputingError>;
}

pub trait DeleteVertexElement<T: ValueType> {
    fn delete_vertex_element_by_key(
        &mut self,
        vertex_type_key: &VertexTypeKeyRef,
        vertex_element_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn delete_vertex_element_by_index(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_element_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
}

impl DeleteVertex for Graph {
    fn drop_vertex_key_and_connected_edges(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<(), GraphComputingError> {
        let vertex_index = *self
            .vertex_store_ref()
            .element_indexer_ref()
            .try_index_for_key(vertex_key)?;
        self.drop_vertex_index_and_connected_edges(&vertex_index)
    }

    fn drop_vertex_index_and_connected_edges(
        &mut self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        // TODO: Consider restricting to valid indices for improved performance
        self.edge_store_mut_ref().map_mut_all_adjacency_matrices(
            |adjacency_matrix: &mut WeightedAdjacencyMatrix| {
                adjacency_matrix
                    .delete_vertex_connections_for_all_value_types_unchecked(vertex_index)
            },
        )?;

        self.vertex_store_mut_ref()
            .delete_vertex_for_all_vertex_types_and_value_types_by_index(vertex_index)
    }
}

macro_rules! implement_delete_vertex_element {
    ($value_type: ty) => {
        impl DeleteVertexElement<$value_type> for Graph {
            fn delete_vertex_element_by_key(
                &mut self,
                vertex_type_key: &VertexTypeKeyRef,
                vertex_element_key: &VertexKeyRef,
            ) -> Result<(), GraphComputingError> {
                DeleteVertexElementFromVertexStore::<$value_type>::delete_vertex_element_by_key(
                    self.vertex_store_mut_ref(),
                    vertex_type_key,
                    vertex_element_key,
                )
            }

            fn delete_vertex_element_by_index(
                &mut self,
                vertex_type_index: &VertexTypeIndex,
                vertex_element_index: &VertexIndex,
            ) -> Result<(), GraphComputingError> {
                DeleteVertexElementFromVertexStore::<$value_type>::delete_vertex_element_by_index(
                    self.vertex_store_mut_ref(),
                    vertex_type_index,
                    vertex_element_index,
                )
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_delete_vertex_element);

#[cfg(test)]
mod tests {
    use super::*;

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
