use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashSet;

fn create_graph_from_adjacency_matrix(matrix: &Vec<Vec<u8>>) -> DiGraph<(), ()> {
    let mut graph = DiGraph::new();

    for (i, row) in matrix.iter().enumerate() {
        let node_from = graph.add_node(());
    }

    for (i, row) in matrix.iter().enumerate() {
        let node_i = NodeIndex::new(i);
        for (j, &value) in row.iter().enumerate() {
            if value == 1 {
                let node_j = NodeIndex::new(j);
                graph.add_edge(node_i, node_j, ());
            }
        }
    }
    graph
}

fn find_loops(graph: &DiGraph<(), ()>, v: NodeIndex, stack: &mut Vec<NodeIndex>, loops: &mut Vec<(NodeIndex, NodeIndex)> ) {
    stack.push(v);
    for w in graph.neighbors(v) {
        if stack.contains(&w) {
            loops.push((v, w));
        } else if !stack.contains(&w) {
            find_loops(graph, w, stack, loops);
        }
    }
}

fn main() {

    let adj_matrix: Vec<Vec<u8>> = vec![
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
    ];
    // Create a graph from the adjacency matrix
    let graph = create_graph_from_adjacency_matrix(&adj_matrix);
    println!("Graph: {:?}", graph);

    let mut stack = Vec::new();
    let mut loops = Vec::new();
    let start_node = NodeIndex::new(0); // Set the start node index

    find_loops(&graph, start_node, &mut stack, &mut loops);

    println!("Loops found: {:?}", loops);
}
