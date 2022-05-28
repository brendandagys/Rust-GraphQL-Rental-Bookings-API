use actix_web::{guard, web, App, HttpServer};
use std::env;

mod db;
mod graphql;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let host = env::var("HOST").expect("`HOST` is not set.");
    let port = env::var("PORT").expect("`PORT` is not set.");

    let schema = graphql::schema::get_graphql_schema().await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/health")
                    .guard(guard::Get())
                    .to(routes::health),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(routes::graphql_playground),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(routes::index),
            )
    });

    println!(
        "GraphQL server running at http://{}:{}/graphql",
        &host, &port
    );

    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
