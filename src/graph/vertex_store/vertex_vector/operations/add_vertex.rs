use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::graph::VertexIndex;
use crate::graph::graph::VertexTypeIndex;
use crate::graph::indexer::AssignedIndex;
use crate::graph::indexer::GetAssignedIndexData;
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::get_vertex_vector::GetVertexVector;
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};
use crate::graph::vertex_store::VertexVector;

use super::CreateVertexIndex;

// pub(crate) trait AddVertexIndex {
//     fn new_vertex_index(
//         &mut self,
//         type_index: &VertexTypeIndex,
//     ) -> Result<AssignedIndex, GraphComputingError>;
// }

// impl AddVertexIndex for VertexStore {
//     fn new_vertex_index(
//         &mut self,
//         type_index: &VertexTypeIndex,
//     ) -> Result<AssignedIndex, GraphComputingError> {
//         let vertex_vector: &mut VertexVector = self.vertex_vector_mut_ref(type_index)?;
//         let vertex_index = self.element_indexer_mut_ref().new_index()?;
//         Ok(vertex_index)
//     }
// }

pub(crate) trait AddVertex<T>
where
    T: ValueType,
{
    fn add_new_vertex(
        &mut self,
        type_index: &VertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError>;

    fn add_or_update_vertex(
        &mut self,
        vertex_type: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError>;
}

impl<T> AddVertex<T> for VertexStore
where
    T: ValueType + SetVectorElementTyped<T>,
{
    fn add_new_vertex(
        &mut self,
        type_index: &VertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self.new_vertex_index()?;
        let vertex_vector: &mut VertexVector = self.vertex_vector_mut_ref(type_index)?;
        T::set_value(vertex_vector, vertex_index.index_ref(), value)?;
        Ok(vertex_index)
    }

    fn add_or_update_vertex(
        &mut self,
        vertex_type_index: &VertexTypeIndex,
        vertex_index: &VertexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index)?;
        if self.element_indexer_ref().is_valid_index(vertex_index)? {
            let vertex_vector: &mut VertexVector = self.vertex_vector_mut_ref(vertex_type_index)?;
            T::set_value(vertex_vector, vertex_index, value)?;
            return Ok(None);
        } else {
            let index = self.add_new_vertex(vertex_type_index, value)?;
            return Ok(Some(index));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    use crate::graph::vertex_store::operations::add_vertex_type::AddVertexType;

    #[test]
    fn test_add_new_vertex() {
        let context = GraphblasContext::init_default().unwrap();

        let mut store = VertexStore::with_initial_capacity(&context, &0, &0).unwrap();

        let mut vertex_type_indices = Vec::new();
        for _i in 0..2 {
            vertex_type_indices.push(AddVertexType::<i32>::new_vertex_type(&mut store).unwrap());
        }

        for i in 0..50 {
            store.add_new_vertex(&vertex_type_indices[1], i).unwrap();
        }
    }
}
