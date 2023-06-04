use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use graphblas_sparse_linear_algebra::{
    context::Context,
    operators::{
        binary_operator::{Assignment, Plus},
        options::OperatorOptions,
    },
};
use stacked_linear_algebra_graph::{
    graph::{
        edge::{DirectedEdgeCoordinateDefinedByKeys, WeightedDirectedEdgeDefinedByKeys},
        graph::Graph,
        vertex::VertexDefinedByKey,
    },
    operators::{
        AddEdge, AddEdgeType, AddVertex, AddVertexType, ApplyScalarBinaryOperatorToAdjacencyMatrix,
    },
};

fn graph_binary_operator_benchmark(c: &mut Criterion) {
    let mut graph = Graph::with_initial_capacity(&256, &4096, &256).unwrap();

    for i in 0..1000 {
        graph
            .add_new_vertex_type(format!("vertex_type_{}", i).as_str())
            .unwrap();
    }

    for i in 0..1000 {
        graph
            .add_new_edge_type(format!("edge_type_{}", i).as_str())
            .unwrap();
    }

    for i in 0..10_000 {
        graph
            .add_new_vertex(VertexDefinedByKey::new(
                "vertex_type_250",
                format!("vertex_{}", i).as_str(),
                &i,
            ))
            .unwrap();
    }

    for i in 0..10_000 / 10 {
        for j in 3..10_000 / 6 {
            graph
                .add_new_edge_using_keys(WeightedDirectedEdgeDefinedByKeys::new(
                    DirectedEdgeCoordinateDefinedByKeys::new(
                        "edge_type_250",
                        format!("vertex_{}", i * 4).as_str(),
                        format!("vertex_{}", j * 3).as_str(),
                    ),
                    i + j,
                ))
                .unwrap();
        }
    }

    c.bench_with_input(
        BenchmarkId::new("with_key", "parameter"),
        &graph,
        |b, data| b.iter(|| add_scalar_to_adjacency_matrix_with_key(&mut graph.clone())),
    );
}

criterion_group!(benches, graph_binary_operator_benchmark);
criterion_main!(benches);

fn add_scalar_to_adjacency_matrix_with_key(graph: &mut Graph) {
    ApplyScalarBinaryOperatorToAdjacencyMatrix::<i32, i32, i32>::with_key_defined_adjacency_matrix_as_left_argument(
        graph,
        "edge_type_250",
        &Plus::<i32>::new(),
        &1,
        &Assignment::new(),
        "edge_type_251",
        &OperatorOptions::new_default(),
    ).unwrap();

    // assert_eq!(
    //     ReadEdgeWeight::<u16>::key_defined_edge_weight(
    //         &graph,
    //         &DirectedEdgeCoordinateDefinedByKeys::new(
    //             result_type_key,
    //             vertex_1.key_ref(),
    //             vertex_2.key_ref(),
    //         ),
    //     )
    //     .unwrap(),
    //     Some(2)
    // );
}
