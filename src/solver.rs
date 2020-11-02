use crate::grid::{Direction, Grid};
use crate::ids::find_solution;

use log::debug;

use std::collections::{HashMap, VecDeque};
use std::time::SystemTime;

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

pub struct Solver {}

impl Solver {
    fn input_to_grid(input: &str) -> Grid {
        let mut lines = input.lines();

        let line = lines.next().expect("Couldn't read from input");
        let numbers_count: u32 = line.trim().parse().expect("Couldn't parse input as number");
        let grid_side = (numbers_count as f32).sqrt().ceil() as u32;

        let line = lines.next().expect("Couldn't read from input");
        let zero_number: i32 = line.trim().parse().expect("Couldn't parse input as number");

        let mut numbers_buf = String::new();

        for _ in 0..grid_side {
            numbers_buf.push_str(lines.next().expect("Couldn't read from input"));
            numbers_buf.push('\n');
        }

        let split_numbers: Vec<u32> = numbers_buf
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
                if let Some(adj_idx) = Solver::get_node_idx_by_weight(&weight_idx_hash, &adj) {
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

    pub fn solve(input: &str) -> String {
        let starting_grid = Solver::input_to_grid(input);
        //dbg!(&starting_grid);

        let (graph, starting_grid_idx) = Solver::generate_graph(&starting_grid);
        //dbg!(&graph);
        //dbg!(starting_grid_idx);

        let direction_strings = find_solution(&graph, starting_grid_idx)
            .unwrap()
            .iter()
            .map(|(d, _)| d.to_string())
            .collect::<Vec<String>>();

        format!(
            "{}\n{}",
            direction_strings.len(),
            direction_strings.join(" ")
        )
    }
}
