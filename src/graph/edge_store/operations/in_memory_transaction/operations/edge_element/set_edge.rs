use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::GetSparseMatrixElementValueTyped;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;

use crate::graph::edge::GetDirectedEdgeCoordinateIndex;
use crate::graph::edge::GetEdgeWeight;
use crate::graph::edge_store::operations::in_memory_transaction::GetEdgeStore;
use crate::graph::edge_store::operations::in_memory_transaction::InMemoryEdgeStoreTransaction;
use crate::graph::edge_store::operations::in_memory_transaction::RegisterEdgeWeightToRestore;
use crate::graph::edge_store::operations::operations::edge_element::Indexing;
use crate::graph::edge_store::operations::operations::edge_element::SetEdge;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrixWithCachedAttributes;
use crate::graph::indexing::GetEdgeTypeIndex;
use crate::graph::indexing::GetVertexIndexIndex;
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::graph::edge_store::operations::in_memory_transaction::edge_store_state_restorer::adjacency_matrices_state_restorer::adjacency_matrices_state_restorer::GetAdjacencyMatrixStateRevertersByEdgeTypeMap;

impl<'s, T> SetEdge<T> for InMemoryEdgeStoreTransaction<'s>
where
    T: ValueType
        + Copy
        + Default
        + GetSparseMatrixElementValueTyped<T>
        + GetAdjacencyMatrixStateRevertersByEdgeTypeMap<T>
        + SetSparseMatrixElementTyped<T>,
{
    fn set_weighted_directed_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.set_edge(
            vertex_indexer,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn set_edge(
        &mut self,
        vertex_indexer: &impl CheckVertexIndex,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.try_is_valid_edge(vertex_indexer, edge_type_index, tail, head)?;

        self.set_edge_unchecked(edge_type_index, tail, head, weight)
    }

    fn set_weighted_directed_edge_unchecked(
        &mut self,
        edge: &(impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>),
    ) -> Result<(), GraphComputingError> {
        self.set_edge_unchecked(
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            edge.weight(),
        )
    }

    fn set_edge_unchecked(
        &mut self,
        edge_type_index: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        RegisterEdgeWeightToRestore::register_optional_edge_weight_to_restore(
            &mut self.edge_store_state_restorer,
            self.edge_store
                .adjacency_matrix_with_cached_attributes_mut_ref_unchecked(edge_type_index),
            edge_type_index,
            tail,
            head,
        )?;

        self.edge_store_mut_ref()
            .set_edge_unchecked(edge_type_index, tail, head, weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge_store::operations::operations::edge_type::add_edge_type::AddEdgeType;
    use crate::graph::edge_store::{EdgeStore, GetEdgeTypeIndicer};
    use crate::graph::graph::GetGraphblasContext;
    use crate::graph::indexing::{GetIndexCapacity, VertexIndex};
    use crate::graph::vertex_store::operations::vertex_element::AddVertex;
    use crate::graph::vertex_store::operations::vertex_type::AddVertexType;
    use crate::graph::vertex_store::{GetVertexTypeIndexer, VertexStore};

    use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

    #[test]
    fn test_rollback_set_edge() {
        let mut vertex_store = initialize_vertex_store();

        let mut edge_store = EdgeStore::with_initial_capacity(
            vertex_store.graphblas_context(),
            vertex_store.vertex_type_indexer_ref().capacity().unwrap(),
            2,
        )
        .unwrap();

        let edge_type_index_1 = AddEdgeType::<u16>::apply(&mut edge_store).unwrap();
        let edge_type_index_2 = AddEdgeType::<u16>::apply(&mut edge_store).unwrap();

        let _ = edge_store
            .set_edge(
                &vertex_store,
                &edge_type_index_1,
                &VertexIndex::new(2),
                &VertexIndex::new(3),
                0u16,
            )
            .unwrap();

        {
            let mut transaction = InMemoryEdgeStoreTransaction::new(&mut edge_store).unwrap();

            let edge_type_index_3 = AddEdgeType::<u16>::apply(&mut transaction).unwrap();

            let _ = transaction
                .set_edge(
                    &vertex_store,
                    &edge_type_index_2,
                    &VertexIndex::new(0),
                    &VertexIndex::new(1),
                    0u16,
                )
                .unwrap();

            let _ = transaction
                .set_edge(
                    &vertex_store,
                    &edge_type_index_2,
                    &VertexIndex::new(0),
                    &VertexIndex::new(1),
                    10u16,
                )
                .unwrap();

            let _ = transaction
                .set_edge(
                    &vertex_store,
                    &edge_type_index_3,
                    &VertexIndex::new(2),
                    &VertexIndex::new(3),
                    3u16,
                )
                .unwrap();
        }

        assert_eq!(edge_store.edge_type_indexer_ref().capacity().unwrap(), 2);
        assert!(edge_store
            .is_edge(
                &edge_type_index_1,
                &VertexIndex::new(2),
                &VertexIndex::new(3),
            )
            .unwrap());

        assert!(!edge_store
            .is_edge(
                &edge_type_index_2,
                &VertexIndex::new(0),
                &VertexIndex::new(1),
            )
            .unwrap())
    }

    fn initialize_vertex_store() -> VertexStore {
        let context = GraphblasContext::init_default().unwrap();

        let mut vertex_store = VertexStore::with_initial_capacity(context, 0, 0).unwrap();

        let mut public_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            public_vertex_type_indices
                .push(AddVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_vertex(&public_vertex_type_indices[1], i)
                .unwrap();
        }

        let mut private_vertex_type_indices = Vec::new();
        for _i in 0..3 {
            private_vertex_type_indices
                .push(AddVertexType::<i32>::apply(&mut vertex_store).unwrap());
        }

        for i in 0..5 {
            vertex_store
                .add_new_vertex(&private_vertex_type_indices[1], i)
                .unwrap();
        }

        vertex_store
    }
}
