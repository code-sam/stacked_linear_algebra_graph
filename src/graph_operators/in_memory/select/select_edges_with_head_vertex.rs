use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix::GetAdjacencyMatrix;
use crate::graph::edge_store::operations::operations::edge_type::get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes;
use crate::graph::edge_store::operations::operations::edge_type::indexing::Indexing as EdgeTypeIndexing;
use crate::graph::edge_store::{
    ArgumentsForAdjacencyMatrixOperator, CreateArgumentsForAdjacencyMatrixOperator,
    GetArgumentsForAdjacencyMatrixOperator,
};
use crate::graph::indexing::{GetEdgeTypeIndex, GetVertexIndexIndex, GetVertexTypeIndex, VertexTypeIndex};
use crate::graph::vertex_store::operations::vertex_type::{CheckVertexTypeIndex, GetVertexVector};
use crate::graph_operators::operator_traits::select::{
    SelectEdgesWithHeadVertex, SelectEdgesWithHeadVertexUnchecked,
};
use crate::operator_options::OptionsForOperatorWithAdjacencyMatrixArgument;
use graphblas_sparse_linear_algebra::index::ElementIndexSelector as VertexSelector;
use graphblas_sparse_linear_algebra::operators::binary_operator::AccumulatorBinaryOperator;
use graphblas_sparse_linear_algebra::operators::extract::ExtractMatrixColumn;

use crate::graph::graph::GetGraphblasOperatorAppliers;
use crate::graph::graph::{Graph, GraphblasOperatorApplierCollection};
use crate::graph::vertex_store::operations::vertex_element::CheckVertexIndex;
use crate::{error::GraphComputingError, graph::value_type::ValueType};

impl<EvaluationDomain> SelectEdgesWithHeadVertex<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        head_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_edges_with_head_vertex::<EvaluationDomain>(
            &mut self.public_edge_store,
            &mut self.public_vertex_store,
            adjacency_matrix,
            head_vertex,
            accumlator,
            extract_to,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

impl<EvaluationDomain> SelectEdgesWithHeadVertexUnchecked<EvaluationDomain> for Graph
where
    EvaluationDomain: ValueType,
{
    fn apply(
        &mut self,
        adjacency_matrix: &impl GetEdgeTypeIndex,
        head_vertex: &impl GetVertexIndexIndex,
        accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
        extract_to: &impl GetVertexTypeIndex,
        mask: Option<&VertexTypeIndex>,
        options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    ) -> Result<(), GraphComputingError> {
        select_edges_with_head_vertex_unchecked::<EvaluationDomain>(
            &mut self.public_edge_store,
            &mut self.public_vertex_store,
            adjacency_matrix,
            head_vertex,
            accumlator,
            extract_to,
            mask,
            options,
            &self.graphblas_operator_applier_collection,
        )
    }
}

pub(crate) fn select_edges_with_head_vertex<EvaluationDomain>(
    edge_store: &mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes + EdgeTypeIndexing),
    vertex_store: &mut (impl GetVertexVector + CheckVertexTypeIndex + CheckVertexIndex),
    adjacency_matrix: &impl GetEdgeTypeIndex,
    head_vertex: &impl GetVertexIndexIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    extract_to: &impl GetVertexTypeIndex,
    mask: Option<&VertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    edge_store.try_edge_type_index_validity(adjacency_matrix)?;
    vertex_store.try_vertex_index_validity(head_vertex)?;
    vertex_store.try_vertex_type_index_validity(extract_to)?;
    vertex_store.try_optional_vertex_type_index_validity(mask)?;

    select_edges_with_head_vertex_unchecked::<EvaluationDomain>(
        edge_store,
        vertex_store,
        adjacency_matrix,
        head_vertex,
        accumlator,
        extract_to,
        mask,
        options,
        graphblas_operator_applier_collection,
    )
}

pub(crate) fn select_edges_with_head_vertex_unchecked<EvaluationDomain>(
    edge_store: *mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    vertex_store: *mut impl GetVertexVector,
    adjacency_matrix: &impl GetEdgeTypeIndex,
    head_vertex: &impl GetVertexIndexIndex,
    accumlator: &impl AccumulatorBinaryOperator<EvaluationDomain>,
    extract_to: &impl GetVertexTypeIndex,
    mask: Option<&impl GetVertexTypeIndex>,
    options: &OptionsForOperatorWithAdjacencyMatrixArgument,
    graphblas_operator_applier_collection: &GraphblasOperatorApplierCollection,
) -> Result<(), GraphComputingError>
where
    EvaluationDomain: ValueType,
{
    let adjacency_matrix_argument = ArgumentsForAdjacencyMatrixOperator::create_unchecked(
        edge_store,
        adjacency_matrix,
        options,
    );

    let vertex_vector_extract_to =
        unsafe { &mut *vertex_store }.vertex_vector_mut_ref_unchecked(extract_to)?;

    match mask {
        Some(mask) => {
            let vertex_vector_mask = unsafe { &*vertex_store }.vertex_vector_ref_unchecked(mask);

            Ok(graphblas_operator_applier_collection
                .matrix_column_extractor()
                .apply(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    head_vertex.index(),
                    &VertexSelector::All,
                    accumlator,
                    vertex_vector_extract_to,
                    vertex_vector_mask,
                    adjacency_matrix_argument.options_ref(),
                )?)
        }
        None => {
            let vertex_vector_mask = graphblas_operator_applier_collection.entire_vector_selector();

            Ok(graphblas_operator_applier_collection
                .matrix_column_extractor()
                .apply(
                    adjacency_matrix_argument.adjacency_matrix_ref(),
                    head_vertex.index(),
                    &VertexSelector::All,
                    accumlator,
                    vertex_vector_extract_to,
                    vertex_vector_mask,
                    adjacency_matrix_argument.options_ref(),
                )?)
        }
    }
}

#[cfg(test)]
mod tests {
    use graphblas_sparse_linear_algebra::operators::binary_operator::{Assignment, Plus};

    use super::*;

    use crate::graph::indexing::VertexTypeIndex;
    use crate::graph_operators::operator_traits::new::{
        NewEdge, NewEdgeType, NewVertex, NewVertexType,
    };
    use crate::graph_operators::operator_traits::read::GetVertexValue;

    #[test]
    fn select_edges_with_head_vertex() {
        let mut graph = Graph::with_initial_capacity(5, 5, 5).unwrap();

        let vertex_value_1 = 1u8;
        let vertex_value_2 = 2u8;

        let edge_vertex1_vertex2_value = 1u8;
        let edge_vertex2_vertex1_value = 2u8;
        let edge_vertex1_vertex2_type_2_value = 3u32;

        let vertex_type_1_index = NewVertexType::<u8>::apply(&mut graph).unwrap();
        let vertex_result_type_index = NewVertexType::<u8>::apply(&mut graph).unwrap();

        let vertex_1_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_1.clone())
            .unwrap();
        let vertex_2_index = graph
            .new_vertex(&vertex_type_1_index, vertex_value_2.clone())
            .unwrap();

        let edge_type_1_index = NewEdgeType::<u8>::apply(&mut graph).unwrap();
        let edge_type_2_index = NewEdgeType::<u16>::apply(&mut graph).unwrap();
        let _result_edge_type_index = NewEdgeType::<f32>::apply(&mut graph).unwrap();

        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_1_index,
                &vertex_2_index,
                &vertex_1_index,
                edge_vertex2_vertex1_value,
            )
            .unwrap();
        graph
            .new_edge(
                &edge_type_2_index,
                &vertex_1_index,
                &vertex_2_index,
                edge_vertex1_vertex2_type_2_value,
            )
            .unwrap();

        SelectEdgesWithHeadVertex::<isize>::apply(
            &mut graph,
            &edge_type_1_index,
            &vertex_2_index,
            // &VertexSelector::All,
            &Plus::<isize>::new(),
            &vertex_result_type_index,
            None::<&VertexTypeIndex>,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        assert_eq!(
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_1_index,
            )
            .unwrap(),
            Some(1)
        );

        SelectEdgesWithHeadVertex::<isize>::apply(
            &mut graph,
            &edge_type_1_index,
            &vertex_1_index,
            // &VertexSelector::All,
            &Assignment::new(),
            &vertex_result_type_index,
            None::<&VertexTypeIndex>,
            &OptionsForOperatorWithAdjacencyMatrixArgument::new_default(),
        )
        .unwrap();

        // println!(
        //     "{:?}",
        //     ReadAdjacencyMatrixElementList::<u8>::with_key(&graph, edge_type_1_key).unwrap()
        // );
        // println!(
        //     "{:?}",
        //     ReadVertexVectorElementList::<isize>::with_key(&graph, vertex_result_type_key).unwrap()
        // );
        assert_eq!(
            GetVertexValue::<isize>::vertex_value(
                &graph,
                &vertex_result_type_index,
                &vertex_2_index,
            )
            .unwrap(),
            Some(2)
        );
    }
}
