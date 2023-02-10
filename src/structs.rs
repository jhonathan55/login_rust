use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status:Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub date_at: Option<Date>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub date_up: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestById {
    
    pub id: mongodb::bson::oid::ObjectId,
}