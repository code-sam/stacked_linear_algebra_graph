use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::operations::{
    DropSparseMatrixElement, GetSparseMatrixElementValue, GetSparseMatrixSize,
    IsSparseMatrixElement, SetSparseMatrixElement,
};
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SparseMatrix;
use graphblas_sparse_linear_algebra::operators::apply::{ApplyUnaryOperator, UnaryOperatorApplier};
use graphblas_sparse_linear_algebra::operators::binary_operator::Assignment;
use graphblas_sparse_linear_algebra::operators::options::OptionsForOperatorWithMatrixArgument;
use graphblas_sparse_linear_algebra::operators::unary_operator::Identity;
use rand::distributions::{Distribution, Uniform};

use graphblas_sparse_linear_algebra::context::{Context, GetContext, MatrixStorageFormat, Mode};

fn copy_sparse_matrix_value(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let matrix_size = 100_000;
    let number_of_elements = (0.02 * matrix_size as f32 * matrix_size as f32) as usize;
    let number_of_updates = 1_000_000;

    let context = Context::init(Mode::NonBlocking, MatrixStorageFormat::ByColumn).unwrap();
    let mut matrix = SparseMatrix::<f32>::new(context, (matrix_size, matrix_size).into()).unwrap();

    let row_index_distribution = Uniform::from(0..matrix_size);
    let column_index_distribution = Uniform::from(0..matrix_size);
    let element_value_distribution = Uniform::from(0.0f32..(matrix_size as f32));
    let indices_to_update_distribution = Uniform::from(0..number_of_updates);

    let mut row_indices = Vec::with_capacity(number_of_elements);
    let mut column_indices = Vec::with_capacity(number_of_elements);

    let mut indices_to_update = Vec::<usize>::with_capacity(number_of_updates);
    let mut updated_values = Vec::<f32>::with_capacity(number_of_updates);

    for i in 0..number_of_elements {
        row_indices.push(row_index_distribution.sample(&mut rng));
        column_indices.push(column_index_distribution.sample(&mut rng));

        matrix
            .set_value(
                row_indices[i],
                column_indices[i],
                element_value_distribution.sample(&mut rng),
            )
            .unwrap()
    }

    for i in 0..number_of_updates {
        indices_to_update.push(indices_to_update_distribution.sample(&mut rng));
        updated_values.push(element_value_distribution.sample(&mut rng));
    }

    c.bench_with_input(
        BenchmarkId::new("update_matrix_value", ""),
        &0.0,
        |b, data| {
            b.iter(|| {
                bench_update_matrix_value(
                    matrix.clone(),
                    row_indices.clone(),
                    column_indices.clone(),
                    indices_to_update.clone(),
                    updated_values.clone(),
                )
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("copy_and_update_matrix_value", ""),
        &0.0,
        |b, data| {
            b.iter(|| {
                bench_copy_and_update_matrix_value(
                    matrix.clone(),
                    row_indices.clone(),
                    column_indices.clone(),
                    indices_to_update.clone(),
                    updated_values.clone(),
                )
            })
        },
    );

    c.bench_with_input(
        BenchmarkId::new("copy_to_matrix_and_update_matrix_value", ""),
        &0.0,
        |b, data| {
            b.iter(|| {
                bench_copy_to_matrix_and_update_matrix_value(
                    matrix.clone(),
                    row_indices.clone(),
                    column_indices.clone(),
                    indices_to_update.clone(),
                    updated_values.clone(),
                )
            })
        },
    );
}

criterion_group!(benches, copy_sparse_matrix_value);
criterion_main!(benches);

fn bench_update_matrix_value(
    mut matrix: SparseMatrix<f32>,
    row_indices: Vec<usize>,
    column_indices: Vec<usize>,
    indices_to_update: Vec<usize>,
    updated_values: Vec<f32>,
) {
    for i in indices_to_update {
        matrix
            .set_value(row_indices[i], column_indices[i], updated_values[i])
            .unwrap();
    }
}

fn bench_copy_and_update_matrix_value(
    mut matrix: SparseMatrix<f32>,
    row_indices: Vec<usize>,
    column_indices: Vec<usize>,
    indices_to_update: Vec<usize>,
    updated_values: Vec<f32>,
) {
    let mut row_indices_copy = Vec::new();
    let mut column_indices_copy = Vec::new();
    let mut values_copy = Vec::new();

    for i in indices_to_update {
        row_indices_copy.push(row_indices[i]);
        column_indices_copy.push(column_indices[i]);
        values_copy.push(
            matrix
                .element_value_or_default(row_indices[i], column_indices[i])
                .unwrap(),
        );

        matrix
            .set_value(row_indices[i], column_indices[i], updated_values[i])
            .unwrap();
    }
}

fn bench_copy_to_matrix_and_update_matrix_value(
    mut matrix: SparseMatrix<f32>,
    row_indices: Vec<usize>,
    column_indices: Vec<usize>,
    indices_to_update: Vec<usize>,
    updated_values: Vec<f32>,
) {
    let mut matrix_values_to_restore =
        SparseMatrix::<f32>::new(matrix.context(), matrix.size().unwrap()).unwrap();
    let mut mask = SparseMatrix::<bool>::new(matrix.context(), matrix.size().unwrap()).unwrap();

    for i in indices_to_update {
        if matrix_values_to_restore
            .is_element(row_indices[i], column_indices[i])
            .unwrap()
        {
            mask.set_value(row_indices[i], column_indices[i], true)
                .unwrap();

            UnaryOperatorApplier::new()
                .apply_to_matrix(
                    &Identity::<f32>::new(),
                    &matrix,
                    &Assignment::new(),
                    &mut matrix_values_to_restore,
                    &mask,
                    &OptionsForOperatorWithMatrixArgument::new_default(),
                )
                .unwrap();

            mask.drop_element(row_indices[i], column_indices[i])
                .unwrap();
        }

        matrix
            .set_value(row_indices[i], column_indices[i], updated_values[i])
            .unwrap();
    }
}
