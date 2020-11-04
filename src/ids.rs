use crate::grid::{Direction, Grid};

use std::collections::HashSet;

fn f(grid: &Grid, path_len: u16) -> (u16, u16) {
    let heuristic = grid.heuristic();
    (path_len + heuristic, heuristic)
}

pub fn find_solution_helper<'a>(
    grid: &Grid,
    depth: u16,
    f_limit: u16,
    used: &mut HashSet<Grid>,
    output_path: &mut Option<Vec<(Direction, Grid)>>,
) -> bool {
    let (f_val, h_val) = f(&grid, depth);
    let mut found_goal = false;
    used.insert(grid.clone());

    if f_val > f_limit {
        found_goal = false
    } else if h_val == 0 {
        *output_path = Some(vec![]);
        found_goal = true
    } else {
        let neighbors = grid.generate_neighbors();

        for (adj_grid, adj_dir) in neighbors {
            // If the node is in the current path, don't go to it.
            // This avoids creating loops while traversing.
            if used.contains(&adj_grid) {
                continue;
            }

            if find_solution_helper(&adj_grid, depth + 1, f_limit, used, output_path) {
                if let Some(path) = output_path.as_mut() {
                    path.push((adj_dir, adj_grid));
                }

                found_goal = true;
                break;
            }
        }
    };

    used.remove(&grid);
    found_goal
}

pub fn find_solution(start_grid: Grid) -> Option<Vec<(Direction, Grid)>> {
    let mut path = None;
    let (mut f_threshold, _) = f(&start_grid, 0);
    let mut used: HashSet<Grid> = HashSet::new();

    loop {
        if find_solution_helper(&start_grid, 0, f_threshold, &mut used, &mut path) {
            break;
        }

        f_threshold += 1;
    }

    if let Some(p) = &mut path {
        p.reverse();
    }

    path
}
