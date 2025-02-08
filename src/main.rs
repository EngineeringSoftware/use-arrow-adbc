mod dummy;
mod snowflake;
mod snowflake_dynamic;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    example: Example,
}

#[derive(Debug, Clone, ValueEnum)]
enum Example {
    Snowflake,
    SnowflakeDynamic,
    Dummy,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.example {
        Example::Snowflake => snowflake::main(),
        Example::SnowflakeDynamic => snowflake_dynamic::main(),
        Example::Dummy => dummy::main(),
    }
}
