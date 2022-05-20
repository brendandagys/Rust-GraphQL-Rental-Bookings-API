use sqlx::{postgres::PgPool, Error};
use std::env;

pub async fn db_connection() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` is not set.");
    let db_pool = PgPool::connect(&database_url).await?;

    Ok(db_pool)
}
