use adbc_core::{Connection, Statement};
use adbc_snowflake::{connection, database, Driver};
use arrow_array::{cast::AsArray, types::Decimal128Type};

use libc::{sigaltstack, stack_t, SS_DISABLE};
use std::mem::MaybeUninit;

use tokio::task;

// use std::error::Error;
// use std::sync::Arc;
// use tokio;

fn stack() {
    unsafe {
        let mut current_stack = MaybeUninit::<stack_t>::uninit();
        let ret = sigaltstack(std::ptr::null(), current_stack.as_mut_ptr());

        if ret != 0 {
            eprintln!("sigaltstack call failed");
        } else {
            let current_stack = current_stack.assume_init();
            if (current_stack.ss_flags & SS_DISABLE) != 0 {
                println!("sigaltstack is DISABLED");
            } else {
                println!(
                    "sigaltstack is ENABLED: sp={:?}, size={}",
                    current_stack.ss_sp, current_stack.ss_size
                );
            }
        }
    }
}

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut driver = Driver::try_load_dynamic()?;

//     stack();

//     // Construct a database using environment variables
//     let mut database = database::Builder::from_env()?.build(&mut driver)?;

//     // Create a connection to the database
//     let mut connection = connection::Builder::from_env()?.build(&mut database)?;

//     // Construct a statement to execute a query
//     let mut statement = connection.new_statement()?;

//     // Execute a query
//     statement.set_sql_query("SELECT 21 + 21")?;
//     let mut reader = statement.execute()?;

//     // Check the result
//     let batch = reader.next().expect("a record batch")?;
//     assert_eq!(
//         batch.column(0).as_primitive::<Decimal128Type>().value(0),
//         42
//     );

//     stack();
//     Ok(())
// }

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     // Construct a database using environment variables
    let mut driver = Driver::try_load_dynamic()?;
    let database = database::Builder::from_env()?.build(&mut driver)?;

    // Spawn 5 tasks
    let handles: Vec<_> = (0..100)
        .map(|i| {
            task::spawn({
                let mut value = database.clone();
                async move {
                    println!("Task {} started", i);
                    let mut connection = connection::Builder::from_env()
                        .unwrap()
                        .build(&mut value)
                        .unwrap();
                    let mut statement = connection.new_statement().unwrap();
                    statement.set_sql_query("SELECT 21 + 21").unwrap();
                    let mut reader = statement.execute().unwrap();
                    //tokio::time::sleep(Duration::from_secs(1)).await;
                    //println!("Task {} done", i);
                    //i * 2 // return value

                    let batch = reader.next().expect("a record batch").unwrap();
                    assert_eq!(
                        batch.column(0).as_primitive::<Decimal128Type>().value(0),
                        42
                    );

                    stack();
                    ()
                }
            })
        })
        .collect();

    // Wait for all tasks to complete
    for handle in handles {
        match handle.await {
            Ok(_result) => {} // println!("Got result: {:?}", result)},
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }

    println!("All tasks completed.");

    Ok(())
}
