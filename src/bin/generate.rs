use clap::Parser;
use square_1_solver_rust::{state::{stateall::StateAll, statecs::StateCS, statesqsq::StateSqSq}, table::SliceCountTable};


/// Generate Slice Count Table for the Square-1 Solver
#[derive(Parser)]
struct Cli {
    /// The table to generate: cs, sqsq, all
    table: String,
    /// Disables Progress Output
    #[clap(long, short, action)]
    quiet: bool
}

fn main() {
    let args = Cli::parse();
    match args.table.as_str() {
        "cs" => {SliceCountTable::<StateCS>::new(!args.quiet).generate();}
        "sqsq" => {SliceCountTable::<StateSqSq>::new(!args.quiet).generate();}
        "all" => {SliceCountTable::<StateAll>::new(!args.quiet).generate();}
        _ => {}
    }
}