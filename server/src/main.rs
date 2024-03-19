pub mod dto;
pub mod models;
pub mod service;

use crate::dto::user_dto::User;
use crate::service::user::get_user_handler::get_user_handler;
use crate::service::user::insert_user_handler::insert_user_handler;

use actix_web::{get, post, web, App, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::Client;

#[post("/insert-user")]
async fn create_user(client: web::Data<Client>, data: web::Form<User>) -> impl Responder {
    insert_user_handler(client, data).await
}

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
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
