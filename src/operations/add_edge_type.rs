use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};

use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
use crate::graph::edge::{EdgeType, EdgeTypeIndex};
use crate::graph::graph::Graph;

use crate::operations::read_edge::ReadEdge;

pub trait AddEdgeType {
    fn add_new_edge_type(
        &mut self,
        edge_type_key: EdgeType,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;

    /// If the EdgeType already exits, returns a duplicate of its EdgeTypeIndex
    fn add_new_edge_type_or_return_index(
        &mut self,
        edge_type: EdgeType,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<'g> AddEdgeType for Graph {
    fn add_new_edge_type(
        &mut self,
        edge_type: EdgeType,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        if !self.is_edge_type_in_graph(edge_type.as_str())? {
            add_edge_type(self, edge_type)
        } else {
            Err(LogicError::new(
                LogicErrorType::EdgeTypeAlreadyExists,
                format!("Edge type \"{}\" already exists", edge_type),
                None,
            )
            .into())
        }
    }

    fn add_new_edge_type_or_return_index(
        &mut self,
        edge_type: EdgeType,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        if self.is_edge_type_in_graph(edge_type.as_str())? {
            Ok(self
                .try_edge_type_ref_to_edge_type_index_ref(edge_type.as_str())?
                .clone())
        } else {
            add_edge_type(self, edge_type)
        }
    }
}

fn add_edge_type(
    graph: &mut Graph,
    edge_type: EdgeType,
) -> Result<EdgeTypeIndex, GraphComputingError> {
    if !graph.is_edge_type_in_graph(edge_type.as_str())? {
        let new_adjacency_matrix = AdjacencyMatrix::new(
            &graph.graphblas_context_ref(),
            edge_type.clone(),
            graph.vertex_store_ref().get_capacity()?,
        )?;
        let edge_type_index: EdgeTypeIndex = graph
            .adjacency_matrices_mut_ref()
            .push(new_adjacency_matrix)?
            .into();
        graph
            .edge_type_to_edge_type_index_map_mut_ref()
            .insert(edge_type, edge_type_index.clone());
        Ok(edge_type_index)
    } else {
        Err(LogicError::new(
            LogicErrorType::EdgeTypeAlreadyExists,
            format!("Edge type \"{}\" already exists", edge_type),
            None,
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::GraphComputingErrorType;
    use crate::graph::graph::Graph;

    #[test]
    fn add_new_edge_type_or_return_existing_index() {
        let mut graph = Graph::new(5, 5).unwrap();

        let some_edge_type = String::from("some_edge_type");
        let edge_type_index = graph
            .add_new_edge_type_or_return_index(some_edge_type.clone())
            .unwrap();

        let the_same_edge_type_index = graph
            .add_new_edge_type_or_return_index(some_edge_type)
            .unwrap();

        assert_eq!(edge_type_index, the_same_edge_type_index)
    }

    #[test]
    fn add_new_edge_type() {
        let mut graph = Graph::new(5, 5).unwrap();

        let some_edge_type = String::from("some_edge_type");
        let edge_type_index = graph.add_new_edge_type(some_edge_type.clone()).unwrap();

        match graph.add_new_edge_type(some_edge_type) {
            Err(error) => match error.error_type() {
                GraphComputingErrorType::LogicErrorType(LogicErrorType::EdgeTypeAlreadyExists) => {
                    assert!(true)
                }
                _ => assert!(false),
            },
            Ok(_) => assert!(false),
        }
    }
}
