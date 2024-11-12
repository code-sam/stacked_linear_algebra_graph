use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;

use crate::graph::indexing::operations::CheckIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::GetVertexVector;
use crate::graph::vertex_store::vertex_store::VertexStore;
use crate::graph::vertex_store::GetVertexElementIndexer;
use crate::graph::vertex_store::GetVertexTypeIndexer;

pub(crate) trait UpdateVertex<T: ValueType> {
    fn update_public_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn update_private_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;

    fn update_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError>;
}
