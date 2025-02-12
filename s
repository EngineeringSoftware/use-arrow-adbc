#!/bin/bash

# This script is an entry point to illustrate ways to use ADBC
# drivers.

readonly DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

readonly CONFIG=$HOME/.snowsql/config

function _snow_extract() {
        ### Extract value from snowsql config file.
        local fld="${1}"

        cat $CONFIG | grep '^'"${fld}"' =' | head -n 1 | sed 's/'"${fld}"' = \(.*\)/\1/g'
}

function _snow_export_envs() {
        # The following is not required.
        #export ADBC_SNOWFLAKE_SQL_DB="milos_test"

        export ADBC_SNOWFLAKE_SQL_ACCOUNT=$(_snow_extract "accountname")
        export ADBC_SNOWFLAKE_USERNAME=$(_snow_extract "username")
        export ADBC_SNOWFLAKE_PASSWORD=$(_snow_extract "password")
        export ADBC_SNOWFLAKE_SQL_WAREHOUSE=$(_snow_extract "warehousename")
        export ADBC_SNOWFLAKE_SQL_ROLE=$(_snow_extract "rolename")

        return 0
}

function _snow_check_precond() {
        ### Check preconditions.

        [ ! -f "$CONFIG" ] && \
                { echo "config file not available"; return 1; }

        hash go &> /dev/null ||
                { echo "missing go for building .so"; return 1; }

        return 0
}

function _snow_build_so() {
        if [ ! -f "lib/libadbc_driver_snowflake.so" ]; then
                rm -rf arrow-adbc
                git clone https://github.com/apache/arrow-adbc || \
                        { echo "Error: could not clone"; return 1; }

                ( cd arrow-adbc/go/adbc/pkg/snowflake
                  go build \
                     -tags driverlib \
                     -buildmode=c-shared \
                     -o "${DIR}/lib/libadbc_driver_snowflake.so" .
                ) || \
                        { echo "Error: could not build .so"; return 1; }

                [ ! -f "lib/libadbc_driver_snowflake.so" ] && \
                        { echo "Error: no .so file"; return 1; }
        fi
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

        _snow_check_precond || return 1
        _snow_build_so || return 1
        _snow_export_envs || return 1

        cargo build
        ./target/debug/use-arrow-adbc --example $(echo $FUNCNAME | sed 's/_/-/g')
}

function snowflake_static() {
        ### Runs ADBC example with the Snowflake driver.

        _snow_check_precond || return 1
        _snow_export_envs || return 1

        cargo run -- --example $(echo $FUNCNAME | sed 's/_/-/g')
}

[ $# -ne 1 ] && \
        { echo "Error: Required argument (example name) missing"; exit 1; }

"$@" || \
        { echo "Error"; exit 1; }
