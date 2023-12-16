use clap::{Parser, Subcommand};

mod day_1;
mod day_2;
// mod day_3;
// mod day_4;
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
}

fn main() {
    let args = Arguments::parse();
    match args.operation {
        Op::Day1 => day_1::main(),
        Op::Day2 => day_2::main(),
    }
}
