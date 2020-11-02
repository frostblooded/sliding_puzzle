use sliding_puzzle::solver::Solver;

use std::io::stdin;

fn main() {
    //simple_logger::init().expect("Failed to initialize simple_logger");

    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");
    let numbers_count: u32 = buf.trim().parse().expect("Couldn't parse input as number");
    let grid_side = (numbers_count as f32).sqrt().ceil() as u32;

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");

    for _ in 0..grid_side {
        stdin()
            .read_line(&mut buf)
            .expect("Couldn't read from stdin");
    }

    println!("{}", Solver::solve(&buf));
}
