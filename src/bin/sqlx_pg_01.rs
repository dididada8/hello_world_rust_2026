use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // user: postgres
    // password: gfaSSBqJf3-
    // db: localhost:5432/postgres
    let database_url = "postgres://postgres:gfaSSBqJf3-@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let rows = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT table_schema, table_name
        FROM information_schema.tables
        WHERE table_type = 'BASE TABLE'
          AND table_schema NOT IN ('pg_catalog', 'information_schema')
        ORDER BY table_schema, table_name
        "#,
    )
    .fetch_all(&pool)
    .await?;

    if rows.is_empty() {
        println!("postgres 数据库当前没有用户表。");
    } else {
        println!("postgres 数据库中的表：");
        for (schema, table) in rows {
            println!("{}.{}", schema, table);
        }
    }

    Ok(())
}
