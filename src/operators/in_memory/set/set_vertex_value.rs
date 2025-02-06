use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::graph::GetVertexStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetVertexIndexIndex, GetVertexTypeIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::operators::operators::set::SetVertexValue;

impl<T> SetVertexValue<T> for Graph
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    fn set_vertex_value(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<(), GraphComputingError> {
        self.vertex_store_mut_ref()
            .set_vertex(vertex_type_index, vertex_index, value)
    }
}

#[cfg(test)]
mod tests {}
