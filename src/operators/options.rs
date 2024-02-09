use graphblas_sparse_linear_algebra::{
    graphblas_bindings::GrB_Descriptor,
    operators::options::{
        GetGraphblasDescriptor, GetOperatorOptions as GetGraphblasOperatorOptions,
        OperatorOptions as GraphblasOperatorOptions,
    },
};

// Implemented methods do not provide mutable access to GraphBLAS operators or options.
// Code review must consider that no mtable access is provided.
// https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for OperatorOptions {}
unsafe impl Sync for OperatorOptions {}

#[derive(Clone, Debug)]
pub struct OperatorOptions {
    graphblas_operator_options: GrB_Descriptor,
    clear_output_before_use: bool,
    use_mask_structure_of_stored_values_as_mask: bool,
    use_mask_complement: bool,
    transpose_input0: bool,
    transpose_input1: bool,
    use_cached_adjacency_matrix_transpose: bool,
}

impl OperatorOptions {
    pub fn new(
        clear_output_before_use: bool,
        use_mask_structure_of_stored_values_as_mask: bool,
        use_mask_complement: bool,
        transpose_input0: bool,
        transpose_input1: bool,
        use_cached_adjacency_matrix_transpose: bool,
    ) -> Self {
        let graphblas_operator_options = GraphblasOperatorOptions::new(
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_input0 && !use_cached_adjacency_matrix_transpose,
            transpose_input1 && !use_cached_adjacency_matrix_transpose,
        );
        Self {
            graphblas_operator_options: graphblas_operator_options.graphblas_descriptor(),
            clear_output_before_use,
            use_mask_structure_of_stored_values_as_mask,
            use_mask_complement,
            transpose_input0: use_cached_adjacency_matrix_transpose && transpose_input0,
            transpose_input1: use_cached_adjacency_matrix_transpose && transpose_input1,
            use_cached_adjacency_matrix_transpose,
        }
    }

    pub fn new_default() -> Self {
        let default_graphblas_operator_options = GraphblasOperatorOptions::new_default();
        Self {
            graphblas_operator_options: default_graphblas_operator_options.graphblas_descriptor(),
            clear_output_before_use: default_graphblas_operator_options.clear_output_before_use(),
            use_mask_structure_of_stored_values_as_mask: default_graphblas_operator_options
                .use_mask_structure_of_stored_values_as_mask(),
            use_mask_complement: default_graphblas_operator_options.use_mask_complement(),
            transpose_input0: false,
            transpose_input1: false,
            use_cached_adjacency_matrix_transpose: true,
        }
    }
}

pub trait GetOperatorOptions: GetGraphblasOperatorOptions {
    fn use_cached_adjacency_matrix_transpose(&self) -> bool;
}

impl GetGraphblasOperatorOptions for OperatorOptions {
    fn clear_output_before_use(&self) -> bool {
        self.clear_output_before_use
    }

    fn use_mask_structure_of_stored_values_as_mask(&self) -> bool {
        self.use_mask_structure_of_stored_values_as_mask
    }

    fn use_mask_complement(&self) -> bool {
        self.use_mask_complement
    }

    fn transpose_input0(&self) -> bool {
        self.transpose_input0
    }

    fn transpose_input1(&self) -> bool {
        self.transpose_input1
    }
}

impl GetOperatorOptions for OperatorOptions {
    fn use_cached_adjacency_matrix_transpose(&self) -> bool {
        self.use_cached_adjacency_matrix_transpose
    }
}

impl GetGraphblasDescriptor for OperatorOptions {
    fn graphblas_descriptor(&self) -> GrB_Descriptor {
        self.graphblas_operator_options
    }
}
