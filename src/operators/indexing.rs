use crate::error::{GraphComputingError, LogicError, LogicErrorType};

use crate::graph::edge::{
    DirectedEdgeCoordinateDefinedByIndices, DirectedEdgeCoordinateDefinedByIndicesTrait,
    DirectedEdgeCoordinateDefinedByKeys, DirectedEdgeCoordinateDefinedByKeysTrait, EdgeTypeIndex,
    EdgeTypeKey, EdgeTypeKeyRef,
};
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex};
use crate::graph::indexer::IndexerTrait;

use crate::graph::vertex::vertex::{VertexKey, VertexKeyRef, VertexTypeKey, VertexTypeKeyRef};
use crate::graph::vertex_store::VertexStoreTrait;

// pub trait IndexMonitoring {
//     fn vertex_key_to_index_map_ref(&self) -> &HashMap<VertexKey, VertexIndex>;

// TODO: exposing a raw array using a map won't work since the vector may contain invalidated (deleted) keys
//     fn vertex_index_to_key_map_ref(&self) -> &[VertexKey];

//     fn vertex_type_key_to_index_map_ref(&self) -> &HashMap<VertexTypeKey, VertexTypeIndex>;
//     fn vertex_type_index_to_key_map_ref(&self) -> &[VertexTypeKey];

//     fn edge_type_key_to_index_map_ref(&self) -> &HashMap<EdgeTypeKey, EdgeTypeIndex>;
//     fn edge_type_index_to_key_map_ref(&self) -> &[EdgeTypeKey];
// }

pub trait Indexing {
    fn is_valid_vertex_key(&self, vertex_key: &VertexKeyRef) -> bool;
    fn is_valid_vertex_type_key(&self, vertex_type_key: &VertexTypeKeyRef) -> bool;
    fn is_valid_edge_type_key(&self, edge_type_key: &EdgeTypeKeyRef) -> bool;

    fn is_valid_vertex_index(&self, vertex_key: &VertexIndex) -> Result<bool, GraphComputingError>;
    fn is_valid_vertex_type_index(
        &self,
        vertex_key: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError>;
    fn is_valid_edge_type_index(
        &self,
        vertex_key: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError>;

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
    fn try_vertex_key_validity(
        &self,
        vertex_index: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexIndex,
    ) -> Result<(), GraphComputingError>;
    fn try_vertex_type_key_validity(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError>;
    fn try_edge_type_key_validity(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError>;

    fn vertex_index_for_key(&self, vertex_key: &VertexKeyRef) -> Option<&VertexIndex>;
    fn try_vertex_index_for_key(
        &self,
        vertex_key: &VertexKeyRef,
    ) -> Result<&VertexIndex, GraphComputingError>;

    fn vertex_type_index_for_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Option<&VertexTypeIndex>;
    fn try_vertex_type_index_for_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<&VertexTypeIndex, GraphComputingError>;

    fn edge_type_index_for_key(&self, edge_type_key: &EdgeTypeKeyRef) -> Option<&EdgeTypeIndex>;
    fn try_edge_type_index_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&EdgeTypeIndex, GraphComputingError>;

    fn vertex_key_for_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<VertexKey, GraphComputingError>;
    // fn try_vertex_key_for_index(&self, vertex_index: &VertexIndex) -> Result<&VertexKeyRef, GraphComputingError>;

    fn vertex_type_key_for_index(
        &self,
        vertex_index: &VertexTypeIndex,
    ) -> Result<VertexTypeKey, GraphComputingError>;
    // fn try_vertex_type_key_for_index(&self, vertex_index: &VertexTypeIndex) -> Result<&VertexTypeKeyRef, GraphComputingError>;

    fn edge_type_key_for_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<EdgeTypeKey, GraphComputingError>;
    // fn try_edge_type_key_for_index(&self, edge_type_index: &EdgeTypeIndex) -> Result<&EdgeTypeKeyRef, GraphComputingError>;

    fn key_defined_to_index_defined_edge_coordinate(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<DirectedEdgeCoordinateDefinedByIndices, GraphComputingError>;
    // fn index_defined_to_key_defined_edge_coordinate(
    //     &self,
    //     edge_coordinate: &EdgeCoordinateDefinedByIndices,
    // ) -> Result<EdgeCoordinateDefinedByKeys, GraphComputingError>;

    fn is_valid_key_defined_edge_coordinate(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> bool;
    fn is_valid_index_defined_edge_coordinate(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<bool, GraphComputingError>;

    fn try_key_defined_edge_coordinate_validity(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<(), GraphComputingError>;
    fn try_index_defined_edge_coordinate_validity(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<(), GraphComputingError>;
}

// TODO: where applicable, move implementations down to store level
impl Indexing for Graph {
    fn is_valid_vertex_key(&self, vertex_key: &VertexKeyRef) -> bool {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_key(vertex_key)
    }

    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .is_valid_index(vertex_index)
    }

    fn key_defined_to_index_defined_edge_coordinate(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<DirectedEdgeCoordinateDefinedByIndices, GraphComputingError> {
        let tail_vertex_index;
        match self
            .vertex_store_ref()
            .element_indexer_ref()
            .index_for_key(edge_coordinate.tail_ref())
        {
            Some(index) => tail_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for tail-vertex with key: {}",
                        edge_coordinate.tail_ref()
                    ),
                    None,
                )
                .into())
            }
        }

        let head_vertex_index;
        match self
            .vertex_store_ref()
            .element_indexer_ref()
            .index_for_key(edge_coordinate.head_ref())
        {
            Some(index) => head_vertex_index = index,
            None => {
                return Err(LogicError::new(
                    LogicErrorType::VertexMustExist,
                    format!(
                        "No vertex found for head-vertex with key: {}",
                        edge_coordinate.head_ref()
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

        Ok(DirectedEdgeCoordinateDefinedByIndices::new(
            edge_type_index,
            tail_vertex_index.clone(),
            head_vertex_index.clone(),
        ))
    }

    fn is_valid_vertex_type_key(&self, vertex_type_key: &VertexTypeKeyRef) -> bool {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_key(vertex_type_key)
    }

    fn is_valid_edge_type_key(&self, edge_type_key: &EdgeTypeKeyRef) -> bool {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .is_valid_key(edge_type_key)
    }

    fn is_valid_vertex_type_index(
        &self,
        vertex_type_index: &VertexTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .is_valid_index(vertex_type_index)
    }

    fn is_valid_edge_type_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<bool, GraphComputingError> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .is_valid_index(edge_type_index)
    }

    fn try_vertex_index_validity(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_index_validity(vertex_index)
    }

    fn try_vertex_key_validity(
        &self,
        vertex_key: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_key_validity(vertex_key)
    }

    fn try_vertex_type_index_validity(
        &self,
        vertex_type_index: &VertexIndex,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)
    }

    fn try_vertex_type_key_validity(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_key_validity(vertex_type_key)
    }

    fn try_edge_type_index_validity(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .try_index_validity(edge_type_index)
    }

    fn try_edge_type_key_validity(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<(), GraphComputingError> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .try_key_validity(edge_type_key)
    }

    fn vertex_index_for_key(&self, vertex_key: &VertexKeyRef) -> Option<&VertexIndex> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .index_for_key(vertex_key)
    }

    fn try_vertex_index_for_key(
        &self,
        vertex_key: &VertexKeyRef,
    ) -> Result<&VertexIndex, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .try_index_for_key(vertex_key)
    }

    fn vertex_type_index_for_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Option<&VertexTypeIndex> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .index_for_key(vertex_type_key)
    }

    fn try_vertex_type_index_for_key(
        &self,
        vertex_type_key: &VertexTypeKeyRef,
    ) -> Result<&VertexTypeIndex, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .try_index_for_key(vertex_type_key)
    }

    fn edge_type_index_for_key(&self, edge_type_key: &EdgeTypeKeyRef) -> Option<&EdgeTypeIndex> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .index_for_key(edge_type_key)
    }

    fn try_edge_type_index_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&EdgeTypeIndex, GraphComputingError> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .try_index_for_key(edge_type_key)
    }

    fn vertex_key_for_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<VertexKey, GraphComputingError> {
        self.vertex_store_ref()
            .element_indexer_ref()
            .key_for_index(vertex_index)
    }

    // fn try_vertex_key_for_index(&self, vertex_index: &VertexIndex) -> Result<&VertexKeyRef, GraphComputingError> {
    //     self.vertex_store_ref().element_indexer_ref().tr
    // }

    fn vertex_type_key_for_index(
        &self,
        vertex_index: &VertexTypeIndex,
    ) -> Result<VertexTypeKey, GraphComputingError> {
        self.vertex_store_ref()
            .vertex_type_indexer_ref()
            .key_for_index(vertex_index)
    }

    // fn try_vertex_type_key_for_index(&self, vertex_index: &VertexTypeIndex) -> Result<&VertexTypeKeyRef, GraphComputingError> {
    //     todo!()
    // }

    fn edge_type_key_for_index(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<EdgeTypeKey, GraphComputingError> {
        self.edge_store_ref()
            .edge_type_indexer_ref()
            .key_for_index(edge_type_index)
    }

    fn is_valid_key_defined_edge_coordinate(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> bool {
        self.is_valid_edge_type_key(edge.edge_type_ref())
            && self.is_valid_vertex_key(edge.tail_ref())
            && self.is_valid_vertex_key(edge.head_ref())
    }

    fn is_valid_index_defined_edge_coordinate(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<bool, GraphComputingError> {
        Ok(self.is_valid_edge_type_index(edge.edge_type_ref())?
            && self.is_valid_vertex_index(edge.tail_ref())?
            && self.is_valid_vertex_index(edge.head_ref())?)
    }

    fn try_key_defined_edge_coordinate_validity(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_key_validity(edge.edge_type_ref())?;
        self.try_vertex_key_validity(edge.tail_ref())?;
        self.try_vertex_key_validity(edge.head_ref())?;
        Ok(())
    }

    fn try_index_defined_edge_coordinate_validity(
        &self,
        edge: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<(), GraphComputingError> {
        self.try_edge_type_index_validity(edge.edge_type_ref())?;
        self.try_vertex_index_validity(edge.tail_ref())?;
        self.try_vertex_index_validity(edge.head_ref())?;
        Ok(())
    }

    // fn try_edge_type_key_for_index(&self, edge_type_index: &EdgeTypeIndex) -> Result<&EdgeTypeKeyRef, GraphComputingError> {
    //     todo!()
    // }

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
