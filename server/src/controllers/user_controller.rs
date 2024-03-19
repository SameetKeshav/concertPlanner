use crate::dto::user_dto::User;
use crate::service::user::delete_user_handler::delete_user_handler;
use crate::service::user::get_user_handler::get_user_handler;
use crate::service::user::insert_user_handler::insert_user_handler;
use crate::service::user::update_user_handler::update_user_handler;

use actix_web::{delete, get, patch, post, web, Responder};
use mongodb::Client;

#[patch("/update-user/{userId}")]
async fn update_user(
    user_id: web::Path<String>,
    client: web::Data<Client>,
    data: web::Form<User>,
) -> impl Responder {
    update_user_handler(user_id, client, data).await
}

#[post("/insert-user")]
async fn create_user(client: web::Data<Client>, data: web::Form<User>) -> impl Responder {
    insert_user_handler(client, data).await
}

#[get("/user/{userId}")]
async fn get_user(user_id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    get_user_handler(user_id, client).await
}

#[delete("/delete-user/{userId}")]
async fn delete_user(user_id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    delete_user_handler(user_id, client).await
}
