use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde_json::json;

use crate::models::user_model::UserModel;

pub async fn delete_user_handler(
    user_id: web::Path<String>,
    client: web::Data<Client>,
) -> impl Responder {
    let collection: Collection<UserModel> = client
        .database(std::env::var("MONGODB_DB").unwrap().as_str())
        .collection("users");
    let document = doc! {
        "_id": ObjectId::from_str(user_id.as_str()).unwrap()
    };

    let result = collection.delete_one(document, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "User deleted"})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"}))
        }
    }
}
