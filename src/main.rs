mod grid;

use grid::Grid;
use std::io::stdin;

use petgraph::graph::Graph;

fn input_to_grid() -> (Grid, i32) {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");
    let numbers_count: u32 = buf.trim().parse().expect("Couldn't parse input as number");

    buf.clear();
    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");
    let zero_number: i32 = buf.trim().parse().expect("Couldn't parse input as number");

    let grid_side = (numbers_count as f32).sqrt().ceil() as u32;

    buf.clear();
    for _ in 0..grid_side {
        stdin()
            .read_line(&mut buf)
            .expect("Couldn't read from stdin");
    }

    let split_numbers: Vec<u32> = buf
        .split_whitespace()
        .map(|x| x.parse().expect("Couldn't parse input as number"))
        .collect();

    (
        Grid::new(grid_side, &split_numbers).expect("Couldn't create grid from input"),
        zero_number,
    )
}

fn main() {
    let (grid, zero_number) = input_to_grid();
    let mut graph = Graph::<Grid, ()>::new();
    graph.add_node(grid);

    println!("{:?}", graph);
}
