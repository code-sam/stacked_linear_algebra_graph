use graphblas_sparse_linear_algebra::operators::mask::SelectEntireMatrix;

use crate::error::GraphComputingError;
use crate::graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use crate::graph::indexing::{ElementCount, GetEdgeTypeIndex};

pub(crate) trait GetAdjacencyMatrix {
    fn adjacency_matrix_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix, GraphComputingError>;
    fn adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix;
    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix, GraphComputingError>;

    fn adjacency_matrix_size_ref(&self) -> &ElementCount;
    fn mask_to_select_entire_adjacency_matrix_ref(&self) -> &SelectEntireMatrix;
}

pub(crate) trait GetAdjacencyMatrixWithCachedAttributes {
    fn adjacency_matrix_with_cached_attributes_ref(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;
    fn adjacency_matrix_with_cached_attributes_mut_ref(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;

    fn adjacency_matrix_with_cached_attributes_ref_unchecked(
        &self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrixWithCachedAttributes;
    fn adjacency_matrix_with_cached_attributes_mut_ref_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrixWithCachedAttributes, GraphComputingError>;
}

// pub(crate) fn vertex_vector_ref<'s>(
//     vertex_store: &'s VertexStore,
//     vertex_type_index: &impl GetVertexTypeIndex,
// ) -> Result<&'s VertexVector, GraphComputingError> {
//     vertex_store
//         .vertex_type_indexer_ref()
//         .try_index_validity(vertex_type_index.index())?;
//     Ok(vertex_store.vertex_vector_ref_unchecked(vertex_type_index))
// }

// pub(crate) fn vertex_vector_mut_ref<'s>(
//     vertex_store: &'s mut VertexStore,
//     vertex_type_index: &impl GetVertexTypeIndex,
// ) -> Result<&'s mut VertexVector, GraphComputingError> {
//     vertex_store
//         .vertex_type_indexer_ref()
//         .try_index_validity(vertex_type_index.index())?;
//     Ok(vertex_store.vertex_vector_mut_ref_unchecked(vertex_type_index)?)
// }

// pub(crate) fn vertex_vector_ref_unchecked<'s>(
//     vertex_store: &'s VertexStore,
//     vertex_type_index: &impl GetVertexTypeIndex,
// ) -> &'s VertexVector {
//     &vertex_store.vertex_vector_for_all_vertex_types_ref()[*vertex_type_index.index_ref()]
// }

// pub(crate) fn vertex_vector_mut_ref_unchecked<'s>(
//     vertex_store: &'s mut VertexStore,
//     vertex_type_index: &impl GetVertexTypeIndex,
// ) -> &'s mut VertexVector {
//     &mut vertex_store.vertex_vector_for_all_vertex_types_mut_ref()[*vertex_type_index.index_ref()]
// }
