use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
    GetSparseVectorElementValueTyped, SetSparseVectorElementTyped,
};

use crate::error::GraphComputingError;

use crate::graph::indexing::{GetVertexTypeIndex, VertexIndex};
use crate::graph::value_type::ValueType;
use crate::graph::vertex_store::traits::in_memory_transaction::transaction::GetSparseVectorStateRevertersByVertexTypeMap;
use crate::graph_operators::in_memory::new::new_vertex;
use crate::graph_operators::operator_traits::new::NewVertex;
use crate::transaction::in_memory::InMemoryGraphTransaction;

impl<'g, T> NewVertex<T> for InMemoryGraphTransaction<'g>
where
    T: ValueType
        + SetSparseVectorElementTyped<T>
        + Copy
        + Default
        + GetSparseVectorElementValueTyped<T>
        + GetSparseVectorStateRevertersByVertexTypeMap<T>,
{
    fn new_vertex(
        &mut self,
        vertex_type: &impl GetVertexTypeIndex,
        value: T,
    ) -> Result<VertexIndex, GraphComputingError> {
        new_vertex(
            &mut self.vertex_store_transaction,
            &mut self.edge_store_transaction,
            vertex_type,
            value,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::graph::edge_store::GetAdjacencyMatrices;
    use crate::graph::graph::{GetEdgeStore, GetVertexStore, Graph};
    use crate::graph::indexing::GetIndexCapacity;
    use crate::graph::vertex_store::GetVertexElementIndexer;
    use crate::graph_operators::operator_traits::{new::NewVertexType, read::GetVertexValue};

    #[test]
    fn add_vertex() {
        let mut graph = Graph::with_initial_capacity(1, 5, 5).unwrap();

        let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_value = 1u8;
        let another_vertex_value = 2u8;

        let vertex_index = graph.new_vertex(&vertex_type_index, vertex_value).unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index)
            .unwrap();
        assert_eq!(value, vertex_value);

        let vertex_index_2 = graph
            .new_vertex(&vertex_type_index, another_vertex_value)
            .unwrap();

        let value: u8 = graph
            .try_vertex_value(&vertex_type_index, &vertex_index_2)
            .unwrap();
        assert_eq!(value, another_vertex_value);
    }

    #[test]
    fn add_new_vertex() {
        let mut graph = Graph::with_initial_capacity(1, 1, 1).unwrap();

        for _i in 0..3 {
            let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

            for i in 0..50 {
                graph.new_vertex(&vertex_type_index, i).unwrap();
            }
        }

        assert_eq!(
            graph.edge_store_ref().adjacency_matrix_size(),
            graph
                .vertex_store_ref()
                .element_indexer_ref()
                .capacity()
                .unwrap()
        )
    }
}
