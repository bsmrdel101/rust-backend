use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;


pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
}
