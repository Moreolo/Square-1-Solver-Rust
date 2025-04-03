use clap::Parser;
use square_1_solver_rust::{state::{stateall::StateAll, statecs::StateCS, statesqsq::StateSqSq}, table::SliceCountTable};


/// Generate Slice Count Table for the Square-1 Solver
#[derive(Parser)]
struct Cli {
    /// The table to generate: cs, sqsq, all
    table: String,
    /// Disables Progress Output
    #[clap(long, short, action)]
    quiet: bool,
    /// Uses less memory by using files
    #[clap(long, short, action)]
    limram: bool
}

fn main() {
    let args = Cli::parse();
    match args.table.as_str() {
        "cs" => {
            let table = SliceCountTable::<StateCS>::new(!args.quiet);
            if args.limram {
                table.generate_compact();
            } else {
                table.generate();
            }
        }
        "sqsq" => {
            let table = SliceCountTable::<StateSqSq>::new(!args.quiet);
            if args.limram {
                table.generate_compact();
            } else {
                table.generate();
            }
        }
        "all" => {
            let table = SliceCountTable::<StateAll>::new(!args.quiet);
            if args.limram {
                table.generate_compact();
            } else {
                table.generate();
            }
        }
        _ => {}
    }
}