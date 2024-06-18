use sqlx::{
    postgres::{PgArguments, PgPoolOptions},
    Arguments,
};

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    age: Option<i16>,
    email: String,
}

pub async fn example() -> Result<(), sqlx::Error> {
    let url = "postgres://john:password@localhost:5432/example";
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
    println!("{pool:?}");

    let row = sqlx::query_as!(User, "SELECT id, name, age, email FROM users;")
        .fetch_all(&pool)
        .await?;

    // use parameter binding
    let mut args = PgArguments::default();
    args.add(1);
    let _ = sqlx::query_with("SELECT * FROM users WHERE id = $1", args)
        .fetch_one(&pool)
        .await?;
    println!("rows: {:?}", row);

    println!("done");

    let users = (0..).map(|i| User {
        id: i,
        name: format!("user_{i}"),
        age: Some(20),
        email: format!("user_{i}@example.com"),
    });

    // extract the first column of each row
    let over_20: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM users WHERE age > $1;")
        .bind(20)
        .fetch_one(&pool)
        .await?;

    Ok(())
}
