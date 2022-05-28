use super::mutation::Mutation;
use super::query::Query;
use crate::db;
use async_graphql::{EmptySubscription, Schema};

pub type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn get_graphql_schema() -> anyhow::Result<ServiceSchema> {
    let database_pool = db::db_connection().await?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(database_pool)
        .finish();

    Ok(schema)
}
