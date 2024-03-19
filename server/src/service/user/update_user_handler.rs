use std::str::FromStr;

use actix_web::{
    web::{self, Form},
    HttpResponse, Responder,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde_json::json;

use crate::{dto::user_dto::User, models::user_model::UserModel};

pub async fn update_user_handler(
    user_id: web::Path<String>,
    client: web::Data<Client>,
    data: Form<User>,
) -> impl Responder {
    let collection: Collection<UserModel> = client
        .database(std::env::var("MONGODB_DB").unwrap().as_str())
        .collection("users");
    let filter = doc! { "_id": ObjectId::from_str(user_id.as_str()).unwrap() };
    let update = doc! {
        "$set": doc!{
            "username": data.username.clone(),
            "email": data.email.clone(),
            "name": data.name.clone(),
            "last_name": data.last_name.clone(),
            "address": data.address.clone()
        }
    };

    let result = collection.update_one(filter, update, None).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "User updated"})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"}))
        }
    }
}
