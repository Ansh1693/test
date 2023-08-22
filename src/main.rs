use std::error::Error;
use sqlx::Connection;
use sqlx::Row;


#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let url = "postgres://postgres:12345678@localhost:8080/demo";

    let  pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("Select 1+1 as result")
        .fetch_one(&pool)
        .await?;

    let sum : i32 = res.get("result");
    println!("sum = {}", sum);

    Ok(())
}
