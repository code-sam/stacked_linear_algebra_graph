use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetSparseVectorElementTyped;

use crate::error::GraphComputingError;
use crate::graph::indexing::traits::CheckIndex;
use crate::graph::indexing::AssignedIndex;
use crate::graph::indexing::GetAssignedIndexData;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::indexing::GetVertexTypeIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::AddVertex;
use crate::graph::vertex_store::operations::vertex_element::CreateVertexIndex;
use crate::graph::vertex_store::operations::vertex_element::SetVertex;
use crate::graph::vertex_store::operations::vertex_type::GetVertexVector;
use crate::graph::vertex_store::vertex_store::VertexStore;
use crate::graph::vertex_store::GetVertexElementIndexer;
use crate::graph::vertex_store::GetVertexTypeIndexer;
use crate::graph::vertex_store::VertexVector;

impl<'s, T> AddVertex<T> for VertexStore
where
    T: ValueType + SetSparseVectorElementTyped<T>,
{
    fn add_new_vertex(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(type_index.index())?;
        self.add_new_vertex_unchecked(type_index, value)
    }

    fn add_or_set_vertex(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        self.vertex_type_indexer_ref()
            .try_index_validity(vertex_type_index.index())?;
        self.add_or_set_vertex_unchecked(vertex_type_index, vertex_index, value)
    }

    fn add_new_vertex_unchecked(
        &mut self,
        type_index: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<AssignedIndex, GraphComputingError> {
        let vertex_index = self.new_vertex_index()?;
        let vertex_vector: &mut VertexVector = self.vertex_vector_mut_ref_unchecked(type_index)?;
        T::set_graphblas_vector_value(vertex_vector, vertex_index.index(), value)?;
        Ok(vertex_index)
    }

    fn add_or_set_vertex_unchecked(
        &mut self,
        vertex_type_index: &impl GetVertexTypeIndex,
        vertex_index: &impl GetVertexIndexIndex,
        value: T,
    ) -> Result<Option<AssignedIndex>, GraphComputingError> {
        if self
            .element_indexer_ref()
            .is_valid_index(vertex_index.index())?
        {
            self.set_vertex_unchecked(vertex_type_index, vertex_index, value)?;
            return Ok(None);
        } else {
            let index = self.add_new_vertex_unchecked(vertex_type_index, value)?;
            return Ok(Some(index));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::indexing::VertexIndex;
    use crate::graph::vertex_store::operations::vertex_element::GetVertexValue;
    use crate::graph::vertex_store::operations::vertex_type::AddVertexType;

    use super::*;

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    #[test]
    fn test_add_new_vertex() {
        let context = GraphblasContext::init_default().unwrap();

        let mut store = VertexStore::with_initial_capacity(context, 0, 0).unwrap();

        let mut vertex_type_indices = Vec::new();
        for _i in 0..2 {
            vertex_type_indices.push(AddVertexType::<i32>::apply(&mut store).unwrap());
        }

        for i in 0..50 {
            store.add_new_vertex(&vertex_type_indices[1], i).unwrap();
        }
    }

    #[test]
    fn test_vertex_value_type_casting() {
        let context = GraphblasContext::init_default().unwrap();

        let mut store = VertexStore::with_initial_capacity(context, 10, 10).unwrap();

        let vertex_type_index_bool = AddVertexType::<bool>::apply(&mut store).unwrap();
        let vertex_type_index_u8 = AddVertexType::<u8>::apply(&mut store).unwrap();

        let vertex_index_10_as_bool = VertexIndex::new(
            store
                .add_new_vertex(&vertex_type_index_bool, 10)
                .unwrap()
                .index(),
        );
        let vertex_index_minus_1_as_u8 = VertexIndex::new(
            store
                .add_new_vertex(&vertex_type_index_u8, -1)
                .unwrap()
                .index(),
        );
        let vertex_index_f1000_as_u8 = VertexIndex::new(
            store
                .add_new_vertex(&vertex_type_index_u8, 1000.0f32)
                .unwrap()
                .index(),
        );

        assert_eq!(
            true,
            GetVertexValue::<bool>::vertex_value(
                &store,
                &vertex_type_index_bool,
                &vertex_index_10_as_bool
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            10,
            GetVertexValue::<i32>::vertex_value(
                &store,
                &vertex_type_index_bool,
                &vertex_index_10_as_bool
            )
            .unwrap()
            .unwrap()
        );

        assert_eq!(
            u8::MAX,
            GetVertexValue::<u8>::vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_minus_1_as_u8
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            -1,
            GetVertexValue::<i32>::vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_minus_1_as_u8
            )
            .unwrap()
            .unwrap()
        );

        assert_eq!(
            u8::MAX,
            GetVertexValue::<u8>::vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_f1000_as_u8
            )
            .unwrap()
            .unwrap()
        );
        assert_ne!(
            1000.0f32,
            GetVertexValue::<f32>::vertex_value(
                &store,
                &vertex_type_index_u8,
                &vertex_index_f1000_as_u8
            )
            .unwrap()
            .unwrap()
        );
    }
}
