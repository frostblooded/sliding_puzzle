mod grid;
use grid::{Direction, Grid};

mod ids;
use ids::find_solution;

use std::collections::VecDeque;
use std::io::stdin;

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

fn input_to_grid() -> Grid {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");
    let numbers_count: u32 = buf.trim().parse().expect("Couldn't parse input as number");

    buf.clear();
    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");
    let zero_number: i32 = buf.trim().parse().expect("Couldn't parse input as number");

    let grid_side = (numbers_count as f32).sqrt().ceil() as u32;

    buf.clear();
    for _ in 0..grid_side {
        stdin()
            .read_line(&mut buf)
            .expect("Couldn't read from stdin");
    }

    let split_numbers: Vec<u32> = buf
        .split_whitespace()
        .map(|x| x.parse().expect("Couldn't parse input as number"))
        .collect();

    Grid::new(grid_side, zero_number, &split_numbers).expect("Couldn't create grid from input")
}

fn get_node_idx_by_weight(graph: &UnGraph<Grid, Direction>, weight: &Grid) -> Option<NodeIndex> {
    graph
        .node_indices()
        .find(|x| graph.node_weight(*x).unwrap() == weight)
}

fn generate_graph(grid: &Grid) -> UnGraph<Grid, Direction> {
    let mut graph: UnGraph<Grid, Direction> = UnGraph::new_undirected();
    let mut queue = VecDeque::<NodeIndex>::new();
    let mut curr_idx = graph.add_node(grid.clone());

    loop {
        let curr_grid = graph.node_weight(curr_idx).unwrap();

        for (adj, dir) in curr_grid.generate_adjacent_grids() {
            // See if this node is already in the graph
            if let Some(adj_idx) = get_node_idx_by_weight(&graph, &adj) {
                // Add an edge from the current node to the adjacent node
                // if one doesn't exist yet.
                if !graph.contains_edge(curr_idx, adj_idx) {
                    graph.add_edge(curr_idx, adj_idx, dir);
                }

                continue;
            } else {
                // Just add the node to the graph
                let adj_idx = graph.add_node(adj);
                graph.add_edge(curr_idx, adj_idx, dir);
                queue.push_back(adj_idx);
            }
        }

        if queue.is_empty() {
            break;
        }

        curr_idx = queue.pop_front().unwrap();
    }

    graph
}

fn main() {
    let starting_grid = input_to_grid();
    let graph = generate_graph(&starting_grid);
    let starting_grid_idx = get_node_idx_by_weight(&graph, &starting_grid).unwrap();
    dbg!(&graph);

    println!("{:?}", find_solution(&graph, starting_grid_idx));
}
