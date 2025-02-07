use adbc_core::driver_manager::ManagedDriver;
use adbc_core::options::{AdbcVersion, OptionDatabase, OptionStatement};
use adbc_core::Statement;
use adbc_core::{Connection, Database, Driver, Optionable};

const OPTION_STRING_LONG: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const OPTION_BYTES_LONG: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn main() {
    let mut driver = ManagedDriver::load_dynamic_from_name(
        "adbc_dummy",
        Some(b"DummyDriverInit"),
        AdbcVersion::V110,
    )
    .unwrap();

    let mut database = driver.new_database().unwrap();
    let mut connection = database.new_connection().unwrap();
    let mut statement = connection.new_statement().unwrap();

    statement
        .set_option(OptionStatement::Incremental, "true".into())
        .unwrap();
    let value = statement
        .get_option_string(OptionStatement::Incremental)
        .unwrap();
    assert_eq!(value, "true");

    // Pre-init options.
    let options = [
        (OptionDatabase::Username, "Alice".into()),
        (OptionDatabase::Password, 42.into()),
        (OptionDatabase::Uri, std::f64::consts::PI.into()),
        (OptionDatabase::Other("pre.bytes".into()), b"Hello".into()),
        (
            OptionDatabase::Other("pre.string.long".into()),
            OPTION_STRING_LONG.into(),
        ),
        (
            OptionDatabase::Other("pre.bytes.long".into()),
            OPTION_BYTES_LONG.into(),
        ),
    ];

    let database = driver.new_database_with_opts(options).unwrap();

    let value = database
        .get_option_string(OptionDatabase::Username)
        .unwrap();
    assert_eq!(value, "Alice");

    let _ = statement.execute().unwrap();
}
