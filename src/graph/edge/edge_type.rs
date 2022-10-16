use crate::error::GraphComputingError;
use crate::error::{LogicError, LogicErrorType};
use crate::error::{SystemError, SystemErrorType};
use crate::graph::index::{ElementIndex, Index};
use crate::graph::graph::graph::Graph;

// use crate::graph::indexed_data_store::index::Index as IndexedDataStoreIndex;

// TODO: change to EdgeTypeKey?
pub type EdgeType = String;
pub type EdgeTypeRef = str;

// pub enum Edge {
//     Directed(DirectedEdge),
// }

// TODO: add constructor with indices
// TODO: consider modelling a DirectedEdge as an enum. Each variant can model a different state/representation. E.g. defintion by keys, by indices, with existing vertices, with new vertices, etc.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeTypeIndex {
    index: IndexedDataStoreIndex,
}

impl EdgeTypeIndex {
    pub(crate) fn new(index: IndexedDataStoreIndex) -> Self {
        EdgeTypeIndex { index }
    }
    pub(crate) fn index(self) -> ElementIndex {
        self.index
    }
    pub(crate) fn index_ref(&self) -> &IndexedDataStoreIndex {
        &self.index
    }
}

pub trait EdgeTypeKeyAndIndexConversion {
    fn edge_type_index_to_edge_type_ref(
        &self,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<&EdgeTypeRef, GraphComputingError>;

    fn edge_type_ref_to_edge_type_index_ref(
        &self,
        key: &EdgeTypeRef,
    ) -> Result<&EdgeTypeIndex, GraphComputingError>;
}

impl EdgeTypeKeyAndIndexConversion for Graph {
    fn edge_type_index_to_edge_type_ref(
        &self,
        edge_type_index: EdgeTypeIndex,
    ) -> Result<&EdgeTypeRef, GraphComputingError> {
        match self.adjacency_matrices_ref().get_ref(edge_type_index) {
            Ok(adjacency_matrix) => return Ok(adjacency_matrix.edge_type_ref()),
            Err(_) => {
                // TODO:match actual error type
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!("There is no vertex at index [{}]", edge_type_index.index()),
                    None,
                )
                .into());
            }
        }
    }

    fn edge_type_ref_to_edge_type_index_ref(
        &self,
        key: &EdgeTypeRef,
    ) -> Result<&EdgeTypeIndex, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map_ref().get(key) {
            None => Err(SystemError::new(
                SystemErrorType::KeyNotFound,
                format!(
                    "Could not map edge type key '{}' to an edge type index",
                    key
                ),
                None,
            )
            .into()),
            Some(edge_type_index) => Ok(edge_type_index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;
    use crate::graph::graph::GraphTrait;

    #[test]
    fn test_convert_edge_type_index_to_edge_type_key_ref() {
        let mut graph = Graph::new(10, 20).unwrap();

        let edge_type_key_1 = String::from("Vertex_1");
        let adjacency_matrix_1 = AdjacencyMatrix::new(
            graph.graphblas_context_ref(),
            edge_type_key_1.clone(),
            graph.vertex_capacity().unwrap(),
        )
        .unwrap();

        let index_edge_type_1: EdgeTypeIndex = graph
            .adjacency_matrices_mut_ref()
            .push(adjacency_matrix_1)
            .unwrap()
            .into();
        assert_eq!(
            graph
                .edge_type_index_to_edge_type_ref(index_edge_type_1)
                .unwrap(),
            edge_type_key_1.as_str()
        )
    }
}
