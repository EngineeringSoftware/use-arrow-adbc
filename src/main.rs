mod dummy;
mod snowflake_dynamic;
mod snowflake_static;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    example: Example,
}

#[derive(Debug, Clone, ValueEnum)]
enum Example {
    SnowflakeStatic,
    SnowflakeDynamic,
    Dummy,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.example {
        Example::SnowflakeStatic => snowflake_static::main(),
        Example::SnowflakeDynamic => snowflake_dynamic::main(),
        Example::Dummy => dummy::main(),
    }
}
