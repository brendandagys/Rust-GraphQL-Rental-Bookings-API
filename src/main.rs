use actix_web::{App, HttpResponse, HttpServer, web};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:8000".to_owned());

    println!("HTTP Server running at http://{}", &server_address);

    HttpServer::new(
        move || App::new()
            .service(web::resource("/").to(|| HttpResponse::Ok()))
    )
    .bind(&server_address)?
    .run()
    .await?;
    Ok(())
}
