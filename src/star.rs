use petgraph::{graph::IndexType, EdgeType, Graph};

/// Generates a star graph with a single center node connected to `n` other nodes. The resulting
/// graph has `n + 1` nodes and `n` edges.
///
/// # Examples
/// ```
/// use petgraph_gen::star_graph;
/// use petgraph::{Directed, Graph, Undirected};
/// use petgraph::graph::NodeIndex;
/// use petgraph::visit::EdgeRef;
///
/// let graph: Graph<(), ()> = star_graph(10);
/// assert_eq!(graph.node_count(), 11);
/// assert_eq!(graph.edge_count(), 10);
/// let center_node = NodeIndex::new(0);
/// assert_eq!(graph.edges(center_node).count(), 10);
/// for edge in graph.edges(center_node) {
///    assert_eq!(edge.source(), center_node);
/// }
///
pub fn star_graph<N: Default, E: Default, Ty: EdgeType, Ix: IndexType>(
    n: usize,
) -> Graph<N, E, Ty, Ix> {
    let mut graph = Graph::<N, E, Ty, Ix>::with_capacity(n + 1, n);
    let center = graph.add_node(Default::default());
    for _ in 0..n {
        let node = graph.add_node(Default::default());
        graph.add_edge(center, node, Default::default());
    }
    graph
}
