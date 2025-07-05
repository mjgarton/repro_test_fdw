package main

import (
	"context"
	"fmt"
	"os"

	"github.com/jackc/pgx/v5"
)

func main() {
	if err := realMain(); err != nil {
		fmt.Printf("%v\n", err)
		os.Exit(-1)
	}
}

func realMain() error {
	ctx := context.Background()
	conn, err := pgx.Connect(ctx, "postgres://user1:password@localhost:28816/repro_test_fdw?sslmode=disable")
	if err != nil {
		return err
	}

	for i := 0; i < 15; i++ {
		accountNumber := fmt.Sprintf("C0X%04d", i+1)
		fmt.Printf("inserting customer %v\n", accountNumber)
		_, err := conn.Exec(ctx, `insert into customer (account_number) values($1);`, accountNumber)
		if err != nil {
			return fmt.Errorf("error inserting Customer row : %s", err)
		}
		//		time.Sleep(50 * time.Millisecond)
	}

	return nil
}
