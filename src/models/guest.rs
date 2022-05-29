use async_graphql::{Enum, SimpleObject};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    M,
    F,
    O,
}

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Guest {
    id: i32,
    last_name: String,
    first_name: String,
    email: String,
    passport_number: Option<String>,
    country: Option<String>,
    gender: Option<Gender>,
    age: Option<i16>,

    created_at: DateTime<Utc>,
    modified_at: Option<DateTime<Utc>>,
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
