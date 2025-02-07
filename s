#!/bin/bash

# This script is an entry point to illustrate ways to use ADBC
# drivers.

readonly CONFIG=$HOME/.snowsql/config

function _extract() {
        ### Extract value from snowsql config file.
        local fld="${1}"

        cat $CONFIG | grep '^'"${fld}"' =' | head -n 1 | sed 's/'"${fld}"' = \(.*\)/\1/g'
}

function dummy() {
        ### Runs ADBC example with a dummy driver.
        export LD_LIBRARY_PATH=lib
        cargo run -- --example $FUNCNAME
}

function snowflake() {
        ### Runs ADBC example with the Snowflake driver.

        # Check preconditions.
        [ ! -f "$CONFIG" ] && \
                { echo "config file not available"; return 1; }

        export ADBC_SNOWFLAKE_SQL_DB="milos_test"

        export ADBC_SNOWFLAKE_SQL_ACCOUNT=$(_extract "accountname")
        export ADBC_SNOWFLAKE_USERNAME=$(_extract "username")
        export ADBC_SNOWFLAKE_PASSWORD=$(_extract "password")
        export ADBC_SNOWFLAKE_SQL_WAREHOUSE=$(_extract "warehousename")
        export ADBC_SNOWFLAKE_SQL_ROLE=$(_extract "rolename")

        cargo run -- --example $FUNCNAME
}

"$@"
