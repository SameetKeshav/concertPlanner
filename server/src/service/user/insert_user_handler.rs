use crate::dto::user_dto::User;
use crate::models::user_model::UserModel;
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use mongodb::{bson::oid::ObjectId, Client, Collection};
use serde_json::json;

pub async fn insert_user_handler(
    client: web::Data<Client>,
    data: web::Form<User>,
) -> impl Responder {
    let collection: Collection<UserModel> = client
        .database(std::env::var("MONGODB_DB").unwrap().as_str())
        .collection("users");

    let document = UserModel {
        _id: ObjectId::new(),
        username: data.username.clone(),
        email: data.email.clone(),
        name: data.name.clone(),
        last_name: data.last_name.clone(),
        address: data.address.clone(),
    };

    let insert_result = collection.insert_one(document, None).await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "User inserted"})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"}))
        }
    }
}
