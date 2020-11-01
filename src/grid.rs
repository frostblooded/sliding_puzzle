use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Up => write!(f, "up"),
            Direction::Right => write!(f, "right"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Grid {
    data: Vec<u32>,
    grid_side: u32,
    zero_number: i32,
}

impl Grid {
    pub fn new(grid_side: u32, zero_number: i32, input: &Vec<u32>) -> Option<Self> {
        let mut grid_data = Vec::new();

        for i in 0..grid_side {
            for j in 0..grid_side {
                grid_data.push(*input.get((i * grid_side + j) as usize)?);
            }
        }

        Some(Grid {
            data: grid_data,
            grid_side,
            zero_number: zero_number,
        })
    }

    pub fn from_vec(zero_number: i32, input: &Vec<u32>) -> Option<Self> {
        let grid_side = (input.len() as f32).sqrt().ceil() as u32;
        Grid::new(grid_side, zero_number, input)
    }

    fn matrix_pos_to_linear_pos(&self, x: u32, y: u32) -> u32 {
        y * self.grid_side + x
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&u32> {
        self.data.get(self.matrix_pos_to_linear_pos(x, y) as usize)
    }

    pub fn current_zero_position(&self) -> (u32, u32) {
        for j in 0..self.grid_side {
            for i in 0..self.grid_side {
                if *self.get(i, j).unwrap() == 0 {
                    return (i, j);
                }
            }
        }

        unreachable!()
    }

    pub fn swap_indices(&mut self, (x1, y1): (u32, u32), (x2, y2): (u32, u32)) {
        let idx1 = self.matrix_pos_to_linear_pos(x1, y1) as usize;
        let idx2 = self.matrix_pos_to_linear_pos(x2, y2) as usize;
        self.data.swap(idx1, idx2);
    }

    pub fn clone_with_swapped_indices(&self, idx1: (u32, u32), idx2: (u32, u32)) -> Grid {
        let mut new_grid = self.clone();
        new_grid.swap_indices(idx1, idx2);
        new_grid
    }

    pub fn generate_adjacent_grids(&self) -> Vec<(Grid, Direction)> {
        let mut results: Vec<(Grid, Direction)> = vec![];
        let (x, y) = self.current_zero_position();

        if x > 0 {
            results.push((
                self.clone_with_swapped_indices((x, y), (x - 1, y)),
                Direction::Right,
            ));
        }

        if x < self.grid_side - 1 {
            results.push((
                self.clone_with_swapped_indices((x, y), (x + 1, y)),
                Direction::Left,
            ));
        }

        if y > 0 {
            results.push((
                self.clone_with_swapped_indices((x, y), (x, y - 1)),
                Direction::Down,
            ));
        }

        if y < self.grid_side - 1 {
            results.push((
                self.clone_with_swapped_indices((x, y), (x, y + 1)),
                Direction::Up,
            ));
        }

        results
    }

    pub fn linear_pos_to_matrix_pos(&self, pos: u32) -> (u32, u32) {
        let x = pos / self.grid_side;
        let y = pos % self.grid_side;
        (x, y)
    }

    pub fn expected_pos(&self, val: &u32) -> (u32, u32) {
        let expected_idx = if *val == 0 {
            if self.zero_number >= 0 {
                self.zero_number as u32 - 1
            } else {
                // Zero number is -1
                self.data.len() as u32 - 1
            }
        } else {
            val - 1
        };

        self.linear_pos_to_matrix_pos(expected_idx)
    }

    pub fn heuristic_for_index(&self, idx: u32) -> u32 {
        let (current_x, current_y) = self.linear_pos_to_matrix_pos(idx);
        let (expected_x, expected_y) = self.expected_pos(self.data.get(idx as usize).unwrap());

        ((current_x as i32 - expected_x as i32).abs() + (current_y as i32 - expected_y as i32))
            .abs() as u32
    }

    pub fn heuristic(&self) -> u32 {
        (0..self.data.len())
            .map(|i| self.heuristic_for_index(i as u32))
            .sum()
    }
}

impl Debug for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        for j in 0..self.grid_side {
            let mut row_numbers = vec![];

            for i in 0..self.grid_side {
                row_numbers.push(self.get(i, j).unwrap().to_string());
            }

            string.push_str("  ");
            string.push_str(&row_numbers.join(","));

            if j < self.grid_side - 1 {
                string.push('\n');
            }
        }

        write!(formatter, "Grid: [\n{}\n]", string)
    }
}
