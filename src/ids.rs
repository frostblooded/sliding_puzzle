use crate::graph_builder::GraphBuilder;
use crate::grid::{Direction, Grid};

use std::collections::HashSet;

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

fn f(graph: &UnGraph<Grid, Direction>, path_len: u16, current_node: NodeIndex) -> (u16, u16) {
    let current_grid = graph.node_weight(current_node).unwrap();
    let heuristic = current_grid.heuristic();
    (path_len + heuristic, heuristic)
}

pub fn find_solution_helper(
    graph_builder: &mut GraphBuilder,
    curr_idx: NodeIndex,
    depth: u16,
    f_limit: u16,
    used: &mut HashSet<NodeIndex>,
    output_path: &mut Option<Vec<(Direction, NodeIndex)>>,
) -> bool {
    let (f_val, h_val) = f(&graph_builder.graph, depth, curr_idx);
    let mut found_goal = false;
    used.insert(curr_idx);

    if f_val >= f_limit {
        found_goal = false
    } else if h_val == 0 {
        *output_path = Some(vec![]);
        found_goal = true
    } else {
        graph_builder.generate_node_neighbors(curr_idx);
        let neighbors: Vec<NodeIndex> = graph_builder.graph.neighbors(curr_idx).collect();

        for adj_idx in neighbors {
            // If the node is in the current path, don't go to it.
            // This avoids creating loops while traversing.
            if used.contains(&adj_idx) {
                continue;
            }

            if find_solution_helper(
                graph_builder,
                adj_idx,
                depth + 1,
                f_limit,
                used,
                output_path,
            ) {
                if let Some(path) = output_path.as_mut() {
                    let edge_idx = graph_builder.graph.find_edge(curr_idx, adj_idx).unwrap();
                    path.push((
                        graph_builder.graph.edge_weight(edge_idx).unwrap().clone(),
                        adj_idx,
                    ));
                }

                found_goal = true;
                break;
            }
        }
    };

    used.remove(&curr_idx);
    found_goal
}

pub fn find_solution(start_grid: Grid) -> Option<Vec<(Direction, NodeIndex)>> {
    let mut path = None;
    let mut graph_builder = GraphBuilder::new();
    let start_idx = graph_builder.graph.add_node(start_grid.clone());
    graph_builder.weight_idx_hash.insert(start_grid, start_idx);
    let (mut f_threshold, _) = f(&graph_builder.graph, 0, start_idx);
    let mut used: HashSet<NodeIndex> = HashSet::new();

    loop {
        dbg!(f_threshold);

        if find_solution_helper(
            &mut graph_builder,
            start_idx,
            0,
            f_threshold,
            &mut used,
            &mut path,
        ) {
            break;
        }

        f_threshold += 1;
    }

    if let Some(p) = &mut path {
        p.reverse();
    }

    path
}
