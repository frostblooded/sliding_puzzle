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

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Grid {
    data: Vec<u8>,
    grid_side: u8,
    zero_number: i8,
}

impl Grid {
    pub fn new(grid_side: u8, zero_number: i8, input: &Vec<u8>) -> Option<Self> {
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

    pub fn from_vec(zero_number: i8, input: &Vec<u8>) -> Option<Self> {
        let grid_side = (input.len() as f32).sqrt().ceil() as u8;
        Grid::new(grid_side, zero_number, input)
    }

    fn matrix_pos_to_linear_pos(&self, x: u8, y: u8) -> u8 {
        y * self.grid_side + x
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&u8> {
        self.data.get(self.matrix_pos_to_linear_pos(x, y) as usize)
    }

    pub fn current_zero_position(&self) -> (u8, u8) {
        for j in 0..self.grid_side {
            for i in 0..self.grid_side {
                if *self.get(i, j).unwrap() == 0 {
                    return (i, j);
                }
            }
        }

        unreachable!()
    }

    pub fn swap_indices(&mut self, (x1, y1): (u8, u8), (x2, y2): (u8, u8)) {
        let idx1 = self.matrix_pos_to_linear_pos(x1, y1) as usize;
        let idx2 = self.matrix_pos_to_linear_pos(x2, y2) as usize;
        self.data.swap(idx1, idx2);
    }

    pub fn clone_with_swapped_indices(&self, idx1: (u8, u8), idx2: (u8, u8)) -> Grid {
        let mut new_grid = self.clone();
        new_grid.swap_indices(idx1, idx2);
        new_grid
    }

    pub fn generate_neighbors(&self) -> Vec<(Grid, Direction)> {
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

    pub fn linear_pos_to_matrix_pos(&self, pos: u8) -> (u8, u8) {
        let x = pos / self.grid_side;
        let y = pos % self.grid_side;
        (x, y)
    }

    pub fn expected_pos(&self, val: &u8) -> (u8, u8) {
        let expected_idx = if *val == 0 {
            if self.zero_number >= 0 {
                self.zero_number as u8 - 1
            } else {
                // Zero number is -1
                self.data.len() as u8 - 1
            }
        } else {
            val - 1
        };

        self.linear_pos_to_matrix_pos(expected_idx)
    }

    pub fn heuristic_for_index(&self, idx: u8) -> u8 {
        let (current_x, current_y) = self.linear_pos_to_matrix_pos(idx);
        let (expected_x, expected_y) = self.expected_pos(self.data.get(idx as usize).unwrap());

        ((current_x as i8 - expected_x as i8).abs() + (current_y as i8 - expected_y as i8)).abs()
            as u8
    }

    pub fn heuristic(&self) -> u16 {
        (0..self.data.len())
            .map(|i| self.heuristic_for_index(i as u8) as u16)
            .sum()
    }

    pub fn is_unsolvable(&self) -> bool {
        let mut inversions = 0;

        for i in 0..self.data.len() {
            if self.data[i] == 0 {
                continue;
            }

            for j in (i + 1)..self.data.len() {
                if self.data[j] == 0 {
                    continue;
                }

                if self.data[i] > self.data[j] {
                    inversions += 1;
                }
            }
        }

        if self.grid_side % 2 == 0 {
            let (_, zero_y) = self.current_zero_position();
            (inversions + zero_y) % 2 == 0
        } else {
            inversions % 2 == 1
        }
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
