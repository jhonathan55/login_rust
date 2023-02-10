use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestById {
    pub id: mongodb::bson::oid::ObjectId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestByEmail {
    pub email: String,
}
