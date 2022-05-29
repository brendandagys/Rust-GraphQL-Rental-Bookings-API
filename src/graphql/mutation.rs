use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models::{
    booking::Booking,
    guest::{Gender, Guest},
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_booking(
        &self,
        ctx: &Context<'_>,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
        arrival_timestamp: Option<String>,
        adult_count: i16,
        child_count: i16,
        towels: bool,
    ) -> async_graphql::Result<Booking> {
        let pool = ctx.data::<PgPool>()?;

        let created_booking = Booking::create(
            &pool,
            start_timestamp,
            end_timestamp,
            arrival_timestamp,
            adult_count,
            child_count,
            towels,
        )
        .await?;

        Ok(created_booking)
    }

    async fn delete_booking(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Booking> {
        let pool = ctx.data::<PgPool>()?;
        let deleted_booking = Booking::delete(&pool, id).await?;

        Ok(deleted_booking)
    }

    async fn update_booking(
        &self,
        ctx: &Context<'_>,
        id: i32,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
        arrival_timestamp: Option<String>,
        adult_count: Option<i16>,
        child_count: Option<i16>,
        towels: Option<bool>,
    ) -> async_graphql::Result<Booking> {
        let pool = ctx.data::<PgPool>()?;

        let updated_booking = Booking::update(
            &pool,
            id,
            start_timestamp,
            end_timestamp,
            arrival_timestamp,
            adult_count,
            child_count,
            towels,
        )
        .await?;

        Ok(updated_booking)
    }

    async fn create_guest(
        &self,
        ctx: &Context<'_>,
        last_name: String,
        first_name: String,
        email: String,
        passport_number: Option<String>,
        country: Option<String>,
        gender: Option<Gender>,
        age: Option<i16>,
    ) -> async_graphql::Result<Guest> {
        let pool = ctx.data::<PgPool>()?;

        let created_guest = Guest::create(
            &pool,
            last_name,
            first_name,
            email,
            passport_number,
            country,
            gender,
            age,
        )
        .await?;

        Ok(created_guest)
    }

    async fn delete_guest(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Guest> {
        let pool = ctx.data::<PgPool>()?;
        let deleted_guest = Guest::delete(&pool, id).await?;

        Ok(deleted_guest)
    }

    async fn update_guest(
        &self,
        ctx: &Context<'_>,
        id: i32,
        last_name: Option<String>,
        first_name: Option<String>,
        email: Option<String>,
        passport_number: Option<String>,
        country: Option<String>,
        gender: Option<Gender>,
        age: Option<i16>,
    ) -> async_graphql::Result<Guest> {
        let pool = ctx.data::<PgPool>()?;

        let updated_guest = Guest::update(
            &pool,
            id,
            last_name,
            first_name,
            email,
            passport_number,
            country,
            gender,
            age,
        )
        .await?;

        Ok(updated_guest)
    }
}
