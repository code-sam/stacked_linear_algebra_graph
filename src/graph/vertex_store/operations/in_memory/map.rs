use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::operations::{GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices},
        vertex_store::{
            operations::{
                MapAllVertexVectors, MapPrivateVertexVectors, MapPublicVertexVectors,
                MapValidVertexVectors,
            },
            GetVertexTypeIndexer, GetVertexVectors, VertexStore, VertexVector,
        },
    },
};

impl MapAllVertexVectors for VertexStore {
    fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
    where
        F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.vertex_vector_for_all_vertex_types_ref()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }

    fn map_mut_all_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.vertex_vector_for_all_vertex_types_mut_ref()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }
}

impl MapValidVertexVectors for VertexStore {
    // fn map_all_valid_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer_ref()
            .valid_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(&mut self.vertex_vector_for_all_vertex_types_mut_ref()[i])
            })?;
        Ok(())
    }
}

impl MapPublicVertexVectors for VertexStore {
    // fn map_all_valid_public_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_public_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_mut_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_public_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer_mut_ref()
            .valid_public_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(&mut self.vertex_vector_for_all_vertex_types_mut_ref()[i])
            })?;
        Ok(())
    }
}

impl MapPrivateVertexVectors for VertexStore {
    // fn map_all_valid_private_vertex_vectors<F>(
    //     &self,
    //     function_to_apply: F,
    // ) -> Result<(), GraphComputingError>
    // where
    //     F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    // {
    //     // TODO: would par_iter() give better performance?
    //     self.vertex_type_indexer_ref()
    //         .valid_private_indices()?
    //         .into_iter()
    //         .try_for_each(|i: usize| {
    //             function_to_apply(&self.vertex_vector_for_all_vertex_types_ref()[i])
    //         })?;
    //     Ok(())
    // }

    fn map_mut_all_valid_private_vertex_vectors<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
    {
        // TODO: would par_iter() give better performance?
        self.vertex_type_indexer_mut_ref()
            .valid_private_indices()?
            .into_iter()
            .try_for_each(|i: usize| {
                function_to_apply(&mut self.vertex_vector_for_all_vertex_types_mut_ref()[i])
            })?;
        Ok(())
    }
}
