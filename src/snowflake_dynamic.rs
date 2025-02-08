use adbc_core::{Connection, Statement};
use adbc_snowflake::{connection, database, Driver};
use arrow_array::{cast::AsArray, types::Decimal128Type};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut driver = Driver::try_load_dynamic()?;

    // Construct a database using environment variables
    let mut database = database::Builder::from_env()?.build(&mut driver)?;

    // Create a connection to the database
    let mut connection = connection::Builder::from_env()?.build(&mut database)?;

    // Construct a statement to execute a query
    let mut statement = connection.new_statement()?;

    // Execute a query
    statement.set_sql_query("SELECT 21 + 21")?;
    let mut reader = statement.execute()?;

    // Check the result
    let batch = reader.next().expect("a record batch")?;
    assert_eq!(
        batch.column(0).as_primitive::<Decimal128Type>().value(0),
        42
    );

    Ok(())
}
