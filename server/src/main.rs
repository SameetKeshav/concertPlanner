pub mod controllers;
pub mod dto;
pub mod models;
pub mod service;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Starting server...");
    println!("Connecting to MongoDB...");

    let uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    println!("{}", uri);

    let client = Client::with_uri_str(uri).await.expect("Failed to connect");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(controllers::user_controller::get_user)
            .service(controllers::user_controller::create_user)
            .service(controllers::user_controller::update_user)
            .service(controllers::user_controller::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
