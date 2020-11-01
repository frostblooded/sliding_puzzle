use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub struct Grid {
    data: Vec<u32>,
    grid_side: u32,
}

impl Grid {
    pub fn new(grid_side: u32, input: &Vec<u32>) -> Option<Self> {
        let mut grid_data = Vec::new();

        for i in 0..grid_side {
            for j in 0..grid_side {
                grid_data.push(*input.get((i * grid_side + j) as usize)?);
            }
        }

        Some(Grid {
            data: grid_data,
            grid_side,
        })
    }

    pub fn from_vec(input: &Vec<u32>) -> Option<Self> {
        let grid_side = (input.len() as f32).sqrt().ceil() as u32;
        Grid::new(grid_side, input)
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&u32> {
        self.data.get((x * self.grid_side + y) as usize)
    }

    pub fn current_zero_position(&self) -> (u32, u32) {
        for i in 0..self.grid_side {
            for j in 0..self.grid_side {
                if *self.get(i, j).unwrap() == 0 {
                    return (i, j);
                }
            }
        }

        unreachable!()
    }

    pub fn swap_indices(&mut self, (x1, y1): (u32, u32), (x2, y2): (u32, u32)) {
        let idx1 = (x1 * self.grid_side + y1) as usize;
        let idx2 = (x2 * self.grid_side + y2) as usize;
        self.data.swap(idx1, idx2);
    }

    pub fn clone_with_swapped_indices(&self, idx1: (u32, u32), idx2: (u32, u32)) -> Grid {
        let mut new_grid = self.clone();
        new_grid.swap_indices(idx1, idx2);
        new_grid
    }

    pub fn generate_adjacent_grids(&self) -> Vec<Grid> {
        let mut results: Vec<Grid> = vec![];
        let (x, y) = self.current_zero_position();

        if x > 0 {
            results.push(self.clone_with_swapped_indices((x, y), (x - 1, y)));
        }

        if x < self.grid_side - 1 {
            results.push(self.clone_with_swapped_indices((x, y), (x + 1, y)));
        }

        if y > 0 {
            results.push(self.clone_with_swapped_indices((x, y), (x, y - 1)));
        }

        if y < self.grid_side - 1 {
            results.push(self.clone_with_swapped_indices((x, y), (x, y + 1)));
        }

        results
    }
}

impl Debug for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        for i in 0..self.grid_side {
            let mut row_numbers = vec![];

            for j in 0..self.grid_side {
                row_numbers.push(self.get(i, j).unwrap().to_string());
            }

            string.push_str(&row_numbers.join(","));

            if i < self.grid_side - 1 {
                string.push('\n');
            }
        }

        write!(formatter, "\nGrid: [\n{}\n]\n", string)
    }
}
