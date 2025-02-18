use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::SetSparseMatrixElementTyped;

use crate::error::GraphComputingError;
use crate::graph::edge::{GetDirectedEdgeCoordinateIndex, GetEdgeWeight};
use crate::graph::edge_store::operations::operations::edge_element::NewEdge as NewEdgeInEdgeStore;
use crate::graph::graph::Graph;
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex};
use crate::graph::value_type::ValueType;
use crate::graph_operators::operator_traits::new::NewEdge;

impl<T> NewEdge<T> for Graph
where
    T: ValueType + SetSparseMatrixElementTyped<T> + Copy,
{
    fn new_edge_from_edge(
        &mut self,
        edge: impl GetDirectedEdgeCoordinateIndex + GetEdgeWeight<T>,
    ) -> Result<(), GraphComputingError> {
        NewEdge::<T>::new_edge(
            self,
            edge.edge_type_ref(),
            edge.tail_ref(),
            edge.head_ref(),
            *edge.weight_ref(),
        )
    }

    fn new_edge(
        &mut self,
        edge_type: &impl GetEdgeTypeIndex,
        tail: &impl GetVertexIndexIndex,
        head: &impl GetVertexIndexIndex,
        weight: T,
    ) -> Result<(), GraphComputingError> {
        self.public_edge_store
            .new_edge(&self.public_vertex_store, edge_type, tail, head, weight)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::{DirectedEdgeCoordinate, WeightedDirectedEdge};
    use crate::graph::graph::Graph;
    use crate::graph::vertex::{GetVertexIndex, VertexDefinition};
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertexIndex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetEdgeWeight;

    #[test]
    fn add_edge() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_type_index_2 = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex1_index = graph.new_vertex_index().unwrap();
        let vertex2_index = graph.new_vertex_index().unwrap();

        let vertex_1 = VertexDefinition::new(vertex_type_index, vertex1_index, 1u8);
        let vertex_2 = VertexDefinition::new(vertex_type_index, vertex2_index, 2u8);

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();

        let edge_vertex1_vertex2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_1_index,
                *vertex_1.index_ref(),
                *vertex_2.index_ref(),
            ),
            1u8,
        );
        let edge_vertex2_vertex1 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_1_index,
                *vertex_2.index_ref(),
                *vertex_1.index_ref(),
            ),
            2u8,
        );

        let edge_vertex1_vertex2_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_2_index,
                *vertex_1.index_ref(),
                *vertex_2.index_ref(),
            ),
            3u8,
        );
        let edge_vertex2_vertex1_type_2 = WeightedDirectedEdge::new(
            DirectedEdgeCoordinate::new(
                edge_type_2_index,
                *vertex_2.index_ref(),
                *vertex_1.index_ref(),
            ),
            4u8,
        );

        graph.new_edge_from_edge(edge_vertex1_vertex2).unwrap();
        graph.new_edge_from_edge(edge_vertex2_vertex1).unwrap();
        graph
            .new_edge_from_edge(edge_vertex1_vertex2_type_2)
            .unwrap();
        graph
            .new_edge_from_edge(edge_vertex2_vertex1_type_2)
            .unwrap();

        assert_eq!(
            GetEdgeWeight::<u8>::try_edge_weight(
                &graph,
                &edge_type_1_index,
                vertex_1.index_ref(),
                vertex_2.index_ref()
            )
            .unwrap(),
            1u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_1_index,
                    *vertex_2.index_ref(),
                    *vertex_1.index_ref()
                )
            )
            .unwrap(),
            2u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_2_index,
                    *vertex_1.index_ref(),
                    *vertex_2.index_ref()
                )
            )
            .unwrap(),
            3u8
        );
        assert_eq!(
            GetEdgeWeight::<u8>::try_edge_weight_for_coordinate(
                &graph,
                &DirectedEdgeCoordinate::new(
                    edge_type_2_index,
                    *vertex_2.index_ref(),
                    *vertex_1.index_ref()
                )
            )
            .unwrap(),
            4u8
        );
    }
}
