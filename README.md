### Use ADBC

Exploring the use of ADBC (via Rust)
https://github.com/apache/arrow-adbc

The following command runs an example with Rust ADBC static driver.

```
./s snowflake_static
```

The following command runs an example with Rust ADBC dynamic driver.

```
./s snowflake_dynamic
```

The following command will run a simple ADBC example that uses a dummy
driver.

```
./s dummy
```

All the commands are self contained, i.e., they will download/build
necessary artifacts to run examples end-to-end.
