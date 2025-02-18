use graphblas_sparse_linear_algebra::collections::sparse_vector::operations::resize_sparse_vector;
use graphblas_sparse_linear_algebra::index::ElementCount;

use crate::error::GraphComputingError;
use crate::graph::indexing::ElementIndexMap;
use crate::graph::value_type::ValueType;
use crate::transaction::in_memory::SparseVectorStateReverter;
use crate::transaction::RestoreState;
use crate::graph::vertex_store::VertexVector;
use crate::graph::vertex_store::operations::in_memory_transaction::transaction::vertex_store_state_restorer::vertex_vectors_state_restorer::vertex_vectors_state_restorer::{GetVertexTypeVectorLengthToRestore, VertexVectorsStateRestorer};

impl RestoreState<Vec<VertexVector>> for VertexVectorsStateRestorer {
    fn restore(
        self,
        vectors_to_restore: &mut Vec<VertexVector>,
    ) -> Result<(), crate::error::GraphComputingError> {
        restore_vertex_vectors_state(self, vectors_to_restore)
    }

    fn with_reset_state_to_restore(&self) -> Self {
        Self::with_vertex_type_vector_length_to_restore(self.vertex_type_vector_length_to_restore())
    }
}

pub(crate) fn restore_vertex_vectors_state(
    vertex_vectors_state_restorer: VertexVectorsStateRestorer,
    vectors_to_restore: &mut Vec<VertexVector>,
) -> Result<(), crate::error::GraphComputingError> {
    let vertex_vector_state_reverters = vertex_vectors_state_restorer.vertex_vector_state_reverters;

    let vertex_vector_length_to_restore =
        vertex_vectors_state_restorer.vertex_vector_length_to_restore;

    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_bool,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i8,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i16,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_i64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u8,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u16,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_u64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_f32,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_f64,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_isize,
        vectors_to_restore,
    )?;
    restore_vertex_vectors(
        vertex_vector_state_reverters.sparse_vector_state_reverters_usize,
        vectors_to_restore,
    )?;

    vectors_to_restore.truncate(vertex_vectors_state_restorer.vertex_type_vector_length_to_restore);

    restore_vertex_vector_length(vertex_vector_length_to_restore, vectors_to_restore)?;

    Ok(())
}

fn restore_vertex_vectors<T>(
    typed_sparse_vector_state_reverters_for_vertex_type: ElementIndexMap<
        SparseVectorStateReverter<T>,
    >,
    vectors_to_restore: &mut Vec<VertexVector>,
) -> Result<(), GraphComputingError>
where
    T: ValueType,
    SparseVectorStateReverter<T>: RestoreState<VertexVector>,
{
    for (vertex_type_index, sparse_vector_state_reverter) in
        typed_sparse_vector_state_reverters_for_vertex_type.into_iter()
    {
        sparse_vector_state_reverter.restore(&mut vectors_to_restore[vertex_type_index])?;
    }
    Ok(())
}

fn restore_vertex_vector_length(
    vertex_vector_length_to_restore: Option<ElementCount>,
    vectors_to_restore: &mut Vec<VertexVector>,
) -> Result<(), GraphComputingError> {
    Ok(match vertex_vector_length_to_restore {
        Some(element_count) => {
            // TODO: consider resizing in parallel
            for vertex_vector in vectors_to_restore.iter_mut() {
                resize_sparse_vector(vertex_vector, element_count)?;
            }
        }
        None => (),
    })
}
