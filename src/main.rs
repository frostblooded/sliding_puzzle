mod grid;
use grid::{Direction, Grid};

mod ids;
use ids::find_solution;

use log::debug;

use std::collections::{HashMap, VecDeque};
use std::io::stdin;
use std::time::SystemTime;

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

fn get_node_idx_by_weight(
    weight_idx_hash: &HashMap<Grid, NodeIndex>,
    weight: &Grid,
) -> Option<NodeIndex> {
    weight_idx_hash.get(weight).map(|x| x.clone())
}

fn generate_graph(grid: &Grid) -> (UnGraph<Grid, Direction>, NodeIndex) {
    let mut graph: UnGraph<Grid, Direction> = UnGraph::new_undirected();
    let mut queue = VecDeque::<NodeIndex>::new();
    let mut weight_idx_hash: HashMap<Grid, NodeIndex> = HashMap::new();

    let start_idx = graph.add_node(grid.clone());
    let mut curr_idx = start_idx;
    weight_idx_hash.insert(grid.clone(), curr_idx);

    loop {
        let start_handle_node = SystemTime::now();
        let curr_grid = graph.node_weight(curr_idx).unwrap();

        for (adj, dir) in curr_grid.generate_adjacent_grids() {
            let start_get_node_by_index = SystemTime::now();

            // See if this node is already in the graph
            if let Some(adj_idx) = get_node_idx_by_weight(&weight_idx_hash, &adj) {
                let end_get_node_by_undex = SystemTime::now();

                debug!(
                    "Node {}: Searching for node idx by weight done in {:?}",
                    curr_idx.index(),
                    end_get_node_by_undex
                        .duration_since(start_get_node_by_index)
                        .unwrap()
                );

                let start_contains_edge = SystemTime::now();

                // Add an edge from the current node to the adjacent node
                // if one doesn't exist yet.
                if !graph.contains_edge(curr_idx, adj_idx) {
                    let end_contains_edge = SystemTime::now();

                    debug!(
                        "Node {}: Checking if graph contains edge in {:?}",
                        curr_idx.index(),
                        end_contains_edge
                            .duration_since(start_contains_edge)
                            .unwrap()
                    );

                    graph.add_edge(curr_idx, adj_idx, dir);
                }

                continue;
            } else {
                let end_get_node_by_undex = SystemTime::now();

                debug!(
                    "Node {}: Searching for node idx by weight done in {:?}",
                    curr_idx.index(),
                    end_get_node_by_undex
                        .duration_since(start_get_node_by_index)
                        .unwrap()
                );

                // Just add the node to the graph
                let adj_idx = graph.add_node(adj.clone());
                weight_idx_hash.insert(adj, adj_idx);
                graph.add_edge(curr_idx, adj_idx, dir);
                queue.push_back(adj_idx);
            }
        }

        if queue.is_empty() {
            break;
        }

        curr_idx = queue.pop_front().unwrap();
        let end_handle_node = SystemTime::now();

        debug!(
            "Node {}: Handling in {:?}",
            curr_idx.index(),
            end_handle_node.duration_since(start_handle_node).unwrap()
        );
    }

    (graph, start_idx)
}

fn main() {
    simple_logger::init().expect("Failed to initialize simple_logger");

    let starting_grid = input_to_grid();
    //dbg!(&starting_grid);

    let (graph, starting_grid_idx) = generate_graph(&starting_grid);
    //dbg!(&graph);
    //dbg!(starting_grid_idx);

    println!(
        "{}",
        find_solution(&graph, starting_grid_idx)
            .unwrap()
            .iter()
            .map(|(d, _)| d.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
