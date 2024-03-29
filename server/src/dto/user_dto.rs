use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
// pub struct User {
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub address: String,
}
