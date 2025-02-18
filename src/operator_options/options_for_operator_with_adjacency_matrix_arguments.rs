use graphblas_sparse_linear_algebra::{
    graphblas_bindings::GrB_Descriptor,
    operators::options::{
        GetClearOutputBeforeUse, GetGraphblasDescriptor, GetOperatorMaskOptions,
        GetOperatorOptions, GetOptionsForOperatorWithMatrixArguments, GetTransposeArguments,
        OptionsForOperatorWithMatrixArguments, WithTransposeArguments,
    },
};

use super::GetUseCachedAdjacencyMatrixTranspose;

// Implemented methods do not provide mutable access to GraphBLAS operators or options.
// Code review must consider that no mtable access is provided.
// https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for OptionsForOperatorWithAdjacencyMatrixArguments {}
unsafe impl Sync for OptionsForOperatorWithAdjacencyMatrixArguments {}

#[derive(Debug, Clone)]
pub struct OptionsForOperatorWithAdjacencyMatrixArguments {
    use_cached_adjacency_matrix_transpose: bool,

    clear_output_before_use: bool,
    use_mask_structure_of_stored_values_as_mask: bool,
    use_mask_complement: bool,
    transpose_first_argument: bool,
    transpose_second_argument: bool,

    graphblas_descriptor: GrB_Descriptor,
}

pub trait GetOptionsForOperatorWithAdjacencyMatrixArguments:
    GetOptionsForOperatorWithMatrixArguments + GetTransposeArguments
{
}

impl GetOperatorOptions for OptionsForOperatorWithAdjacencyMatrixArguments {}

impl GetOptionsForOperatorWithMatrixArguments for OptionsForOperatorWithAdjacencyMatrixArguments {}

impl GetUseCachedAdjacencyMatrixTranspose for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn use_cached_adjacency_matrix_transpose(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetClearOutputBeforeUse for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn clear_output_before_use(&self) -> bool {
        self.clear_output_before_use
    }
}

impl GetOperatorMaskOptions for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn use_mask_structure_of_stored_values_as_mask(&self) -> bool {
        self.use_mask_structure_of_stored_values_as_mask
    }

    fn use_mask_complement(&self) -> bool {
        self.use_mask_complement
    }
}

impl GetTransposeArguments for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn transpose_first_argument(&self) -> bool {
        self.transpose_first_argument
    }

    fn transpose_second_argument(&self) -> bool {
        self.transpose_second_argument
    }
}

impl GetGraphblasDescriptor for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn graphblas_descriptor(&self) -> GrB_Descriptor {
        self.graphblas_descriptor
    }
}

impl WithTransposeArguments for OptionsForOperatorWithAdjacencyMatrixArguments {
    fn with_negated_transpose_first_argument(&self) -> Self {
        OptionsForOperatorWithAdjacencyMatrixArguments::new(
            self.use_cached_adjacency_matrix_transpose,
            self.clear_output_before_use,
            self.use_mask_structure_of_stored_values_as_mask,
            self.use_mask_complement,
            !self.transpose_first_argument,
            self.transpose_second_argument,
        )
    }

    fn with_negated_transpose_second_argument(&self) -> Self {
        OptionsForOperatorWithAdjacencyMatrixArguments::new(
            self.use_cached_adjacency_matrix_transpose,
            self.clear_output_before_use,
            self.use_mask_structure_of_stored_values_as_mask,
            self.use_mask_complement,
            self.transpose_first_argument,
            !self.transpose_second_argument,
        )
    }

    fn with_transpose_first_argument(&self, transpose_first_argument: bool) -> Self {
        if transpose_first_argument == self.transpose_first_argument {
            self.to_owned()
        } else {
            OptionsForOperatorWithAdjacencyMatrixArguments::new(
                self.use_cached_adjacency_matrix_transpose,
                self.clear_output_before_use,
                self.use_mask_structure_of_stored_values_as_mask,
                self.use_mask_complement,
                transpose_first_argument,
                self.transpose_second_argument,
            )
        }
    }

    fn with_transpose_second_argument(&self, transpose_second_argument: bool) -> Self {
        if transpose_second_argument == self.transpose_second_argument {
            self.to_owned()
        } else {
            OptionsForOperatorWithAdjacencyMatrixArguments::new(
                self.use_cached_adjacency_matrix_transpose,
                self.clear_output_before_use,
                self.use_mask_structure_of_stored_values_as_mask,
                self.use_mask_complement,
                self.transpose_first_argument,
                transpose_second_argument,
            )
        }
    }

    fn with_transpose_matrix_arguments(
        &self,
        transpose_first_argument: bool,
        transpose_second_argument: bool,
    ) -> Self {
        if transpose_first_argument == self.transpose_first_argument
            && transpose_second_argument == self.transpose_second_argument
        {
            self.to_owned()
        } else {
            OptionsForOperatorWithAdjacencyMatrixArguments::new(
                self.use_cached_adjacency_matrix_transpose,
                self.clear_output_before_use,
                self.use_mask_structure_of_stored_values_as_mask,
                self.use_mask_complement,
                transpose_first_argument,
                transpose_second_argument,
            )
        }
    }
}

impl OptionsForOperatorWithAdjacencyMatrixArguments {
    pub fn new(
        use_cached_adjacency_matrix_transpose: bool,
        clear_output_before_use: bool,
        use_mask_structure_of_stored_values_as_mask: bool,
        use_mask_complement: bool,
        transpose_first_argument: bool,
        transpose_second_argument: bool,
    ) -> Self {
        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixArguments::new(
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_first_argument,
            transpose_second_argument,
        );

        Self {
            use_cached_adjacency_matrix_transpose,
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_first_argument,
            transpose_second_argument,

            graphblas_descriptor: sparse_linear_algebra_options.graphblas_descriptor(),
        }
    }

    pub fn new_default() -> Self {
        let use_cached_adjacency_matrix_transpose = true;
        let use_mask_structure_of_stored_values_as_mask = false;
        let use_mask_complement = false;
        let transpose_first_argument = false;
        let transpose_second_argument = false;
        let clear_output_before_use = false;

        let sparse_linear_algebra_options = OptionsForOperatorWithMatrixArguments::new(
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_first_argument,
            transpose_second_argument,
        );

        Self {
            use_cached_adjacency_matrix_transpose,

            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_first_argument,
            transpose_second_argument,

            graphblas_descriptor: sparse_linear_algebra_options.graphblas_descriptor(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_options() {
        let default_options = OptionsForOperatorWithAdjacencyMatrixArguments::new_default();
        let expected_value: GrB_Descriptor = ptr::null_mut();
        assert_eq!(default_options.graphblas_descriptor(), expected_value)
    }
}
