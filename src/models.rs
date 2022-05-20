use async_graphql::SimpleObject;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Booking {
    pub id: i32,
    guest_last_name: String,
    guest_first_name: String,
    start_timestamp: Option<DateTime<Utc>>,
    end_timestamp: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Booking {
    pub async fn read_all(pool: &PgPool) -> async_graphql::Result<Vec<Booking>> {
        let rows = sqlx::query_as!(Booking, "SELECT * FROM bookings")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }
}
