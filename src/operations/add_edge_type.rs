use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};

use crate::graph::edge::{EdgeTypeIndex, EdgeTypeKeyRef};
use crate::graph::edge_store::operations::add_edge_type::AddEdgeType as AddEdgeTypeToStore;
use crate::graph::edge_store::{
    weighted_adjacency_matrix::WeightedAdjacencyMatrix, EdgeStoreTrait,
};
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::indexer::IndexerTrait;

// use crate::operations::ReadEdge;

pub trait AddEdgeType {
    fn add_new_edge_type(
        &mut self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;

    /// If the EdgeType already exits, returns a duplicate of its EdgeTypeIndex
    fn add_new_edge_type_or_return_existing_index(
        &mut self,
        edge_type: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError>;
}

impl<'g> AddEdgeType for Graph {
    fn add_new_edge_type(
        &mut self,
        edge_type: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        self.edge_store_mut_ref().add_new_edge_type(edge_type)
    }

    fn add_new_edge_type_or_return_existing_index(
        &mut self,
        edge_type: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        self.add_new_edge_type_or_return_existing_index(edge_type)
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::context::CallGraphBlasContext;

    use super::*;

    use crate::error::GraphComputingErrorType;
    use crate::graph::graph::Graph;

    #[test]
    fn add_new_edge_type_or_return_existing_index() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let some_edge_type = String::from("some_edge_type");
        let edge_type_index = graph
            .add_new_edge_type_or_return_existing_index(some_edge_type.as_str())
            .unwrap();

        let the_same_edge_type_index = graph
            .add_new_edge_type_or_return_existing_index(some_edge_type.as_str())
            .unwrap();

        assert_eq!(edge_type_index, the_same_edge_type_index)
    }

    #[test]
    fn add_new_edge_type() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let some_edge_type = String::from("some_edge_type");
        let edge_type_index = graph.add_new_edge_type(some_edge_type.as_str()).unwrap();

        match graph.add_new_edge_type(some_edge_type.as_str()) {
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
