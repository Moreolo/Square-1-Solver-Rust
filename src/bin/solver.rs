use std::time::Instant;

use square_1_solver_rust::{solver::{load_table, solve}, square1::Square1, table::format_duration};


fn main() {
    load_table();
    // let (square1, bar_solved) = (Square1::from_arr([2, 7, 13, 15, 5, 1, 14, 0, 11, 12, 3, 9, 10, 6, 8, 4]), false);
    // let (square1, bar_solved) = (Square1::from_arr([0, 5, 2, 1, 4, 3, 6, 7, 9, 8, 11, 10, 13, 12, 15, 14]), false);
    // let (square1, bar_solved) = (Square1::solved(), false);
    let (square1, bar_solved) = Square1::scrambled();
    println!("{square1:?}");
    println!("Solving");
    let now = Instant::now();
    let solution = solve(square1, bar_solved).expect("Square-1 invalid");
    let elapsed = now.elapsed();
    println!("Found solution in {}", format_duration(elapsed));
    println!("Solution: {}", solution);
    println!("Scramble: {}", solution.inverse());
}