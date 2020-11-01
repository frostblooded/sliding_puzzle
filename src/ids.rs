use crate::grid::Grid;
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, VecDeque};

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

pub fn find_solution(graph: &UnGraph<Grid, ()>, start_idx: NodeIndex) -> Option<Vec<NodeIndex>> {
    fn f(graph: &UnGraph<Grid, ()>, path_len: u32, current_node: NodeIndex) -> (u32, u32) {
        let current_grid = graph.node_weight(current_node).unwrap();
        let heuristic = current_grid.heuristic();
        (path_len + heuristic, heuristic)
    }

    let mut curr_idx;
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    queue.push(State {
        cost: f(&graph, 0, start_idx).0,
        idx: start_idx,
        path_len: 0,
    });

    let num_nodes = graph.node_indices().len();

    let mut checked: Vec<bool> = vec![false; num_nodes];
    checked[0] = true;
    let mut parents: Vec<Option<NodeIndex>> = vec![None; num_nodes];

    while !queue.is_empty() {
        let curr_state = queue.pop().unwrap();
        curr_idx = curr_state.idx;
        dbg!(curr_idx);

        for adj_idx in graph.neighbors(curr_idx) {
            if checked[adj_idx.index()] {
                continue;
            }

            dbg!(adj_idx);
            parents[adj_idx.index()] = Some(curr_idx);

            let (f_val, h_val) = f(&graph, curr_state.path_len, adj_idx);

            // If the heurisitc returns 0, we have found the goal
            if h_val == 0 {
                return Some(get_path(&graph, adj_idx, &parents));
            }

            queue.push(State {
                cost: f_val,
                idx: adj_idx,
                path_len: curr_state.path_len + 1,
            });

            checked[adj_idx.index()] = true;
        }

        dbg!(&queue);
    }

    None
}
