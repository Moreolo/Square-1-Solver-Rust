use square_1_solver_rust::{square1::Square1, state::{stateall::StateAll, State}};
use std::time::Instant;
use square_1_solver_rust::table::postable::PosTable;

fn main() {
    let sq1 = Square1::from_num(2837015813540066400);
    let mut state = StateAll::new(sq1);
    println!("State All: {}", state.get_index());
    println!("Sym Indecies: {:?}", state.get_symmetric_indecies());

    // fs::read("temp/opened_0.bin").expect("Unable to read file");
    let size = 500_000_000;
    let now = Instant::now();
    let mut tab = PosTable::new("testmenen", size);
    {
        let mut to_write = vec![255; size];
        tab.write_chunk(&mut to_write);
    }
    
    while let Some(_) = tab.read_chunk(size) {
        println!("Table read");
    }
    let elapsed = now.elapsed();
    println!("Finished generating Table in {:?}", elapsed);
}
