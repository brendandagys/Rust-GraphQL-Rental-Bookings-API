use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::graphql;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("GraphQL server is healthy!")
}

pub async fn index(
    schema: web::Data<graphql::schema::ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> actix_web::Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
