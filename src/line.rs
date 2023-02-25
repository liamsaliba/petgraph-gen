use crate::common::empty_graph_with_capacity;
use petgraph::{
    graph::{IndexType, NodeIndex},
    EdgeType, Graph,
};

/// Generates a line graph with `n` nodes and `n - 1` edges.  Each node is connected to the node
/// immediately after it.
///
/// # Examples
/// ```
/// use petgraph::{Graph, Undirected};
/// use petgraph_gen::line_graph;
///
/// let graph: Graph<(), ()> = line_graph(5);
/// assert_eq!(graph.node_count(), 5);
/// assert_eq!(graph.edge_count(), 8);
///
/// let graph: Graph<(), (), Undirected> = line_graph(5);
/// assert_eq!(graph.node_count(), 5);
/// assert_eq!(graph.edge_count(), 4);
/// ```
pub fn line_graph<N: Default, E: Default, Ty: EdgeType, Ix: IndexType>(
    n: usize,
) -> Graph<N, E, Ty, Ix> {
    let edge_capacity = (n - 1) * if Ty::is_directed() { 2 } else { 1 };
    let mut graph = empty_graph_with_capacity(n, n, edge_capacity);
    for i in 0..n - 1 {
        graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), Default::default());
        if Ty::is_directed() {
            graph.add_edge(NodeIndex::new(i + 1), NodeIndex::new(i), Default::default());
        }
    }
    graph
}
