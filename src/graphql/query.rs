use async_graphql::{Context, Object};
use sqlx::PgPool;

use crate::models::{booking::Booking, guest::Guest};

pub struct Query;

#[Object]
impl Query {
    async fn bookings<'a>(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Booking>> {
        let pool = ctx.data::<PgPool>()?;
        let bookings = Booking::read_all(&pool).await?;

        Ok(bookings)
    }

    async fn booking<'a>(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Booking> {
        let pool = ctx.data::<PgPool>()?;
        let booking = Booking::read_one(&pool, id).await?;

        Ok(booking)
    }

    async fn guests<'a>(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Guest>> {
        let pool = ctx.data::<PgPool>()?;
        let guests = Guest::read_all(&pool).await?;

        Ok(guests)
    }

    async fn guest<'a>(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Guest> {
        let pool = ctx.data::<PgPool>()?;
        let guest = Guest::read_one(&pool, id).await?;

        Ok(guest)
    }
}
