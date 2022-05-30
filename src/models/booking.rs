use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::guest::{Gender, Guest};

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Booking {
    pub id: i32,
    pub start_timestamp: Option<DateTime<Utc>>,
    pub end_timestamp: Option<DateTime<Utc>>,
    pub arrival_timestamp: Option<DateTime<Utc>>,
    pub adult_count: i16,
    pub child_count: i16,
    pub towels: bool,

    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Booking {
    pub async fn guests(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Guest>> {
        let pool = ctx.data::<PgPool>()?;

        let guests = sqlx::query_as!(
            Guest,
            r#"SELECT
                 id,
                 last_name,
                 first_name,
                 email,
                 passport_number,
                 country,
                 gender as "gender: Gender",
                 age,
                 created_at,
                 modified_at
               FROM guests
               WHERE id IN (
                   SELECT guest_id
                   FROM   booking_has_guests
                   WHERE  booking_id = $1 
               )
            "#,
            &self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(guests)
    }
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
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
        arrival_timestamp: Option<String>,
        adult_count: i16,
        child_count: i16,
        towels: bool,
    ) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(
            Booking,
            "INSERT INTO bookings(
                start_timestamp,
                end_timestamp,
                arrival_timestamp,
                adult_count,
                child_count,
                towels
             )
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
            match start_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match end_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match arrival_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            adult_count,
            child_count,
            towels
        )
        .fetch_one(pool)
        .await?;

        Ok(booking)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
        arrival_timestamp: Option<String>,
        adult_count: Option<i16>,
        child_count: Option<i16>,
        towels: Option<bool>,
    ) -> async_graphql::Result<Booking> {
        let booking = sqlx::query_as!(
            Booking,
            "UPDATE bookings
             SET start_timestamp   = COALESCE($1, start_timestamp),
                 end_timestamp     = COALESCE($2, end_timestamp),
                 arrival_timestamp = COALESCE($3, arrival_timestamp),
                 adult_count       = COALESCE($4, adult_count),
                 child_count       = COALESCE($5, child_count),
                 towels            = COALESCE($6, towels)
             WHERE id = $7
             RETURNING *",
            match start_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match end_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            match arrival_timestamp {
                Some(i) => Some(DateTime::parse_from_str(&i, "%Y-%m-%d %H:%M:%S %z").unwrap()),
                None => None,
            },
            adult_count,
            child_count,
            towels,
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
