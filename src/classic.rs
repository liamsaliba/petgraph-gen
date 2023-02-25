use crate::common::empty_graph_with_capacity;
use petgraph::graph::IndexType;
use petgraph::{EdgeType, Graph};

/// Generates an empty graph with `n` nodes and no edges.
///
/// # Examples
/// ```
/// use petgraph::Graph;
/// use petgraph_gen::empty_graph;
///
/// let graph: Graph<(), ()> = empty_graph(5);
/// assert_eq!(graph.node_count(), 5);
/// assert_eq!(graph.edge_count(), 0);
/// ```
pub fn empty_graph<N: Default, E: Default, Ty: EdgeType, Ix: IndexType>(
    n: usize,
) -> Graph<N, E, Ty, Ix> {
    empty_graph_with_capacity(n, n, 0)
}
