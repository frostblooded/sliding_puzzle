use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

use crate::grid::{Direction, Grid};

#[derive(Debug)]
pub struct GraphBuilder {
    pub graph: UnGraph<Grid, Direction>,
    pub weight_idx_hash: HashMap<Grid, NodeIndex>,
    processed_nodes: HashSet<NodeIndex>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            graph: UnGraph::new_undirected(),
            weight_idx_hash: HashMap::new(),
            processed_nodes: HashSet::new(),
        }
    }

    fn get_node_idx_by_weight(&self, weight: &Grid) -> Option<NodeIndex> {
        self.weight_idx_hash.get(weight).map(|x| x.clone())
    }

    pub fn generate_node_neighbors(&mut self, grid_idx: NodeIndex) {
        // Don't process the node if it has already been processed.
        if self.processed_nodes.contains(&grid_idx) {
            return;
        }

        let grid: &mut Grid = self.graph.node_weight_mut(grid_idx).unwrap();

        for (adj, dir) in grid.generate_neighbors() {
            // See if this node is already in the graph
            if let Some(adj_idx) = self.get_node_idx_by_weight(&adj) {
                // Add an edge from the current node to the adjacent node
                // if one doesn't exist yet.
                if !self.graph.contains_edge(grid_idx, adj_idx) {
                    self.graph.add_edge(grid_idx, adj_idx, dir);
                }

                continue;
            } else {
                // Just add the node to the graph
                let adj_idx = self.graph.add_node(adj.clone());
                self.weight_idx_hash.insert(adj, adj_idx);
                self.graph.add_edge(grid_idx, adj_idx, dir);
            }
        }

        self.processed_nodes.insert(grid_idx);
    }
}
