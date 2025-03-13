package main

import (
        "context"
        "fmt"
        driver "github.com/apache/arrow-adbc/go/adbc/driver/bigquery"
        "github.com/apache/arrow-go/v18/arrow/memory"
)

const (
        PROJECT_ID = ""
        DATASET_ID = "abc"
)

// arrow-adbc/go/adbc/driver/bigquery/driver_test.go
func main() {
	ctx := context.Background()
	mem := memory.NewCheckedAllocator(memory.DefaultAllocator)
	t := driver.NewDriver(mem)

        options := map[string]string{
		driver.OptionStringProjectID: PROJECT_ID,
		driver.OptionStringDatasetID: DATASET_ID,
	}

        db, err := t.NewDatabase(options)
	if err != nil {
                panic(err)
        }
	defer db.Close()

        cnxn, err := db.Open(ctx)
	if err != nil {
		panic(err)
	}
	defer cnxn.Close()

        stmt, err := cnxn.NewStatement()
	if err != nil {
		panic(err)
	}
	defer stmt.Close()

        err = stmt.SetOption(driver.OptionBoolQueryUseLegacySQL, "false")
	if err != nil {
		panic(err)
	}

        query := "drop schema abc cascade"
	err = stmt.SetSqlQuery(query)
	if err != nil {
		panic(err)
	}
	rdr, _, err := stmt.ExecuteQuery(ctx)
	if err != nil {
                panic(err)
	}
	defer rdr.Release()

        fmt.Println(rdr.Schema().NumFields())
        fmt.Printf("done\n")
}
