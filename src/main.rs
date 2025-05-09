mod dummy;
mod snowflake_dynamic;
mod snowflake_static;

use clap::{Parser, ValueEnum};
use tokio;

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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();
    //    match args.example {
    //        Example::SnowflakeStatic => snowflake_static::main(),
    //        Example::SnowflakeDynamic => snowflake_dynamic::main(),
    //        Example::Dummy => dummy::main(),
    //    }
    snowflake_dynamic::main().await
}
