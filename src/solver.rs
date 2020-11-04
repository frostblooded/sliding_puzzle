use crate::grid::Grid;
use crate::ids;

pub struct Solver {}

impl Solver {
    fn input_to_grid(input: &str) -> Grid {
        let mut lines = input.lines();

        let line = lines.next().expect("Couldn't read from input");
        let numbers_count: u8 = line.trim().parse().expect("Couldn't parse input as number");
        let grid_side = (numbers_count as f32).sqrt().ceil() as u8;

        let line = lines.next().expect("Couldn't read from input");
        let zero_number: i8 = line.trim().parse().expect("Couldn't parse input as number");

        let mut numbers_buf = String::new();

        for _ in 0..grid_side {
            numbers_buf.push_str(lines.next().expect("Couldn't read from input"));
            numbers_buf.push('\n');
        }

        let split_numbers: Vec<u8> = numbers_buf
            .split_whitespace()
            .map(|x| x.parse().expect("Couldn't parse input as number"))
            .collect();

        Grid::new(grid_side, zero_number, &split_numbers).expect("Couldn't create grid from input")
    }

    pub fn solve(input: &str) -> String {
        let starting_grid = Solver::input_to_grid(input);

        if starting_grid.is_unsolvable() {
            return "No solution".to_string();
        }

        let direction_strings = ids::find_solution(starting_grid)
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
