use std::process::exit;

use anyhow::Error
;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    if let Err(e) = do_main().await {
        eprintln!("error: {}", e);
        exit(1)
    }
}

async fn do_main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "postgres://user1:password@localhost:28816/repro_test_fdw?sslmode=disable",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let prepared = client.prepare(r#"insert into "Customer" ("CustAccountNo") values($1);"#).await?;

    for i in 0..100 {
        let account_number = format!("C0X{:04}", i + 1);
        println!("creating customer {}", account_number);
        client
            .execute(
                &prepared,
                &[&account_number],
            )
            .await?;
    }

    Ok(())
}
