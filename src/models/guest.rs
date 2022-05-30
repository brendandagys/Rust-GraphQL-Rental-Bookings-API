use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::models::booking::Booking;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    M,
    F,
    O,
}

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Guest {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub passport_number: Option<String>,
    pub country: Option<String>,
    pub gender: Option<Gender>,
    pub age: Option<i16>,

    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Guest {
    pub async fn bookings(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Booking>> {
        let pool = ctx.data::<PgPool>()?;

        let bookings = sqlx::query_as!(
            Booking,
            r#"SELECT *
               FROM bookings
               WHERE id IN (
                   SELECT booking_id
                   FROM   booking_has_guests
                   WHERE  guest_id = $1
               )
            "#,
            &self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(bookings)
    }
}

impl Guest {
    pub async fn read_all(pool: &PgPool) -> async_graphql::Result<Vec<Guest>> {
        let guests = sqlx::query_as!(
            Guest,
            r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    email,
                    passport_number,
                    country,
                    gender AS "gender: Gender",
                    age,
                    created_at,
                    modified_at
                FROM guests"#
        )
        .fetch_all(pool)
        .await?;

        Ok(guests)
    }

    pub async fn read_one(pool: &PgPool, id: i32) -> async_graphql::Result<Guest> {
        let guest = sqlx::query_as!(
            Guest,
            r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    email,
                    passport_number,
                    country,
                    gender AS "gender: Gender",
                    age,
                    created_at,
                    modified_at
                FROM guests WHERE id = $1"#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(guest)
    }

    pub async fn create(
        pool: &PgPool,
        last_name: String,
        first_name: String,
        email: String,
        passport_number: Option<String>,
        country: Option<String>,
        gender: Option<Gender>,
        age: Option<i16>,
    ) -> async_graphql::Result<Guest> {
        let guest = sqlx::query_as!(
            Guest,
            r#"INSERT INTO guests(
                last_name,
                first_name,
                email,
                passport_number,
                country,
                gender,
                age
             )
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING
                id,
                last_name,
                first_name,
                email,
                passport_number,
                country,
                gender AS "gender: Gender",
                age,
                created_at,
                modified_at"#,
            last_name,
            first_name,
            email,
            passport_number,
            country,
            gender as Option<Gender>,
            age
        )
        .fetch_one(pool)
        .await?;

        Ok(guest)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        last_name: Option<String>,
        first_name: Option<String>,
        email: Option<String>,
        passport_number: Option<String>,
        country: Option<String>,
        gender: Option<Gender>,
        age: Option<i16>,
    ) -> async_graphql::Result<Guest> {
        let guest = sqlx::query_as!(
            Guest,
            r#"UPDATE guests
             SET last_name       = COALESCE($1, last_name),
                 first_name      = COALESCE($2, first_name),
                 email           = COALESCE($3, email),
                 passport_number = COALESCE($4, passport_number),
                 country         = COALESCE($5, country),
                 gender          = COALESCE($6, gender),
                 age             = COALESCE($7, age)
             WHERE id = $8
             RETURNING
                id,
                last_name,
                first_name,
                email,
                passport_number,
                country,
                gender AS "gender: Gender",
                age,
                created_at,
                modified_at"#,
            last_name,
            first_name,
            email,
            passport_number,
            country,
            gender as Option<Gender>,
            age,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(guest)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> async_graphql::Result<Guest> {
        let guest = sqlx::query_as!(
            Guest,
            r#"DELETE FROM guests
             WHERE id = $1
             RETURNING
                id,
                last_name,
                first_name,
                email,
                passport_number,
                country,
                gender AS "gender: Gender",
                age,
                created_at,
                modified_at"#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(guest)
    }
}
