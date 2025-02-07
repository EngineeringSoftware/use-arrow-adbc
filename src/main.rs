mod dummy;
mod snowflake;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    example: Example,
}

#[derive(Debug, Clone, ValueEnum)]
enum Example {
    Snowflake,
    Dummy,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.example {
        Example::Snowflake => snowflake::main(),
        Example::Dummy => dummy::main(),
    }
}
