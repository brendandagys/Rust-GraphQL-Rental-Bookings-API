use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models;

pub struct Query;

#[Object]
impl Query {
    async fn bookings<'a>(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<models::Booking>> {
        let pool = ctx.data::<PgPool>()?;
        let rows = models::Booking::read_all(&pool).await?;

        Ok(rows)
    }
}
