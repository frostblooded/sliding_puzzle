#[cfg(test)]
mod tests {
    use sliding_puzzle::grid::{Direction, Grid};

    fn grids(vectors: Vec<(Vec<u32>, Direction)>) -> Vec<(Grid, Direction)> {
        vectors
            .iter()
            .map(|(v, d)| (Grid::from_vec(-1, v).unwrap(), d));

        let mut res = vec![];

        for (v, d) in vectors {
            res.push((Grid::from_vec(-1, &v).unwrap(), d));
        }

        res
    }

    #[test]
    fn test_generating_adjacents() {
        let grid = Grid::from_vec(-1, &vec![1, 2, 3, 0]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![
            (vec![1, 2, 0, 3], Direction::Right),
            (vec![1, 0, 3, 2], Direction::Down),
        ]);
        assert_eq!(adjacents, expected_adjacents);

        let grid = Grid::from_vec(-1, &vec![0, 2, 3, 1]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![
            (vec![2, 0, 3, 1], Direction::Left),
            (vec![3, 2, 0, 1], Direction::Up),
        ]);
        assert_eq!(adjacents, expected_adjacents);

        let grid = Grid::from_vec(-1, &vec![1, 0, 3, 4, 5, 6, 7, 8, 2]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![
            (vec![0, 1, 3, 4, 5, 6, 7, 8, 2], Direction::Right),
            (vec![1, 3, 0, 4, 5, 6, 7, 8, 2], Direction::Left),
            (vec![1, 5, 3, 4, 0, 6, 7, 8, 2], Direction::Up),
        ]);
        assert_eq!(adjacents, expected_adjacents);
    }

    #[test]
    fn test_linear_pos_to_matrix_pos() {
        let grid = Grid::new(3, -1, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();

        assert_eq!(grid.linear_pos_to_matrix_pos(0), (0, 0));
        assert_eq!(grid.linear_pos_to_matrix_pos(3), (1, 0));
        assert_eq!(grid.linear_pos_to_matrix_pos(7), (2, 1));
        assert_eq!(grid.linear_pos_to_matrix_pos(8), (2, 2));
    }

    #[test]
    fn test_expected_pos() {
        let grid = Grid::new(3, -1, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();

        assert_eq!(grid.expected_pos(&1), (0, 0));
        assert_eq!(grid.expected_pos(&4), (1, 0));
        assert_eq!(grid.expected_pos(&8), (2, 1));

        // Test the zero
        assert_eq!(grid.expected_pos(&0), (2, 2));

        let grid = Grid::new(3, 4, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();
        assert_eq!(grid.expected_pos(&0), (1, 0));

        let grid = Grid::new(3, 6, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();
        assert_eq!(grid.expected_pos(&4), (1, 0));
        assert_eq!(grid.expected_pos(&0), (1, 2));
    }

    #[test]
    fn test_heuristic_for_index() {
        let grid = Grid::new(3, -1, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();

        for i in 0..=8 {
            assert_eq!(grid.heuristic_for_index(i), 0);
        }

        let grid = Grid::new(3, -1, &vec![1, 2, 4, 5, 6, 3, 7, 8, 0]).unwrap();
        assert_eq!(grid.heuristic_for_index(0), 0);
        assert_eq!(grid.heuristic_for_index(1), 0);
        assert_eq!(grid.heuristic_for_index(2), 3);
        assert_eq!(grid.heuristic_for_index(3), 1);
        assert_eq!(grid.heuristic_for_index(4), 1);
        assert_eq!(grid.heuristic_for_index(5), 1);
        assert_eq!(grid.heuristic_for_index(6), 0);
        assert_eq!(grid.heuristic_for_index(7), 0);
        assert_eq!(grid.heuristic_for_index(8), 0);
    }

    #[test]
    fn test_heuristic() {
        let grid = Grid::new(3, -1, &vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap();
        assert_eq!(grid.heuristic(), 0);

        let grid = Grid::new(3, -1, &vec![1, 2, 4, 5, 6, 3, 7, 8, 0]).unwrap();
        assert_eq!(grid.heuristic(), 6);
    }
}
