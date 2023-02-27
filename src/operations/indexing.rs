use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::error::{UserError, UserErrorType};

use crate::graph::edge::{
    EdgeCoordinateDefinedByIndices, EdgeCoordinateDefinedByKeys, EdgeCoordinateDefinedByKeysTrait,
};
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait, VertexIndex};
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex::VertexKey;
use crate::graph::vertex_store::operations::Indexing as VertexStoreIndexing;

pub trait Indexing {
    fn is_valid_vertex_key(&self, vertex_key: &VertexKey) -> bool;
    fn is_valid_vertex_index(&self, vertex_key: &VertexIndex) -> Result<bool, GraphComputingError>;

    fn key_defined_to_index_defined_edge_coordinate(
        &self,
        edge_coordinate: &EdgeCoordinateDefinedByKeys,
    ) -> Result<EdgeCoordinateDefinedByIndices, GraphComputingError>;
    // fn index_defined_to_key_defined_edge_coordinate(
    //     &self,
    //     edge_coordinate: &EdgeCoordinateDefinedByIndices,
    // ) -> Result<EdgeCoordinateDefinedByKeys, GraphComputingError>;
}

// TODO: where applicable, move implementations down to store level
impl<T: ValueType> Indexing for Graph<T> {
    fn is_valid_vertex_key(&self, vertex_key: &VertexKey) -> bool {
        self.vertex_store_ref().is_valid_key(vertex_key)
    }

    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref().is_valid_index(vertex_index)
    }

    fn key_defined_to_index_defined_edge_coordinate(
        &self,
        edge_coordinate: &EdgeCoordinateDefinedByKeys,
    ) -> Result<EdgeCoordinateDefinedByIndices, GraphComputingError> {
        let from_vertex_index;
        match self
            .vertex_store_ref()
            .index_for_key(edge_coordinate.originates_from_vertex())
        {
            Some(index) => from_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for from-vertex with key: {}",
                        edge_coordinate.originates_from_vertex()
                    ),
                    None,
                )
                .into())
            }
        }

        let to_vertex_index;
        match self
            .vertex_store_ref()
            .index_for_key(edge_coordinate.points_to_vertex())
        {
            Some(index) => to_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for to-vertex with key: {}",
                        edge_coordinate.points_to_vertex()
                    ),
                    None,
                )
                .into())
            }
        }

        let edge_type_index;
        match self
            .edge_store_ref()
            .edge_type_indexer_ref()
            .index_for_key(edge_coordinate.edge_type_ref())
        {
            Some(index) => edge_type_index = index.clone(),
            None => {
                return Err(LogicError::new(
                    LogicErrorType::EdgeTypeMustExist,
                    format!(
                        "No edge type found with key: {}",
                        edge_coordinate.edge_type_ref()
                    ),
                    None,
                )
                .into())
            }
        }

        Ok(EdgeCoordinateDefinedByIndices::new(
            edge_type_index,
            from_vertex_index.clone(),
            to_vertex_index.clone(),
        ))
    }

    // fn index_defined_to_key_defined_edge_coordinate(
    //     &self,
    //     edge_coordinate: &EdgeCoordinateDefinedByIndices,
    // ) -> Result<EdgeCoordinateDefinedByKeys, GraphComputingError> {
    //     Ok(EdgeCoordinate::new(
    //         edge.originates_from_vertex().index(),
    //         edge.points_to_vertex().index(),
    //     ))
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::graph::vertex::VertexValue;

    // TODO
    // #[test]
    // fn new_graph() {
    //     let graph = Graph::new(10, 20);
    // }
}
