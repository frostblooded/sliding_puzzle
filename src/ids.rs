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

fn get_path(
    graph: &UnGraph<Grid, ()>,
    end_idx: NodeIndex,
    parents: &Vec<Option<NodeIndex>>,
) -> Vec<NodeIndex> {
    let mut path = vec![end_idx];
    let mut curr_idx = end_idx;

    while let Some(parent_idx) = parents[curr_idx.index()] {
        path.push(parent_idx);
        curr_idx = parent_idx;
    }

    path.reverse();
    path
}

fn f(graph: &UnGraph<Grid, Direction>, path_len: u32, current_node: NodeIndex) -> (u32, u32) {
    let current_grid = graph.node_weight(current_node).unwrap();
    let heuristic = current_grid.heuristic();
    (path_len + heuristic, heuristic)
}

pub fn find_solution_helper(
    graph: &UnGraph<Grid, Direction>,
    curr_idx: NodeIndex,
    depth: u32,
    f_limit: u32,
    output_path: &mut Option<Vec<(Direction, NodeIndex)>>,
) -> bool {
    let (f_val, h_val) = f(graph, depth, curr_idx);

    if f_val >= f_limit {
        return false;
    }

    if h_val == 0 {
        *output_path = Some(vec![]);
        return true;
    }

    for adj_idx in graph.neighbors(curr_idx) {
        if find_solution_helper(graph, adj_idx, depth + 1, f_limit, output_path) {
            if let Some(path) = output_path.as_mut() {
                let edge_idx = graph.find_edge(curr_idx, adj_idx).unwrap();
                path.push((graph.edge_weight(edge_idx).unwrap().clone(), adj_idx));
            }

            return true;
        }
    }

    false
}

pub fn find_solution(
    graph: &UnGraph<Grid, Direction>,
    start_idx: NodeIndex,
) -> Option<Vec<(Direction, NodeIndex)>> {
    let mut path = None;
    let (mut f_threshold, _) = f(&graph, 0, start_idx);

    loop {
        dbg!(f_threshold);

        if find_solution_helper(graph, start_idx, 0, f_threshold, &mut path) {
            break;
        }

        f_threshold += 1;
    }

    if let Some(p) = &mut path {
        p.reverse();
    }

    path
}
