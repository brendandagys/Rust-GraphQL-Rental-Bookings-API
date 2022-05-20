use super::query::Query;
use crate::db;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn get_graphql_schema() -> anyhow::Result<ServiceSchema> {
    let database_pool = db::db_connection().await?;

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(database_pool)
        .finish();

    Ok(schema)
}
