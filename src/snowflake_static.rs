use adbc_core::{Connection, Statement};
use adbc_snowflake::{connection, database, Driver};
use arrow_array::{cast::AsArray, types::Decimal128Type};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the driver
    let mut driver = Driver::try_load()?;

    // Construct a database using environment variables
    let mut database = database::Builder::from_env()?.build(&mut driver)?;

    // Create a connection to the database
    let mut connection = connection::Builder::from_env()?.build(&mut database)?;

    // Construct a statement to execute a query
    let mut statement = connection.new_statement()?;

    // note: exposes a bug
    // "SELECT
    // i3 = i1 AS i3_eq_i1,
    // i3 != i1 AS i3_neq_i1,
    // i3 <> i1 AS i3_ltgt_i1,
    // i3 < i1 AS i3_lt_i1,
    // i3 > i1 AS i3_gt_i1,
    // i3 <= i1 AS i3_lte_i1,
    // i3 >= i1 AS i3_gte_i1,
    // i3 IS DISTINCT FROM i1 AS i3_distinct_i1,
    // i1 + i3 AS i1_plus_i3,
    // i1 - i3 AS i1_minus_i3,
    // i1 * i3 AS i1_mul_i3,
    // i1 % i3 AS i1_mod_i3,
    // FROM (SELECT 3::integer AS i3, 1::integer AS i1)"

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
