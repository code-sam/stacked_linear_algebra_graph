use graphblas_sparse_linear_algebra::{
    collections::{
        sparse_matrix::{GetMatrixElementList, GetMatrixElementValue},
        sparse_vector::GetVectorElementList,
    },
    value_type::ValueType,
};

use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{EdgeStore, WeightedAdjacencyMatrix, WeightedAdjacencyMatrixTrait},
        graph::VertexIndex,
        index::ElementIndex,
        value_type::implement_macro_for_all_native_value_types,
    },
};
use crate::{
    error::{LogicError, LogicErrorType},
    graph::edge::AdjacencyMatrixCoordinate,
};

use super::vertex_mask::VertexMasking;

pub(crate) trait Indexing {
    fn is_edge(&self, coordinate: &AdjacencyMatrixCoordinate) -> Result<bool, GraphComputingError>;
    fn try_is_edge(
        &self,
        coordinate: &AdjacencyMatrixCoordinate,
    ) -> Result<(), GraphComputingError>;

    fn adjacency_matrix_coordinates(
        &self,
    ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError>;
    fn indices_of_vertices_with_outgoing_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;
    fn indices_of_vertices_with_incoming_edges(
        &self,
    ) -> Result<Vec<VertexIndex>, GraphComputingError>;

    ///
    fn indices_of_connected_vertices(&self) -> Result<Vec<VertexIndex>, GraphComputingError>;
}

macro_rules! implement_indexing {
    ($value_type:ty) => {
        impl Indexing for WeightedAdjacencyMatrix<$value_type> {
            fn is_edge(
                &self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<bool, GraphComputingError> {
                match self.sparse_matrix_ref().get_element_value(coordinate)? {
                    Some(_) => Ok(true),
                    None => Ok(false),
                }
            }

            fn try_is_edge(
                &self,
                coordinate: &AdjacencyMatrixCoordinate,
            ) -> Result<(), GraphComputingError> {
                match self.sparse_matrix_ref().get_element_value(coordinate)? {
                    Some(_) => Ok(()),
                    None => Err(LogicError::new(
                        LogicErrorType::EdgeMustExist,
                        format!("No edge at coordinate {:?}", coordinate),
                        None,
                    )
                    .into()),
                }
            }

            fn adjacency_matrix_coordinates(
                &self,
            ) -> Result<Vec<AdjacencyMatrixCoordinate>, GraphComputingError> {
                let matrix_element_list = self.sparse_matrix_ref().get_element_list()?;
                let element_indices_vertices_with_outgoing_edges =
                    matrix_element_list.row_indices_ref();
                let element_indices_vertices_with_incoming_edges =
                    matrix_element_list.column_indices_ref();

                let mut coordinates: Vec<AdjacencyMatrixCoordinate> =
                    Vec::with_capacity(matrix_element_list.length());
                for element_index in 0..matrix_element_list.length() {
                    let element_coordinate = AdjacencyMatrixCoordinate::new(
                        element_indices_vertices_with_outgoing_edges[element_index],
                        element_indices_vertices_with_incoming_edges[element_index],
                    );
                    coordinates.push(element_coordinate);
                }
                Ok(coordinates)
            }

            fn indices_of_vertices_with_outgoing_edges(
                &self,
            ) -> Result<Vec<VertexIndex>, GraphComputingError> {
                Ok(self
                    .mask_vertices_with_outgoing_edges()?
                    .get_element_list()?
                    .indices_ref()
                    .to_vec())
            }

            fn indices_of_vertices_with_incoming_edges(
                &self,
            ) -> Result<Vec<VertexIndex>, GraphComputingError> {
                Ok(self
                    .mask_vertices_with_incoming_edges()?
                    .get_element_list()?
                    .indices_ref()
                    .to_vec())
            }

            ///
            fn indices_of_connected_vertices(
                &self,
            ) -> Result<Vec<VertexIndex>, GraphComputingError> {
                Ok(self
                    .mask_connected_vertices()?
                    .get_element_list()?
                    .indices_ref()
                    .to_vec())
            }
        }
    };
}
implement_macro_for_all_native_value_types!(implement_indexing);
