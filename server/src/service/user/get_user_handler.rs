use crate::dto::user_dto::User;
use crate::models::user_model::UserModel;
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde_json::json;
use std::str::FromStr;

pub async fn get_user_handler(
    user_id: web::Path<String>,
    client: web::Data<Client>,
) -> impl Responder {
    let collection: Collection<UserModel> = client
        .database(std::env::var("MONGODB_DB").unwrap().as_str())
        .collection("users");
    match collection
        .find_one(
            doc! { "_id": ObjectId::from_str(user_id.as_str()).unwrap() },
            None,
        )
        .await
    {
        Ok(Some(user)) => {
            let user = User {
                user_id: user._id.to_hex(),
                username: user.username,
                email: user.email,
                name: user.name,
                last_name: user.last_name,
                address: user.address,
            };
            HttpResponse::Ok().json(user)
        }
        Ok(None) => HttpResponse::NotFound().json(json!({"message": "User not found"})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"}))
        }
    }
}
