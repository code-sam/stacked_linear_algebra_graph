use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::error::GraphComputingError;

use crate::graph::edge::{
    AdjacencyMatrixCoordinate, DirectedEdgeCoordinateDefinedByIndices,
    DirectedEdgeCoordinateDefinedByIndicesTrait, DirectedEdgeCoordinateDefinedByKeys,
    DirectedEdgeCoordinateDefinedByKeysTrait,
};
use crate::graph::edge_store::operations::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::weighted_adjacency_matrix::operations::ReadEdge;
use crate::graph::edge_store::EdgeStoreTrait;
use crate::graph::graph::{Graph, GraphTrait};
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::implement_macro_for_all_native_value_types;
use crate::graph::vertex_store::VertexStoreTrait;

use super::Indexing;

pub trait ReadEdgeWeight<T: ValueType> {
    fn index_defined_edge_weight(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<Option<T>, GraphComputingError>;
    fn key_defined_edge_weight(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<Option<T>, GraphComputingError>;

    // These still require valid indices
    fn index_defined_edge_weight_or_default(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<T, GraphComputingError>;
    fn key_defined_edge_weight_or_default(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<T, GraphComputingError>;

    fn try_index_defined_edge_weight(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
    ) -> Result<T, GraphComputingError>;
    fn try_key_defined_edge_weight(
        &self,
        edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
    ) -> Result<T, GraphComputingError>;
}

macro_rules! implement_read_edge_weight {
    ($value_type: ty) => {
        impl ReadEdgeWeight<$value_type> for Graph {
            fn index_defined_edge_weight(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                self.try_index_defined_edge_coordinate_validity(edge_coordinate)?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_coordinate.edge_type_ref())
                    .edge_weight_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
            }

            fn key_defined_edge_weight(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
            ) -> Result<Option<$value_type>, GraphComputingError> {
                let edge_type_index = self
                    .edge_store_ref()
                    .edge_type_indexer_ref()
                    .try_index_for_key(edge_coordinate.edge_type_ref())?;
                let tail_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.tail_ref())?;
                let head_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.head_ref())?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_type_index)
                    .edge_weight_unchecked(&AdjacencyMatrixCoordinate::new(
                        *tail_index,
                        *head_index,
                    ))
            }

            fn try_index_defined_edge_weight(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
            ) -> Result<$value_type, GraphComputingError> {
                self.try_index_defined_edge_coordinate_validity(edge_coordinate)?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_coordinate.edge_type_ref())
                    .try_edge_weight_unchecked(&edge_coordinate.adjacency_matrix_coordinate())
            }

            fn try_key_defined_edge_weight(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
            ) -> Result<$value_type, GraphComputingError> {
                let edge_type_index = self
                    .edge_store_ref()
                    .edge_type_indexer_ref()
                    .try_index_for_key(edge_coordinate.edge_type_ref())?;
                let tail_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.tail_ref())?;
                let head_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.head_ref())?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_type_index)
                    .try_edge_weight_unchecked(&AdjacencyMatrixCoordinate::new(
                        *tail_index,
                        *head_index,
                    ))
            }

            /// Requires valid coordinate
            fn index_defined_edge_weight_or_default(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByIndices,
            ) -> Result<$value_type, GraphComputingError> {
                self.try_index_defined_edge_coordinate_validity(edge_coordinate)?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_coordinate.edge_type_ref())
                    .edge_weight_or_default_unchecked(
                        &edge_coordinate.adjacency_matrix_coordinate(),
                    )
            }

            /// Requires valid coordinate
            fn key_defined_edge_weight_or_default(
                &self,
                edge_coordinate: &DirectedEdgeCoordinateDefinedByKeys,
            ) -> Result<$value_type, GraphComputingError> {
                let edge_type_index = self
                    .edge_store_ref()
                    .edge_type_indexer_ref()
                    .try_index_for_key(edge_coordinate.edge_type_ref())?;
                let tail_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.tail_ref())?;
                let head_index = self
                    .vertex_store_ref()
                    .element_indexer_ref()
                    .try_index_for_key(edge_coordinate.head_ref())?;
                self.edge_store_ref()
                    .adjacency_matrix_ref_for_index_unchecked(edge_type_index)
                    .edge_weight_or_default_unchecked(&AdjacencyMatrixCoordinate::new(
                        *tail_index,
                        *head_index,
                    ))
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_read_edge_weight);

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::tests::standard_graph_for_testing::standard_graph_for_testing;

    // #[test]
    // fn test_is_edge_type() {
    //     let graph = standard_graph_for_testing();

    //     assert!(!graph
    //         .is_edge_type_in_graph(String::from("this_edge_type_does_not_exist").as_str())
    //         .unwrap());
    //     assert!(graph
    //         .is_edge_type_in_graph(String::from("is_a").as_str())
    //         .unwrap());
    // }
}
