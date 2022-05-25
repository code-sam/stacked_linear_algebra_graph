use crate::graph::edge::DirectedEdgeDefinedByKeys;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::operations::add_edge::AddEdge;
use crate::operations::add_vertex::AddVertex;

macro_rules! add_new_edge {
    ($from_vertex:ident, $edge_type:ident, $to_vertex:ident, $graph:ident) => {
        let edge = DirectedEdgeDefinedByKeys::new(
            $from_vertex.clone().into(),
            $edge_type.clone(),
            $to_vertex.clone().into(),
        );
        $graph
            .add_edge_and_edge_type_using_keys(edge.clone())
            .unwrap();
    };
}

pub fn standard_graph_for_testing() -> Graph {
    create_test_graph()
}

fn create_test_graph() -> Graph {
    let initial_vertex_capacity = 10;
    let initial_edge_type_capacity = 10;
    let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

    let zero = Vertex::new(String::from("0"), 0u8.into());
    graph.add_or_replace_vertex(zero.clone()).unwrap();

    let one = Vertex::new(String::from("1"), 1u8.into());
    graph.add_or_replace_vertex(one.clone()).unwrap();

    let one_duplicate = Vertex::new(String::from("1_duplicate"), 1u8.into());
    graph.add_or_replace_vertex(one_duplicate.clone()).unwrap();

    let one_dot_one = Vertex::new(String::from("1.1"), 1.1f32.into());
    graph.add_or_replace_vertex(one_dot_one.clone()).unwrap();

    let one_dot_two = Vertex::new(String::from("1.2"), 1.2f32.into());
    graph.add_or_replace_vertex(one_dot_two.clone()).unwrap();

    let two = Vertex::new(String::from("2"), 2u8.into());
    graph.add_or_replace_vertex(two.clone()).unwrap();

    let negative_one = Vertex::new(String::from("-1"), (-1i8).into());
    graph.add_or_replace_vertex(negative_one.clone()).unwrap();

    let negative_one_dot_one = Vertex::new(String::from("-1.1"), (-1.1f32).into());
    graph
        .add_or_replace_vertex(negative_one_dot_one.clone())
        .unwrap();

    let not_a_number = Vertex::new(String::from("NaN"), String::from("not_a_number").into());
    graph.add_or_replace_vertex(not_a_number.clone()).unwrap();

    let integer = Vertex::new(String::from("integer"), String::from("integer").into());
    graph.add_or_replace_vertex(integer.clone()).unwrap();

    let natural_number = Vertex::new(
        String::from("natural_number"),
        String::from("natural_number").into(),
    );
    graph.add_or_replace_vertex(natural_number.clone()).unwrap();

    let real_number = Vertex::new(
        String::from("real_number"),
        String::from("real_number").into(),
    );
    graph.add_or_replace_vertex(real_number.clone()).unwrap();

    let positive = Vertex::new(String::from("positive"), String::from("positive").into());
    graph.add_or_replace_vertex(positive.clone()).unwrap();

    let negative = Vertex::new(String::from("negative"), String::from("negative").into());
    graph.add_or_replace_vertex(negative.clone()).unwrap();

    let string = Vertex::new(String::from("string"), String::from("string").into());
    graph.add_or_replace_vertex(string.clone()).unwrap();

    let sign = String::from("sign");
    let smaller_than = String::from("smaller_than");
    let larger_than = String::from("larger_than");
    let equal_to = String::from("equal_to");
    let is_a = String::from("is_a");

    add_new_edge!(not_a_number, is_a, string, graph);

    add_new_edge!(negative_one_dot_one, is_a, real_number, graph);
    add_new_edge!(negative_one_dot_one, sign, negative, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, negative_one, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, zero, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, one, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, one_dot_one, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, one_dot_two, graph);
    add_new_edge!(negative_one_dot_one, smaller_than, two, graph);

    add_new_edge!(negative_one, is_a, real_number, graph);
    add_new_edge!(negative_one, is_a, integer, graph);
    add_new_edge!(negative_one, sign, negative, graph);
    add_new_edge!(negative_one, larger_than, negative_one_dot_one, graph);
    add_new_edge!(negative_one, smaller_than, zero, graph);
    add_new_edge!(negative_one, smaller_than, one, graph);
    add_new_edge!(negative_one, smaller_than, one_duplicate, graph);
    add_new_edge!(negative_one, smaller_than, one_dot_one, graph);
    add_new_edge!(negative_one, smaller_than, one_dot_two, graph);
    add_new_edge!(negative_one, smaller_than, two, graph);

    add_new_edge!(zero, is_a, real_number, graph);
    add_new_edge!(zero, is_a, integer, graph);
    add_new_edge!(zero, is_a, natural_number, graph);
    // add_new_edge!(zero, sign, positive, graph);
    // add_new_edge!(zero, sign, negative, graph);
    add_new_edge!(zero, larger_than, negative_one_dot_one, graph);
    add_new_edge!(zero, larger_than, negative_one, graph);
    add_new_edge!(zero, smaller_than, one, graph);
    add_new_edge!(zero, smaller_than, one_duplicate, graph);
    add_new_edge!(zero, smaller_than, one_dot_one, graph);
    add_new_edge!(zero, smaller_than, one_dot_two, graph);
    add_new_edge!(zero, smaller_than, two, graph);

    add_new_edge!(one, is_a, real_number, graph);
    add_new_edge!(one, is_a, integer, graph);
    add_new_edge!(one, is_a, natural_number, graph);
    add_new_edge!(one, sign, positive, graph);
    add_new_edge!(one, larger_than, negative_one_dot_one, graph);
    add_new_edge!(one, larger_than, negative_one, graph);
    add_new_edge!(one, larger_than, zero, graph);
    add_new_edge!(one, equal_to, one_duplicate, graph);
    add_new_edge!(one, smaller_than, one_dot_one, graph);
    add_new_edge!(one, smaller_than, one_dot_two, graph);
    add_new_edge!(one, smaller_than, two, graph);

    add_new_edge!(one_duplicate, is_a, real_number, graph);
    add_new_edge!(one_duplicate, is_a, integer, graph);
    add_new_edge!(one_duplicate, is_a, natural_number, graph);
    add_new_edge!(one_duplicate, sign, positive, graph);
    add_new_edge!(one_duplicate, larger_than, negative_one_dot_one, graph);
    add_new_edge!(one_duplicate, larger_than, negative_one, graph);
    add_new_edge!(one_duplicate, larger_than, zero, graph);
    add_new_edge!(one_duplicate, equal_to, one_duplicate, graph);
    add_new_edge!(one_duplicate, smaller_than, one_dot_one, graph);
    add_new_edge!(one_duplicate, smaller_than, one_dot_two, graph);
    add_new_edge!(one_duplicate, smaller_than, two, graph);

    add_new_edge!(one_dot_one, is_a, real_number, graph);
    // add_new_edge!(one_dot_one, is_a, integer, graph);
    // add_new_edge!(one_dot_one, is_a, natural_number, graph);
    add_new_edge!(one_dot_one, sign, positive, graph);
    add_new_edge!(one_dot_one, larger_than, negative_one_dot_one, graph);
    add_new_edge!(one_dot_one, larger_than, negative_one, graph);
    add_new_edge!(one_dot_one, larger_than, zero, graph);
    add_new_edge!(one_dot_one, larger_than, one, graph);
    add_new_edge!(one_dot_one, larger_than, one_duplicate, graph);
    add_new_edge!(one_dot_one, smaller_than, one_dot_two, graph);
    add_new_edge!(one_dot_one, smaller_than, two, graph);

    add_new_edge!(one_dot_two, is_a, real_number, graph);
    // add_new_edge!(one_dot_one, is_a, integer, graph);
    // add_new_edge!(one_dot_one, is_a, natural_number, graph);
    add_new_edge!(one_dot_two, sign, positive, graph);
    add_new_edge!(one_dot_two, larger_than, negative_one_dot_one, graph);
    add_new_edge!(one_dot_two, larger_than, negative_one, graph);
    add_new_edge!(one_dot_two, larger_than, zero, graph);
    add_new_edge!(one_dot_two, larger_than, one, graph);
    add_new_edge!(one_dot_two, larger_than, one_duplicate, graph);
    add_new_edge!(one_dot_two, larger_than, one_dot_one, graph);
    add_new_edge!(one_dot_two, smaller_than, two, graph);

    add_new_edge!(two, is_a, real_number, graph);
    add_new_edge!(two, is_a, integer, graph);
    add_new_edge!(two, is_a, natural_number, graph);
    add_new_edge!(two, sign, positive, graph);
    add_new_edge!(two, larger_than, negative_one_dot_one, graph);
    add_new_edge!(two, larger_than, negative_one, graph);
    add_new_edge!(two, larger_than, zero, graph);
    add_new_edge!(two, larger_than, one, graph);
    add_new_edge!(two, larger_than, one_duplicate, graph);
    add_new_edge!(two, larger_than, one_dot_one, graph);
    add_new_edge!(two, larger_than, one_dot_two, graph);

    return graph;
}
