use crate::common::empty_graph_with_capacity;
use petgraph::graph::{IndexType, NodeIndex};
use petgraph::{EdgeType, Graph};

/// Generates a complete graph with `n` nodes. A complete graph is a graph where
/// each node is connected to every other node. On a directed graph, this means
/// that each node has `n - 1` incoming edges and `n - 1` outgoing edges.
///
/// # Examples
/// ```
/// use petgraph_gen::complete_graph;
/// use petgraph::{Directed, Graph, Undirected};
///
/// let directed_graph: Graph<(), (), Directed> = complete_graph(10);
/// assert_eq!(directed_graph.node_count(), 10);
/// assert_eq!(directed_graph.edge_count(), 90);
///
/// let undirected_graph: Graph<(), (), Undirected> = complete_graph(10);
/// assert_eq!(undirected_graph.node_count(), 10);
/// assert_eq!(undirected_graph.edge_count(), 45);
/// ```
///
pub fn complete_graph<N: Default, E: Default, Ty: EdgeType, Ix: IndexType>(
    n: usize,
) -> Graph<N, E, Ty, Ix> {
    let mut graph = empty_graph_with_capacity(n, n, n * n);
    if n <= 1 {
        return graph;
    }
    for i in 0..n - 1 {
        for j in i + 1..n {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), Default::default());
            if Ty::is_directed() {
                graph.add_edge(NodeIndex::new(j), NodeIndex::new(i), Default::default());
            }
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::assert_graph_eq;
    use petgraph::graph::{DiGraph, UnGraph};

    #[test]
    fn test_complete_directed_graph() {
        let graph: DiGraph<(), (), u32> = complete_graph(4);
        let expected = Graph::from_edges(&[
            (0, 1),
            (1, 0),
            (0, 2),
            (2, 0),
            (0, 3),
            (3, 0),
            (1, 2),
            (2, 1),
            (1, 3),
            (3, 1),
            (2, 3),
            (3, 2),
        ]);
        assert_graph_eq(&graph, &expected);
    }

    #[test]
    fn test_complete_undirected_graph() {
        let graph: UnGraph<(), (), u16> = complete_graph(4);
        let expected = Graph::from_edges(&[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        assert_graph_eq(&graph, &expected);
    }
}
