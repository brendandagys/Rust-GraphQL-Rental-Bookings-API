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
        let bookings = sqlx::query_as!(Booking, "SELECT * FROM bookings")
            .fetch_all(pool)
            .await?;

        Ok(bookings)
    }

    pub async fn read_one(pool: &PgPool, id: i32) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(Booking, "SELECT * FROM bookings WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(booking)
    }

    pub async fn create(
        pool: &PgPool,
        guest_last_name: &str,
        guest_first_name: &str,
        start_timestamp: Option<&str>,
        end_timestamp: Option<&str>,
    ) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(
            Booking,
            "INSERT INTO bookings(
                guest_last_name,
                guest_first_name,
                start_timestamp,
                end_timestamp
             )
             VALUES ($1,$2,$3,$4)
             RETURNING *",
            guest_last_name,
            guest_first_name,
            match start_timestamp {
                Some(i) => Some(DateTime::parse_from_str(i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match end_timestamp {
                Some(i) => Some(DateTime::parse_from_str(i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            }
        )
        .fetch_one(pool)
        .await?;

        Ok(booking)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        guest_last_name: Option<String>,
        guest_first_name: Option<String>,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
    ) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(
            Booking,
            "UPDATE bookings
             SET guest_last_name = COALESCE($1, guest_last_name),
                 guest_first_name = COALESCE($2, guest_first_name),
                 start_timestamp = COALESCE($3, start_timestamp),
                 end_timestamp = COALESCE($4, end_timestamp)
             WHERE id = $5
             RETURNING *",
            guest_last_name,
            guest_first_name,
            match start_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match end_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(booking)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(
            Booking,
            "DELETE FROM bookings
             WHERE id = $1
             RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(booking)
    }
}
