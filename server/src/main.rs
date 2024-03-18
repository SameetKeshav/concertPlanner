pub mod dto;
use std::str::FromStr;

use crate::dto::user_dto::User;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

#[get("/user/{userId}")]
async fn get_user(user_id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client
        .database(std::env::var("MONGODB_DB").unwrap().as_str())
        .collection("users");
    match collection
        .find_one(
            doc! { "_id": ObjectId::from_str(user_id.as_str()).unwrap() },
            None,
        )
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User} not found"),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
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
