use graphblas_sparse_linear_algebra::{
    graphblas_bindings::GrB_Descriptor,
    operators::options::{
        GetClearOutputBeforeUse, GetGraphblasDescriptor, GetOperatorMaskOptions,
        GetOperatorOptions, GetOptionsForOperatorWithMatrixArgument, GetTransposeMatrixArgument,
        OptionsForOperatorWithMatrixArgument, WithTransposeMatrixArgument,
    },
};

use super::GetUseCachedAdjacencyMatrixTranspose;

// Implemented methods do not provide mutable access to GraphBLAS operators or options.
// Code review must consider that no mtable access is provided.
// https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for OptionsForOperatorWithAdjacencyMatrixArgument {}
unsafe impl Sync for OptionsForOperatorWithAdjacencyMatrixArgument {}

#[derive(Debug, Clone)]
pub struct OptionsForOperatorWithAdjacencyMatrixArgument {
    use_cached_adjacency_matrix_transpose: bool,
    clear_output_before_use: bool,
    use_mask_structure_of_stored_values_as_mask: bool,
    use_mask_complement: bool,
    transpose_matrix_argument: bool,

    graphblas_descriptor: GrB_Descriptor,
}

pub trait GetOptionsForMaskedOperatorWithAdjacencyMatrixArgument:
    GetOptionsForOperatorWithMatrixArgument + GetUseCachedAdjacencyMatrixTranspose
{
}

impl GetOptionsForOperatorWithMatrixArgument for OptionsForOperatorWithAdjacencyMatrixArgument {}

impl GetOperatorOptions for OptionsForOperatorWithAdjacencyMatrixArgument {}

impl GetUseCachedAdjacencyMatrixTranspose for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn use_cached_adjacency_matrix_transpose(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetClearOutputBeforeUse for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn clear_output_before_use(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetOperatorMaskOptions for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn use_mask_structure_of_stored_values_as_mask(&self) -> bool {
        self.use_mask_structure_of_stored_values_as_mask
    }

    fn use_mask_complement(&self) -> bool {
        self.use_mask_complement
    }
}

impl GetTransposeMatrixArgument for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn transpose_matrix_argument(&self) -> bool {
        self.transpose_matrix_argument
    }
}

impl GetGraphblasDescriptor for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn graphblas_descriptor(&self) -> GrB_Descriptor {
        self.graphblas_descriptor
    }
}

impl WithTransposeMatrixArgument for OptionsForOperatorWithAdjacencyMatrixArgument {
    fn with_negated_transpose_matrix_argument(&self) -> Self {
        OptionsForOperatorWithAdjacencyMatrixArgument::new(
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
            OptionsForOperatorWithAdjacencyMatrixArgument::new(
                self.use_cached_adjacency_matrix_transpose,
                self.clear_output_before_use,
                self.use_mask_structure_of_stored_values_as_mask,
                self.use_mask_complement,
                transpose_matrix,
            )
        }
    }
}

impl OptionsForOperatorWithAdjacencyMatrixArgument {
    pub fn new(
        use_cached_adjacency_matrix_transpose: bool,
        clear_output_before_use: bool,
        use_mask_structure_of_stored_values_as_mask: bool,
        use_mask_complement: bool,
        transpose_matrix_argument: bool,
    ) -> Self {
        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixArgument::new(
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

        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixArgument::new(
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
