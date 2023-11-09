use crate::error::GraphComputingError;

use crate::graph::edge::{EdgeTypeIndex, EdgeTypeKeyRef};
use crate::graph::edge_store::operations::add_edge_type::AddEdgeType as AddEdgeTypeToStore;

use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::value_type::{GetValueTypeIdentifier, ValueType};

pub trait AddEdgeType<T: ValueType> {
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

impl<T: ValueType + GetValueTypeIdentifier> AddEdgeType<T> for Graph {
    fn add_new_edge_type(
        &mut self,
        edge_type: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToStore::<T>::add_new_edge_type(self.edge_store_mut_ref(), edge_type)
    }

    fn add_new_edge_type_or_return_existing_index(
        &mut self,
        edge_type: &EdgeTypeKeyRef,
    ) -> Result<EdgeTypeIndex, GraphComputingError> {
        AddEdgeTypeToStore::<T>::add_new_edge_type_or_return_existing_index(
            self.edge_store_mut_ref(),
            edge_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::{GraphComputingErrorType, LogicErrorType};
    use crate::graph::graph::Graph;

    #[test]
    fn add_new_edge_type_or_return_existing_index() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let some_edge_type = String::from("some_edge_type");
        let edge_type_index = AddEdgeType::<u8>::add_new_edge_type_or_return_existing_index(
            &mut graph,
            some_edge_type.as_str(),
        )
        .unwrap();

        let the_same_edge_type_index =
            AddEdgeType::<u8>::add_new_edge_type_or_return_existing_index(
                &mut graph,
                some_edge_type.as_str(),
            )
            .unwrap();

        assert_eq!(edge_type_index, the_same_edge_type_index)
    }

    #[test]
    fn add_new_edge_type() {
        let mut graph = Graph::with_initial_capacity(&5, &5, &5).unwrap();

        let some_edge_type = String::from("some_edge_type");

        let _edge_type_index =
            AddEdgeType::<u8>::add_new_edge_type(&mut graph, some_edge_type.as_str()).unwrap();

        match AddEdgeType::<u16>::add_new_edge_type(&mut graph, some_edge_type.as_str()) {
            Err(error) => match error.error_type() {
                GraphComputingErrorType::LogicErrorType(LogicErrorType::KeyAlreadyExists) => {
                    assert!(true)
                }
                _ => assert!(false),
            },
            Ok(_) => assert!(false),
        }
    }
}
