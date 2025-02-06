use crate::{
    error::GraphComputingError,
    graph::edge_store::adjacency_matrix_with_cached_attributes::WeightedAdjacencyMatrixWithCachedAttributes,
};

pub(crate) trait MapAdjacencyMatricesWithCachedAttributes<MappingFunction> {
    fn map_all_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError>;

    fn map_all_valid_adjacency_matrices(
        &self,
        function_to_apply: MappingFunction,
    ) -> Result<(), GraphComputingError>;
}

pub(crate) trait MapMutableAdjacencyMatrices<F> {
    fn map_mut_all_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;

    fn map_mut_all_valid_adjacency_matrices(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>;
}

// pub(crate) trait MapMutableAdjacencyMatrices {
//     fn map_mut_all_adjacency_matrices(
//         &mut self,
//         function_to_apply: &(dyn Fn(&mut WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
//         + Send
//         + Sync),
//     ) -> Result<(), GraphComputingError>;

//     fn map_mut_all_valid_adjacency_matrices(
//         &mut self,
//         function_to_apply: &(dyn Fn(&mut WeightedAdjacencyMatrixWithCachedAttributes) -> Result<(), GraphComputingError>
//         + Send
//         + Sync),
//     ) -> Result<(), GraphComputingError>;
// }
