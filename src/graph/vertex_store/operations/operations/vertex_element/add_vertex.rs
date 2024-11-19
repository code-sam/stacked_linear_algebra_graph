use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::GetVertexTypeIndexer;
use crate::graph::vertex_store::VertexVector;

pub(crate) trait AddVertex<'s, T>
where
    T: ValueType,
{
    fn add_new_public_vertex(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_update_public_vertex(
        &'s mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;

    fn add_new_private_vertex(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_update_private_vertex(
        &'s mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;

    fn add_new_vertex_unchecked(
        &'s mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_update_vertex_unchecked(
        &'s mut self,
        vertex_type: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;
}
