use petgraph::graph::IndexType;
use petgraph::{EdgeType, Graph};
use std::default::Default;

/// Generates an empty graph with `n` nodes and given node and edge capacities.
pub(crate) fn empty_graph_with_capacity<N: Default, E: Default, Ty: EdgeType, Ix: IndexType>(
    n: usize,
    node_capacity: usize,
    edge_capacity: usize,
) -> Graph<N, E, Ty, Ix> {
    let mut graph = Graph::<N, E, Ty, Ix>::with_capacity(node_capacity, edge_capacity);
    for _ in 0..n {
        graph.add_node(Default::default());
    }
    graph
}

#[cfg(test)]
pub(crate) fn assert_graph_eq<
    N: Default + PartialEq + std::fmt::Debug,
    E: Default,
    Ty: EdgeType,
    Ix: IndexType,
>(
    graph: &Graph<N, E, Ty, Ix>,
    expected: &Graph<N, E, Ty, Ix>,
) {
    assert_eq!(graph.node_count(), expected.node_count());
    assert_eq!(graph.edge_count(), expected.edge_count());
    for (node_index, expected_index) in graph.node_indices().zip(expected.node_indices()) {
        assert_eq!(graph[node_index], expected[expected_index]);
    }
    for (edge_index, expected_index) in graph.edge_indices().zip(expected.edge_indices()) {
        assert_eq!(
            graph.edge_endpoints(edge_index),
            expected.edge_endpoints(expected_index)
        );
    }
}
