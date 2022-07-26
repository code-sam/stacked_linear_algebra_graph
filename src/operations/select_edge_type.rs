use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::edge::{EdgeType, EdgeTypeIndex};
use crate::graph::graph::Graph;

use crate::operations::selection::edge_selection::EdgeSelection;

pub trait EdgeTypeSelectorTrait<'g> {
    fn select_edge_type(
        self: &'g Self,
        edge_type: EdgeType,
    ) -> Result<EdgeSelection<'g>, GraphComputingError>;
    fn select_edge_type_by_index(
        self: &'g Self,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<EdgeSelection<'g>, GraphComputingError>;
    // fn select_out_edge_type(
    //     graph: &Graph,
    //     edge_type: EdgeTypeRef,
    // ) -> Result<EdgeSelection, GraphComputingError>;
    // fn select_out_edge_type_by_index(
    //     graph: &Graph,
    //     edge_type: EdgeTypeIndex,
    // ) -> Result<EdgeSelection, GraphComputingError>;
}

impl<'g> EdgeTypeSelectorTrait<'g> for Graph {
    fn select_edge_type(
        self: &'g Self,
        edge_type: EdgeType,
    ) -> Result<EdgeSelection<'g>, GraphComputingError> {
        let adjacency_matrix_mask = self.get_edge_adjacency_matrix_ref(edge_type.as_str())?;
        EdgeSelection::new_for_edge_type(self, edge_type, adjacency_matrix_mask.clone())
    }

    fn select_edge_type_by_index(
        self: &'g Self,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<EdgeSelection<'g>, GraphComputingError> {
        match self.adjacency_matrices_ref().get_ref(edge_type_index) {
            Ok(adjacency_matrix_mask) => {
                EdgeSelection::new(self, edge_type_index, adjacency_matrix_mask.clone())
            }
            Err(_) => {
                // TODO: technically, another system error could have occured
                return Err(LogicError::new(
                    LogicErrorType::EdgeTypeMustExist,
                    format!("No edge type: for edge type index {:?}", edge_type_index),
                    None,
                )
                .into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::EdgeTypeKeyAndIndexConversion;
    use crate::graph::vertex::Vertex;
    use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    #[test]
    fn test_select_edge_type() {
        let graph = standard_graph_for_testing();

        let selection = graph.select_edge_type(String::from("equal_to")).unwrap();
        let selected_vertices = selection.get_vertices().unwrap();
        assert_eq!(selected_vertices.len(), 2);
        assert!(selected_vertices.contains(&Vertex::new(String::from("1"), 1u8.into())));
        assert!(selected_vertices.contains(&Vertex::new(String::from("1_duplicate"), 1u8.into())));
    }

    #[test]
    fn test_select_edge_type_by_index() {
        let graph = standard_graph_for_testing();

        let edge_type_index = graph
            .edge_type_ref_to_edge_type_index_ref(&"equal_to")
            .unwrap();
        let selection = graph
            .select_edge_type_by_index(edge_type_index.clone())
            .unwrap();
        let selected_vertices = selection.get_vertices().unwrap();
        assert_eq!(selected_vertices.len(), 2);
        assert!(selected_vertices.contains(&Vertex::new(String::from("1"), 1u8.into())));
        assert!(selected_vertices.contains(&Vertex::new(String::from("1_duplicate"), 1u8.into())));
    }
}
