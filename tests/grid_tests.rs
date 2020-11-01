#[cfg(test)]
mod tests {
    use sliding_puzzle::grid::Grid;

    fn grids(vectors: Vec<Vec<u32>>) -> Vec<Grid> {
        vectors.iter().map(|v| Grid::from_vec(v).unwrap()).collect()
    }

    #[test]
    fn test_generating_adjacents() {
        let grid = Grid::from_vec(&vec![1, 2, 3, 0]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![vec![1, 0, 3, 2], vec![1, 2, 0, 3]]);
        assert_eq!(adjacents, expected_adjacents);

        let grid = Grid::from_vec(&vec![0, 2, 3, 1]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![vec![3, 2, 0, 1], vec![2, 0, 3, 1]]);
        assert_eq!(adjacents, expected_adjacents);

        let grid = Grid::from_vec(&vec![1, 0, 3, 4, 5, 6, 7, 8, 2]).unwrap();
        let adjacents = grid.generate_adjacent_grids();
        let expected_adjacents = grids(vec![
            vec![1, 5, 3, 4, 0, 6, 7, 8, 2],
            vec![0, 1, 3, 4, 5, 6, 7, 8, 2],
            vec![1, 3, 0, 4, 5, 6, 7, 8, 2],
        ]);
        assert_eq!(adjacents, expected_adjacents);
    }
}
