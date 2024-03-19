pub mod dto;
pub mod models;
pub mod service;

use crate::service::user::get_user_handler::get_user_handler;

use actix_web::{
    get,
    web::{self},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use mongodb::Client;

#[get("/user/{userId}")]
async fn get_user(user_id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    get_user_handler(user_id, client).await
}

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
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
