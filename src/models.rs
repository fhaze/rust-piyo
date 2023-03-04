use crate::schema::{messages, users};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Selectable, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserWithMessages {
    #[serde(flatten)]
    pub user: User,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    pub msg: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = messages, belongs_to(User))]
pub struct NewMessage {
    pub msg: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMessage {
    pub message: String,
}
