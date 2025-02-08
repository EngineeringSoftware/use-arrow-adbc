#!/bin/bash

# This script is an entry point to illustrate ways to use ADBC
# drivers.

readonly CONFIG=$HOME/.snowsql/config

function _extract() {
        ### Extract value from snowsql config file.
        local fld="${1}"

        cat $CONFIG | grep '^'"${fld}"' =' | head -n 1 | sed 's/'"${fld}"' = \(.*\)/\1/g'
}

function _export_snowflake_envs() {
        export ADBC_SNOWFLAKE_SQL_DB="milos_test"

        export ADBC_SNOWFLAKE_SQL_ACCOUNT=$(_extract "accountname")
        export ADBC_SNOWFLAKE_USERNAME=$(_extract "username")
        export ADBC_SNOWFLAKE_PASSWORD=$(_extract "password")
        export ADBC_SNOWFLAKE_SQL_WAREHOUSE=$(_extract "warehousename")
        export ADBC_SNOWFLAKE_SQL_ROLE=$(_extract "rolename")
}

function _check_precondition() {
        ### Check preconditions.

        [ ! -f "$CONFIG" ] && \
                { echo "config file not available"; return 1; }

        return 0
}

function dummy() {
        ### Runs ADBC example with the dummy driver.
        export LD_LIBRARY_PATH=lib

        cargo run -- --example $FUNCNAME
}

function snowflake_dynamic() {
        ### Runs ADBC example with a snowflake (.so) driver.
        export LD_LIBRARY_PATH=lib

        _check_precondition || return 1

        _export_snowflake_envs
        cargo run -- --example $(echo $FUNCNAME | sed 's/_/-/g')
}

function snowflake_static() {
        ### Runs ADBC example with the Snowflake driver.

        _check_precondition || return 1

        _export_snowflake_envs
        cargo run -- --example $(echo $FUNCNAME | sed 's/_/-/g')
}

[ $# -ne 1 ] && \
        { echo "Required argument"; exit 1; }

"$@" || \
        { echo "Error"; exit 1; }
