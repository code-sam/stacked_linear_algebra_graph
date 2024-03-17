use graphblas_sparse_linear_algebra::{
    graphblas_bindings::GrB_Descriptor,
    operators::options::{
        GetClearOutputBeforeUse, GetGraphblasDescriptor, GetOperatorMaskOptions,
        GetOperatorOptions, GetOptionsForOperatorWithMatrixAsFirstArgument,
        GetTransposeFirstMatrixArgument, OptionsForOperatorWithMatrixAsFirstArgument,
        WithTransposeMatrixArgument,
    },
};

use super::GetUseCachedAdjacencyMatrixTranspose;

// Implemented methods do not provide mutable access to GraphBLAS operators or options.
// Code review must consider that no mtable access is provided.
// https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {}
unsafe impl Sync for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {}

#[derive(Debug, Clone)]
pub struct OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    use_cached_adjacency_matrix_transpose: bool,

    clear_output_before_use: bool,
    use_mask_structure_of_stored_values_as_mask: bool,
    use_mask_complement: bool,
    transpose_matrix_argument: bool,

    graphblas_descriptor: GrB_Descriptor,
}

pub trait GetOptionsForOperatorWithAdjacencyMatrixAsFirstArgument:
    GetOptionsForOperatorWithMatrixAsFirstArgument + GetUseCachedAdjacencyMatrixTranspose
{
}

impl GetOptionsForOperatorWithMatrixAsFirstArgument
    for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument
{
}

impl GetOperatorOptions for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {}

impl GetUseCachedAdjacencyMatrixTranspose for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn use_cached_adjacency_matrix_transpose(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetClearOutputBeforeUse for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn clear_output_before_use(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetOperatorMaskOptions for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn use_mask_structure_of_stored_values_as_mask(&self) -> bool {
        self.use_mask_structure_of_stored_values_as_mask
    }

    fn use_mask_complement(&self) -> bool {
        self.use_mask_complement
    }
}

impl GetTransposeFirstMatrixArgument for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn transpose_first_matrix_argument(&self) -> bool {
        self.transpose_matrix_argument
    }
}

impl GetGraphblasDescriptor for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn graphblas_descriptor(&self) -> GrB_Descriptor {
        self.graphblas_descriptor
    }
}

impl WithTransposeMatrixArgument for OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    fn with_negated_transpose_matrix_argument(&self) -> Self {
        OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
            self.use_cached_adjacency_matrix_transpose,
            self.clear_output_before_use,
            self.use_mask_structure_of_stored_values_as_mask,
            self.use_mask_complement,
            !self.transpose_matrix_argument,
        )
    }

    fn with_transpose_matrix_argument(&self, transpose_matrix: bool) -> Self {
        if transpose_matrix == self.transpose_matrix_argument {
            self.to_owned()
        } else {
            OptionsForOperatorWithAdjacencyMatrixAsLeftArgument::new(
                self.use_cached_adjacency_matrix_transpose,
                self.clear_output_before_use,
                self.use_mask_structure_of_stored_values_as_mask,
                self.use_mask_complement,
                transpose_matrix,
            )
        }
    }
}

impl OptionsForOperatorWithAdjacencyMatrixAsLeftArgument {
    pub fn new(
        use_cached_adjacency_matrix_transpose: bool,
        clear_output_before_use: bool,
        use_mask_structure_of_stored_values_as_mask: bool,
        use_mask_complement: bool,
        transpose_matrix_argument: bool,
    ) -> Self {
        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixAsFirstArgument::new(
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_matrix_argument,
        );

        Self {
            use_cached_adjacency_matrix_transpose,
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_matrix_argument,

            graphblas_descriptor: sparse_linear_algebra_options.graphblas_descriptor(),
        }
    }

    pub fn new_default() -> Self {
        let use_cached_adjacency_matrix_transpose = true;
        let clear_output_before_use = false;
        let use_mask_structure_of_stored_values_as_mask = false;
        let use_mask_complement = false;
        let transpose_matrix_argument = false;

        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixAsFirstArgument::new(
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_matrix_argument,
        );

        Self {
            use_cached_adjacency_matrix_transpose,
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_matrix_argument,

            graphblas_descriptor: sparse_linear_algebra_options.graphblas_descriptor(),
        }
    }
}

#[cfg(test)]
mod tests {}
