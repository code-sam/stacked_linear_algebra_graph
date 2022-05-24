use once_cell::sync::Lazy;

use graphblas_sparse_linear_algebra::{
    operators::{
        insert::{
            InsertVectorIntoColumn, InsertVectorIntoColumnTrait, InsertVectorIntoRow,
            InsertVectorIntoRowTrait,
        },
        options::OperatorOptions,
    },
    util::ElementIndexSelector,
    value_types::sparse_vector::SparseVector,
};

use crate::error::GraphComputingError;

use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
use crate::graph::graph::Graph;
use crate::graph::vertex::VertexKey;

static DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS: Lazy<OperatorOptions> =
    Lazy::new(|| OperatorOptions::new_default());

static INSERT_VECTOR_INTO_COLUMN_OPERATOR: Lazy<InsertVectorIntoColumn<bool, bool>> =
    Lazy::new(|| {
        InsertVectorIntoColumn::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, None)
    });

static INSERT_VECTOR_INTO_ROW_OPERATOR: Lazy<InsertVectorIntoRow<bool, bool>> =
    Lazy::new(|| InsertVectorIntoRow::<bool, bool>::new(&DEFAULT_GRAPHBLAS_OPERATOR_OPTIONS, None));

pub trait DeleteVertex {
    fn delete_vertex_and_connected_edges(
        &mut self,
        vertex_key: VertexKey,
    ) -> Result<(), GraphComputingError>;
    // fn delete_selected_vertices_and_connected_edges(
    //     &mut self,
    //     vertex_selection: VertexSelection,
    // ) -> Result<(), GraphComputingError>;
}

impl DeleteVertex for Graph {
    fn delete_vertex_and_connected_edges(
        &mut self,
        vertex_key: VertexKey,
    ) -> Result<(), GraphComputingError> {
        let vertex_index;
        match self.vertex_key_to_vertex_index_map_ref().get(&vertex_key) {
            Some(index) => vertex_index = index.clone(),
            None => return Ok(()),
        }

        self.vertex_store_mut_ref().free(vertex_index.clone())?;
        self.vertex_key_to_vertex_index_map_mut_ref()
            .remove_entry(&vertex_key);

        let empty_column = SparseVector::<bool>::new(
            &self.graphblas_context_ref(),
            &self.vertex_store_ref().get_capacity()?,
        )?;

        // TODO: is inserting an empty vector the fastest way to delete a row/column?
        let delete_connected_edges =
            |adjacency_matrix: &mut AdjacencyMatrix| -> Result<(), GraphComputingError> {
                INSERT_VECTOR_INTO_COLUMN_OPERATOR.apply(
                    adjacency_matrix.as_mut_sparse_matrix(),
                    &ElementIndexSelector::All,
                    vertex_index.index_ref(),
                    &empty_column,
                )?;
                INSERT_VECTOR_INTO_ROW_OPERATOR.apply(
                    adjacency_matrix.as_mut_sparse_matrix(),
                    &ElementIndexSelector::All,
                    vertex_index.index_ref(),
                    &empty_column,
                )?;
                Ok(())
            };

        // TODO: some matrices may have been freed and do not need to be updated, potentially saving time.
        self.adjacency_matrices_mut_ref()
            .map_mut_all(delete_connected_edges)
        // self.edge_type_to_edge_type_index_map
        //     .par_values()
        //     .for_each(|edge_type_index| {
        //         let adjacency_matrix = self
        //             .adjacency_matrices
        //             .get_mut(edge_type_index)
        //             .unwrap().unwrap() // REVIEW: should this throw an error instead of panicing?
        //             .as_mut_sparse_matrix();

        //         insert_column_operator.apply(
        //             adjacency_matrix,
        //             &ElementIndexSelector::All,
        //             edge_type_index,
        //             &empty_column,
        //         );
        //         insert_row_operator.apply(
        //             adjacency_matrix,
        //             &ElementIndexSelector::All,
        //             edge_type_index,
        //             &empty_column,
        //         );
        //     });
        // Ok(())
        // REVIEW: should edge_types that have no connections after deleting a vertex be deleted as well?
        // => probably not, it's simply an unsed edge_type, which may be deleted in a separate operation
    }

    // fn delete_vertex_by_index(&mut self, vertex_index: VertexIndex) {
    //     unimplemented!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::DirectedEdgeDefinedByKeys;
    use crate::graph::vertex::Vertex;
    use crate::operations::add_edge::AddEdge;
    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_edge::ReadEdge;
    use crate::operations::read_vertex_value::ReadVertexValue;

    #[test]
    fn delete_vertex_and_connected_edges() {
        let mut graph = Graph::new(5, 5).unwrap();
        let vertex_key_1 = String::from("vertex_1");
        let vertex_key_2 = String::from("vertex_2");

        let vertex_1 = Vertex::new(String::from("vertex_1"), String::from("vertex_1").into());
        let vertex_2 = Vertex::new(String::from("vertex_2"), String::from("vertex_2").into());

        let edge_vertex1_vertex2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_1"),
            vertex_2.clone().into(),
        );
        let edge_vertex2_vertex1 = DirectedEdgeDefinedByKeys::new(
            vertex_2.clone().into(),
            String::from("edge_type_1"),
            vertex_1.clone().into(),
        );
        let edge_vertex1_vertex2_type2 = DirectedEdgeDefinedByKeys::new(
            vertex_1.clone().into(),
            String::from("edge_type_2"),
            vertex_2.clone().into(),
        );

        graph.add_or_replace_vertex(vertex_1.clone()).unwrap();
        graph.add_or_replace_vertex(vertex_2.clone()).unwrap();

        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2.clone())
            .unwrap();
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex2_vertex1.clone())
            .unwrap();
        graph
            .add_edge_and_edge_type_using_keys(edge_vertex1_vertex2_type2.clone())
            .unwrap();

        graph
            .delete_vertex_and_connected_edges(vertex_key_1.clone())
            .unwrap();

        assert!(
            !graph
                .is_key_defined_edge_in_graph(&edge_vertex1_vertex2)
                .unwrap()
        );
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex2_vertex1)
            .unwrap());
        assert!(!graph
            .is_key_defined_edge_in_graph(&edge_vertex1_vertex2_type2)
            .unwrap());

        assert!(!graph.is_valid_vertex_key(&vertex_key_1));
        assert!(graph.is_valid_vertex_key(&vertex_key_2));
    }
}
