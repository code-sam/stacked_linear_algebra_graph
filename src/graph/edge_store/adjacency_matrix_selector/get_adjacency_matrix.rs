use crate::{
    error::GraphComputingError,
    graph::{
        edge_store::{
            operations::{
                get_adjacency_matrix::GetAdjacencyMatrix,
                get_adjacency_matrix_cached_attributes::GetAdjacencyMatrixCachedAttributes,
            },
            weighted_adjacency_matrix::WeightedAdjacencyMatrix,
        }, indexing::EdgeTypeIndex,
    },
};

pub(crate) fn try_adjacency_matrix_ref<'a>(
    edge_store: &'a mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    edge_type_index: &EdgeTypeIndex,
    use_cached_adjacency_matrix_transpose: bool,
    transpose_argument_by_graphblas: &mut bool,
) -> Result<&'a WeightedAdjacencyMatrix, GraphComputingError> {
    if use_cached_adjacency_matrix_transpose && *transpose_argument_by_graphblas {
        *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
        edge_store.try_transposed_adjacency_matrix_ref(edge_type_index)
    } else {
        edge_store.try_public_adjacency_matrix_ref(edge_type_index)
    }
}

pub(crate) fn adjacency_matrix_ref_unchecked<'a>(
    edge_store: &'a mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    edge_type_index: &EdgeTypeIndex,
    use_cached_adjacency_matrix_transpose: bool,
    transpose_argument_by_graphblas: &mut bool,
) -> &'a WeightedAdjacencyMatrix {
    if use_cached_adjacency_matrix_transpose && *transpose_argument_by_graphblas {
        *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
        edge_store.transposed_adjacency_matrix_ref_unchecked(edge_type_index)
    } else {
        edge_store.adjacency_matrix_ref_unchecked(edge_type_index)
    }
}

pub(crate) fn try_transposed_adjacency_matrix_ref<'a>(
    edge_store: &'a mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    edge_type_index: &EdgeTypeIndex,
    use_cached_adjacency_matrix_transpose: bool,
    transpose_argument_by_graphblas: &mut bool,
) -> Result<&'a WeightedAdjacencyMatrix, GraphComputingError> {
    if use_cached_adjacency_matrix_transpose && !*transpose_argument_by_graphblas {
        edge_store.try_transposed_adjacency_matrix_ref(edge_type_index)
    } else {
        *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
        edge_store.try_public_adjacency_matrix_ref(edge_type_index)
    }
}

pub(crate) fn transposed_adjacency_matrix_ref_unchecked<'a>(
    edge_store: &'a mut (impl GetAdjacencyMatrix + GetAdjacencyMatrixCachedAttributes),
    edge_type_index: &EdgeTypeIndex,
    use_cached_adjacency_matrix_transpose: bool,
    transpose_argument_by_graphblas: &mut bool,
) -> &'a WeightedAdjacencyMatrix {
    // match (
    //     use_cached_adjacency_matrix_transpose,
    //     *transpose_argument_by_graphblas,
    // ) {
    //     (false, false) => {
    //         *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
    //         edge_store.adjacency_matrix_ref_unchecked(edge_type_index)
    //     }
    //     (false, true) => {
    //         *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
    //         edge_store.adjacency_matrix_ref_unchecked(edge_type_index)
    //     }
    //     (true, false) => edge_store.transposed_adjacency_matrix_ref_unchecked(edge_type_index),
    //     (true, true) => {
    //         *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
    //         edge_store.adjacency_matrix_ref_unchecked(edge_type_index)
    //     }
    // }

    if use_cached_adjacency_matrix_transpose && !*transpose_argument_by_graphblas {
        edge_store.transposed_adjacency_matrix_ref_unchecked(edge_type_index)
    } else {
        *transpose_argument_by_graphblas = !*transpose_argument_by_graphblas;
        edge_store.adjacency_matrix_ref_unchecked(edge_type_index)
    }
}
