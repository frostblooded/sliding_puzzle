use crate::graph_builder::GraphBuilder;
use crate::grid::{Direction, Grid};

use std::cmp::{Ord, Ordering};

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    idx: NodeIndex,
    path_len: u32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn f(graph: &UnGraph<Grid, Direction>, path_len: u32, current_node: NodeIndex) -> (u32, u32) {
    let current_grid = graph.node_weight(current_node).unwrap();
    let heuristic = current_grid.heuristic();
    (path_len + heuristic, heuristic)
}

pub fn find_solution_helper(
    graph_builder: &mut GraphBuilder,
    curr_idx: NodeIndex,
    depth: u32,
    f_limit: u32,
    output_path: &mut Option<Vec<(Direction, NodeIndex)>>,
) -> bool {
    let (f_val, h_val) = f(&graph_builder.graph, depth, curr_idx);

    if f_val >= f_limit {
        return false;
    }

    if h_val == 0 {
        *output_path = Some(vec![]);
        return true;
    }

    graph_builder.generate_node_neighbors(curr_idx);
    let neighbors: Vec<NodeIndex> = graph_builder.graph.neighbors(curr_idx).collect();

    for adj_idx in neighbors {
        if find_solution_helper(graph_builder, adj_idx, depth + 1, f_limit, output_path) {
            if let Some(path) = output_path.as_mut() {
                let edge_idx = graph_builder.graph.find_edge(curr_idx, adj_idx).unwrap();
                path.push((
                    graph_builder.graph.edge_weight(edge_idx).unwrap().clone(),
                    adj_idx,
                ));
            }

            return true;
        }
    }

    false
}

pub fn find_solution(start_grid: Grid) -> Option<Vec<(Direction, NodeIndex)>> {
    let mut path = None;
    let mut graph_builder = GraphBuilder::new();
    let start_idx = graph_builder.graph.add_node(start_grid.clone());
    graph_builder.weight_idx_hash.insert(start_grid, start_idx);
    let (mut f_threshold, _) = f(&graph_builder.graph, 0, start_idx);

    loop {
        dbg!(f_threshold);

        if find_solution_helper(&mut graph_builder, start_idx, 0, f_threshold, &mut path) {
            break;
        }

        f_threshold += 1;
    }

    if let Some(p) = &mut path {
        p.reverse();
    }

    path
}
