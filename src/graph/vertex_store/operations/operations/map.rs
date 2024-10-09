use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::operations::{GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices},
        vertex_store::{GetVertexTypeIndexer, GetVertexVectors, VertexStore, VertexVector},
    },
};

pub(crate) trait MapAllVertexVectors {
    fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
}

pub(crate) trait MapValidVertexVectors {
    // fn map_all_valid_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
}

pub(crate) trait MapPublicVertexVectors {
    // fn map_all_valid_public_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_valid_public_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
}

pub(crate) trait MapPrivateVertexVectors {
    // fn map_all_valid_private_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

    fn map_mut_all_valid_private_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
}
