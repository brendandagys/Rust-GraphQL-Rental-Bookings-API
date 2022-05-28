use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_booking(
        &self,
        ctx: &Context<'_>,
        guest_last_name: String,
        guest_first_name: String,
        start_timestamp: String,
        end_timestamp: String,
    ) -> async_graphql::Result<models::Booking> {
        let pool = ctx.data::<PgPool>().unwrap();

        let created_booking = models::Booking::create(
            &pool,
            &guest_last_name,
            &guest_first_name,
            Some(&start_timestamp),
            Some(&end_timestamp),
        )
        .await?;

        Ok(created_booking)
    }

    async fn delete_booking(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<models::Booking> {
        let pool = ctx.data::<PgPool>().unwrap();
        let deleted_booking = models::Booking::delete(&pool, id).await?;

        Ok(deleted_booking)
    }

    async fn update_booking(
        &self,
        ctx: &Context<'_>,
        id: i32,
        guest_last_name: Option<String>,
        guest_first_name: Option<String>,
        start_timestamp: Option<String>,
        end_timestamp: Option<String>,
    ) -> async_graphql::Result<models::Booking> {
        let pool = ctx.data::<PgPool>().unwrap();

        let updated_booking = models::Booking::update(
            &pool,
            id,
            guest_last_name,
            guest_first_name,
            start_timestamp,
            end_timestamp,
        )
        .await?;

        Ok(updated_booking)
    }
}
