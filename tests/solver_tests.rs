#[cfg(test)]
mod tests {
    use sliding_puzzle::solver::Solver;

    #[test]
    fn end_to_end_tests() {
        // 2x2
        let input = "3
            -1
            3 1
            2 0";
        let solution = Solver::solve(input);
        assert_eq!(solution, "4\nright\ndown\nleft\nup");

        // 3x3
        let input = "8
            -1
            1 2 3
            4 5 6
            7 8 0";
        let solution = Solver::solve(input);
        assert_eq!(solution, "0\n");

        let input = "8
            -1
            4 3 5
            6 0 1
            8 7 2";
        let solution = Solver::solve(input);
        assert_eq!(solution, "22\nright\nup\nleft\ndown\nleft\ndown\nright\nup\nright\ndown\nleft\nleft\nup\nup\nright\ndown\nleft\ndown\nright\nup\nleft\nup");
    }
}
