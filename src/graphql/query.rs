use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models;

pub struct Query;

#[Object]
impl Query {
    async fn bookings<'a>(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<models::Booking>> {
        let pool = ctx.data::<PgPool>()?;
        let bookings = models::Booking::read_all(&pool).await?;

        Ok(bookings)
    }

    async fn booking<'a>(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<models::Booking> {
        let pool = ctx.data::<PgPool>()?;
        let booking = models::Booking::read_one(&pool, id).await?;

        Ok(booking)
    }
}
