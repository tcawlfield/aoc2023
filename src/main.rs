use clap::{Parser, Subcommand};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
// mod day_5;
// mod day_6;
// mod day_7;
// mod day_8;
// mod day_9;
// mod day_10;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    operation: Op,
}

#[derive(Subcommand)]
enum Op {
    Day1,
    Day2,
    Day3,
    Day4,
    // Day5,
}

fn main() {
    let args = Arguments::parse();
    match args.operation {
        Op::Day1 => day_1::main(),
        Op::Day2 => day_2::main(),
        Op::Day3 => day_3::main(),
        Op::Day4 => day_4::main(),
        // Op::Day5 => day_5::main(),
    }
}
