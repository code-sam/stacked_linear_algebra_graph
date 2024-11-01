use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    error::GraphComputingError,
    graph::{
        indexing::{
            operations::{GetValidIndices, GetValidPrivateIndices, GetValidPublicIndices},
            Index, VertexTypeIndex,
        },
        vertex_store::{GetVertexTypeIndexer, GetVertexVectors, VertexStore, VertexVector},
    },
};

// pub(crate) trait MapAllVertexVectors {
//     fn map_all_vertex_vectors<F>(&self, function_to_apply: F) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync;

//     fn map_mut_all_vertex_vectors<F>(
//         &mut self,
//         function_to_apply: F,
//     ) -> Result<(), GraphComputingError>
//     where
//         F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync;
// }

pub(crate) fn map_all_vertex_vectors<F>(
    vertex_store: &VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store
        .vertex_vector_for_all_vertex_types_ref()
        .into_par_iter()
        .try_for_each(function_to_apply)?;
    Ok(())
}

pub(crate) fn map_mut_all_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store
        .vertex_vector_for_all_vertex_types_mut_ref()
        .into_par_iter()
        .try_for_each(function_to_apply)?;
    Ok(())
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

pub(crate) fn map_mut_all_valid_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.map_mut_all_valid_vertex_vectors(function_to_apply)
}

pub(crate) fn indexed_map_mut_all_valid_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: FnMut(&VertexTypeIndex, &mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.indexed_map_mut_all_valid_vertex_vectors(function_to_apply)
}

pub(crate) fn map_mut_all_valid_public_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.map_mut_all_valid_public_vertex_vectors(function_to_apply)
}

pub(crate) fn indexed_map_mut_all_valid_public_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: FnMut(&VertexTypeIndex, &mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.indexed_map_mut_all_valid_public_vertex_vectors(function_to_apply)
}

pub(crate) fn map_mut_all_valid_private_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: Fn(&mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.map_mut_all_valid_private_vertex_vectors(function_to_apply)
}

pub(crate) fn indexed_map_mut_all_valid_private_vertex_vectors<F>(
    vertex_store: &mut VertexStore,
    function_to_apply: F,
) -> Result<(), GraphComputingError>
where
    F: FnMut(&VertexTypeIndex, &mut VertexVector) -> Result<(), GraphComputingError> + Send + Sync,
{
    vertex_store.indexed_map_mut_all_valid_private_vertex_vectors(function_to_apply)
}
