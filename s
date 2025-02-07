#!/bin/bash

function _extract() {
        local fld="${1}"
        cat $HOME/.snowsql/config | grep '^'"${fld}"' =' | head -n 1 | sed 's/'"${fld}"' = \(.*\)/\1/g'
}

function dummy() {
        export LD_LIBRARY_PATH=lib
        cargo run
}

function snowflake() {
        export ADBC_SNOWFLAKE_SQL_ACCOUNT=$(_extract "accountname")
        export ADBC_SNOWFLAKE_USERNAME=$(_extract "username")
        export ADBC_SNOWFLAKE_PASSWORD=$(_extract "password")
        export ADBC_SNOWFLAKE_SQL_DB="milos_test"
        export ADBC_SNOWFLAKE_SQL_WAREHOUSE=$(_extract "warehousename")
        export ADBC_SNOWFLAKE_SQL_ROLE=$(_extract "rolename")

        RUST_BACKTRACE=1 cargo run
}

"$@"
